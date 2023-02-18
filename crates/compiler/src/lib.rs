/*!
This crate provides functionality for compiling [Sass](https://sass-lang.com/) to CSS.

This crate targets compatibility with the reference implementation in Dart. If
upgrading from the [now deprecated](https://sass-lang.com/blog/libsass-is-deprecated)
`libsass`, one may have to modify their stylesheets. These changes will not differ
from those necessary to upgrade to `dart-sass`, and in general such changes should
be quite rare.

This crate is capable of compiling Bootstrap 4 and 5, bulma and bulma-scss, Bourbon,
as well as most other large Sass libraries with complete accuracy. For the vast
majority of use cases there should be no perceptible differences from the reference
implementation.

## Use as library
```
# use grass_compiler as grass;
fn main() -> Result<(), Box<grass::Error>> {
    let css = grass::from_string(
        "a { b { color: &; } }".to_owned(),
        &grass::Options::default().style(grass::OutputStyle::Compressed)
    )?;
    assert_eq!(css, "a b{color:a b}");
    Ok(())
}
```

## Use as binary
```bash
cargo install grass
grass input.scss
```
*/

#![warn(clippy::all, clippy::cargo, clippy::dbg_macro)]
#![deny(missing_debug_implementations)]
#![allow(
    clippy::use_self,
    clippy::missing_docs_in_private_items,
    clippy::unreachable,
    clippy::module_name_repetitions,
    // filter isn't fallible
    clippy::manual_filter_map,
    clippy::new_ret_no_self,
    renamed_and_removed_lints,
    clippy::unknown_clippy_lints,
    clippy::single_match,
    clippy::unimplemented,
    clippy::option_if_let_else,
    clippy::branches_sharing_code,
    clippy::derive_partial_eq_without_eq,

    // temporarily allowed while under heavy development.
    // eventually these allows should be refactored away
    // to no longer be necessary
    clippy::too_many_lines,
    clippy::cast_possible_truncation,
    clippy::single_match_else,
    clippy::redundant_pub_crate,
    // the api is changing too often to allot this
    clippy::missing_errors_doc,
    clippy::missing_const_for_fn,
    clippy::multiple_crate_versions,

    clippy::wrong_self_convention,
    clippy::items_after_statements,
    // this is only available on nightly
    clippy::unnested_or_patterns,
    clippy::uninlined_format_args,

    // todo:
    clippy::cast_sign_loss,
    clippy::cast_lossless,
    clippy::cast_precision_loss,
    clippy::float_cmp,
    clippy::wildcard_imports,
    clippy::comparison_chain,
    clippy::bool_to_int_with_if,

    unknown_lints,
)]

use std::path::Path;

use parse::{CssParser, SassParser, StylesheetParser};
use serializer::Serializer;
#[cfg(feature = "wasm-exports")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "wasm-exports")]
pub use crate::fs::JsFS;
use js_sys::Array as JSArray;

use codemap::CodeMap;

pub use crate::error::{
    PublicSassErrorKind as ErrorKind, SassError as Error, SassResult as Result,
};
pub use crate::fs::{Fs, NullFs, StdFs};
pub use crate::options::{InputSyntax, Options, OutputStyle};
pub(crate) use crate::{context_flags::ContextFlags, lexer::Token};
use crate::{evaluate::Visitor, lexer::Lexer, parse::ScssParser};

mod ast;
mod builtin;
mod color;
mod common;
mod context_flags;
mod error;
mod evaluate;
mod fs;
mod interner;
mod lexer;
mod options;
mod parse;
mod selector;
mod serializer;
mod unit;
mod utils;
mod value;

fn raw_to_parse_error(map: &CodeMap, err: Error, unicode: bool) -> Box<Error> {
    let (message, span) = err.raw();
    Box::new(Error::from_loc(message, map.look_up_span(span), unicode))
}

fn from_string_with_file_name<P: AsRef<Path>>(
    input: String,
    file_name: P,
    options: &Options,
) -> Result<String> {
    let mut map = CodeMap::new();
    let path = file_name.as_ref();
    let file = map.add_file(path.to_string_lossy().into_owned(), input);
    let empty_span = file.span.subspan(0, 0);
    let lexer = Lexer::new_from_file(&file);

    let input_syntax = options
        .input_syntax
        .unwrap_or_else(|| InputSyntax::for_path(path));

    let stylesheet = match input_syntax {
        InputSyntax::Scss => {
            ScssParser::new(lexer, &mut map, options, empty_span, file_name.as_ref()).__parse()
        }
        InputSyntax::Sass => {
            SassParser::new(lexer, &mut map, options, empty_span, file_name.as_ref()).__parse()
        }
        InputSyntax::Css => {
            CssParser::new(lexer, &mut map, options, empty_span, file_name.as_ref()).__parse()
        }
    };

    let stylesheet = match stylesheet {
        Ok(v) => v,
        Err(e) => return Err(raw_to_parse_error(&map, *e, options.unicode_error_messages)),
    };

    let mut visitor = Visitor::new(path, options, &mut map, empty_span);
    match visitor.visit_stylesheet(stylesheet) {
        Ok(_) => {}
        Err(e) => return Err(raw_to_parse_error(&map, *e, options.unicode_error_messages)),
    }
    let stmts = visitor.finish();

    let mut serializer = Serializer::new(options, &map, false, empty_span);

    let mut prev_was_group_end = false;
    let mut prev_requires_semicolon = false;
    for stmt in stmts {
        if stmt.is_invisible() {
            continue;
        }

        let is_group_end = stmt.is_group_end();
        let requires_semicolon = Serializer::requires_semicolon(&stmt);

        serializer
            .visit_group(stmt, prev_was_group_end, prev_requires_semicolon)
            .map_err(|e| raw_to_parse_error(&map, *e, options.unicode_error_messages))?;

        prev_was_group_end = is_group_end;
        prev_requires_semicolon = requires_semicolon;
    }

    Ok(serializer.finish(prev_requires_semicolon))
}

