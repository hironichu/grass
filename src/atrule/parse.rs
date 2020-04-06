use std::iter::Peekable;

use crate::error::SassResult;
use crate::scope::Scope;
use crate::selector::Selector;
use crate::{eat_expr, Expr, RuleSet, Stmt, Token};

pub(crate) fn eat_stmts<I: Iterator<Item = Token>>(
    toks: &mut Peekable<I>,
    scope: &mut Scope,
    super_selector: &Selector,
) -> SassResult<Vec<Stmt>> {
    let mut stmts = Vec::new();
    while let Some(expr) = eat_expr(toks, scope, super_selector)? {
        match expr {
            Expr::AtRule(a) => stmts.push(Stmt::AtRule(a)),
            Expr::Style(s) => stmts.push(Stmt::Style(s)),
            Expr::Styles(s) => stmts.extend(s.into_iter().map(Box::new).map(Stmt::Style)),
            Expr::Include(s) => stmts.extend(s),
            Expr::MixinDecl(..) | Expr::FunctionDecl(..) | Expr::Debug(..) | Expr::Warn(..) => {
                todo!()
            }
            Expr::Selector(selector) => {
                let rules = eat_stmts(toks, scope, &super_selector.zip(&selector))?;
                stmts.push(Stmt::RuleSet(RuleSet {
                    super_selector: super_selector.clone(),
                    selector,
                    rules,
                }));
            }
            Expr::VariableDecl(name, val) => {
                scope.insert_var(&name, *val)?;
            }
            Expr::MultilineComment(s) => stmts.push(Stmt::MultilineComment(s)),
        }
    }
    Ok(stmts)
}

pub(crate) fn eat_stmts_at_root<I: Iterator<Item = Token>>(
    toks: &mut Peekable<I>,
    scope: &mut Scope,
    super_selector: &Selector,
    mut nesting: usize,
    is_some: bool,
) -> SassResult<Vec<Stmt>> {
    let mut stmts = Vec::new();
    while let Some(expr) = eat_expr(toks, scope, super_selector)? {
        match expr {
            Expr::AtRule(a) => stmts.push(Stmt::AtRule(a)),
            Expr::Style(s) => stmts.push(Stmt::Style(s)),
            Expr::Styles(s) => stmts.extend(s.into_iter().map(Box::new).map(Stmt::Style)),
            Expr::Include(s) => stmts.extend(s),
            Expr::MixinDecl(..) | Expr::FunctionDecl(..) | Expr::Debug(..) | Expr::Warn(..) => {
                todo!()
            }
            Expr::Selector(mut selector) => {
                if nesting > 1 || is_some {
                    selector = super_selector.zip(&selector);
                } else {
                    selector = Selector::replace(super_selector.clone(), selector);
                }
                nesting += 1;
                let rules = eat_stmts_at_root(toks, scope, &selector, nesting, true)?;
                nesting -= 1;
                stmts.push(Stmt::RuleSet(RuleSet {
                    super_selector: if nesting > 1 {
                        super_selector.clone()
                    } else {
                        Selector::new()
                    },
                    selector,
                    rules,
                }));
            }
            Expr::VariableDecl(name, val) => {
                scope.insert_var(&name, *val)?;
            }
            Expr::MultilineComment(s) => stmts.push(Stmt::MultilineComment(s)),
        }
    }
    Ok(stmts)
}