/// Compile CSS from a path
///
/// n.b. `grass` does not currently support files or paths that are not valid UTF-8
///
/// ```
/// # use grass_compiler as grass;
/// fn main() -> Result<(), Box<grass::Error>> {
///     let css = grass::from_path("input.scss", &grass::Options::default())?;
///     Ok(())
/// }
/// ```
#[inline]
pub fn from_path<P: AsRef<Path>>(p: P, options: &Options) -> Result<String> {
    from_string_with_file_name(String::from_utf8(options.fs.read(p.as_ref())?)?, p, options)
}

/// Compile CSS from a string
///
/// ```
/// # use grass_compiler as grass;
/// fn main() -> Result<(), Box<grass::Error>> {
///     let css = grass::from_string("a { b { color: &; } }".to_string(), &grass::Options::default())?;
///     assert_eq!(css, "a b {\n  color: a b;\n}\n");
///     Ok(())
/// }
/// ```
#[inline]
pub fn from_string(input: String, options: &Options) -> Result<String> {
    from_string_with_file_name(input, "stdin", options)
}

/// Bellow is the part where WASM is added to the library
/// This library uses a custom FS system for Deno.
/// please note that it could be modified for NodeJS or even the browser.
#[cfg(feature = "wasm-exports")]
#[wasm_bindgen(getter_with_clone)]
pub struct JSOptions {
    pub load_paths: js_sys::Array,
    pub style: String,
    pub quiet: bool,
    pub unicode_error_messages: bool,
    pub input_syntax: String,
    pub allows_charset: bool,
}
#[cfg(feature = "wasm-exports")]
impl std::fmt::Debug for JSOptions {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JSOptions")
            .field("load_paths", &self.load_paths)
            .field("style", &self.style)
            .field("quiet", &self.quiet)
            .field("unicode_error_messages", &self.unicode_error_messages)
            .field("input_syntax", &self.input_syntax)
            .field("allows_charset", &self.allows_charset)
            .finish()
    }
}
#[cfg(feature = "wasm-exports")]
#[wasm_bindgen(js_name = "from_string")]
pub fn wasm_from_string(p: String, options: JSOptions ) -> std::result::Result<String, JsValue> {
    let jsopts: JSOptions = options;//serde_wasm_bindgen::from_value(options)?;
    let opt = Options {
        style: match  jsopts.style.as_str() {
            "compressed" => OutputStyle::Compressed,
            "expanded" => OutputStyle::Expanded,
            _ => OutputStyle::Compressed,
        },
        unicode_error_messages: jsopts.unicode_error_messages,
        allows_charset: jsopts.allows_charset,
        load_paths: jsopts.load_paths.iter().map(|p| std::path::PathBuf::from(p.as_string().unwrap())).collect(),
        quiet: jsopts.quiet,
        input_syntax: match jsopts.input_syntax.as_str() {
            "scss" => Some(InputSyntax::Scss),
            "sass" => Some(InputSyntax::Sass),
            "css" => Some(InputSyntax::Css),
            _ => None,
        },
        fs: &JsFS,
        ..Default::default()
    };
    from_string(p, &opt).map_err(|e| JsValue::from_str(&format!("{}", e)))
}

#[cfg(feature = "wasm-exports")]
#[wasm_bindgen(js_name = "get_config")]
pub fn wasm_get_config() -> JSOptions {
    // let load_paths = Box::new([]);
    let example = JSOptions {
        load_paths: JSArray::new(),
        style: String::from("expanded"),
        quiet: true,
        input_syntax: String::from("scss"),
        unicode_error_messages: true,
        allows_charset: true,
    };
    example
}

#[cfg(feature = "wasm-exports")]
#[wasm_bindgen(js_name = "from_file")]
pub fn wasm_from_file(path: String, options: JSOptions) -> std::result::Result<String, JsValue> {
    let jsopts: JSOptions = options;
    let opt = Options {
        style: match  jsopts.style.as_str() {
            "compressed" => OutputStyle::Compressed,
            "expanded" => OutputStyle::Expanded,
            _ => OutputStyle::Compressed,
        },
        unicode_error_messages: jsopts.unicode_error_messages,
        allows_charset: jsopts.allows_charset,
        load_paths: jsopts.load_paths.iter().map(|p| std::path::PathBuf::from(p.as_string().unwrap())).collect(),
        quiet: jsopts.quiet,
        input_syntax: match jsopts.input_syntax.as_str() {
            "scss" => Some(InputSyntax::Scss),
            "sass" => Some(InputSyntax::Sass),
            "css" => Some(InputSyntax::Css),
            _ => None,
        },
        fs: &JsFS,
        ..Default::default()
    };
    from_path(&path, &opt).map_err(|e| JsValue::from_str(&format!("{}", e)))
}
