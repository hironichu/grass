#![cfg(test)]

#[macro_use]
mod macros;

test!(preserves_named_color_case, "a {\n  color: OrAnGe;\n}\n");
test!(preserves_hex_color_case, "a {\n  color: #FfFfFf;\n}\n");
test!(
    preserves_hex_8_val_10000000,
    "a {\n  color: #10000000;\n}\n"
);
test!(
    preserves_hex_8_val_12312312,
    "a {\n  color: #12312312;\n}\n"
);
test!(
    preserves_hex_8_val_ab234cff,
    "a {\n  color: #ab234cff;\n}\n"
);
test!(preserves_hex_6_val_000000, "a {\n  color: #000000;\n}\n");
test!(preserves_hex_6_val_123123, "a {\n  color: #123123;\n}\n");
test!(preserves_hex_6_val_ab234c, "a {\n  color: #ab234c;\n}\n");
test!(preserves_hex_4_val_0000, "a {\n  color: #0000;\n}\n");
test!(preserves_hex_4_val_123a, "a {\n  color: #123a;\n}\n");
test!(preserves_hex_4_val_ab2f, "a {\n  color: #ab2f;\n}\n");
test!(preserves_hex_3_val_000, "a {\n  color: #000;\n}\n");
test!(preserves_hex_3_val_123, "a {\n  color: #123;\n}\n");
test!(preserves_hex_3_val_ab2, "a {\n  color: #ab2;\n}\n");
test!(
    converts_rgb_to_named_color,
    "a {\n  color: rgb(0, 0, 0);\n}\n",
    "a {\n  color: black;\n}\n"
);
test!(
    converts_rgba_to_named_color_red,
    "a {\n  color: rgb(255, 0, 0, 255);\n}\n",
    "a {\n  color: red;\n}\n"
);
test!(
    rgb_binop,
    "a {\n  color: rgb(1, 2, 1+2);\n}\n",
    "a {\n  color: #010203;\n}\n"
);
test!(
    rgb_pads_0,
    "a {\n  color: rgb(1, 2, 3);\n}\n",
    "a {\n  color: #010203;\n}\n"
);
test!(
    rgba_percent,
    "a {\n  color: rgba(159%, 169, 169%, 50%);\n}\n",
    "a {\n  color: rgba(255, 169, 255, 0.5);\n}\n"
);
test!(
    rgba_percent_round_up,
    "a {\n  color: rgba(59%, 169, 69%, 50%);\n}\n",
    "a {\n  color: rgba(150, 169, 176, 0.5);\n}\n"
);
test!(
    rgb_double_digits,
    "a {\n  color: rgb(254, 255, 255);\n}\n",
    "a {\n  color: #feffff;\n}\n"
);
test!(
    rgb_double_digits_white,
    "a {\n  color: rgb(255, 255, 255);\n}\n",
    "a {\n  color: white;\n}\n"
);
test!(
    alpha_function_4_hex,
    "a {\n  color: alpha(#0123);\n}\n",
    "a {\n  color: 0.2;\n}\n"
);
test!(
    alpha_function_named_color,
    "a {\n  color: alpha(red);\n}\n",
    "a {\n  color: 1;\n}\n"
);
test!(opacity_function_number, "a {\n  color: opacity(1);\n}\n");
test!(
    opacity_function_number_unit,
    "a {\n  color: opacity(1px);\n}\n"
);
test!(
    rgba_opacity_over_1,
    "a {\n  color: rgba(1, 2, 3, 3);\n}\n",
    "a {\n  color: #010203;\n}\n"
);
test!(
    rgba_negative_alpha,
    "a {\n  color: rgba(1, 2, 3, -10%);\n}\n",
    "a {\n  color: rgba(1, 2, 3, 0);\n}\n"
);
test!(
    rgba_opacity_decimal,
    "a {\n  color: rgba(1, 2, 3, .6);\n}\n",
    "a {\n  color: rgba(1, 2, 3, 0.6);\n}\n"
);
test!(
    rgba_opacity_percent,
    "a {\n  color: rgba(1, 2, 3, 50%);\n}\n",
    "a {\n  color: rgba(1, 2, 3, 0.5);\n}\n"
);
test!(
    hsl_basic,
    "a {\n  color: hsl(193, 67%, 99);\n}\n",
    "a {\n  color: #fbfdfe;\n}\n"
);
test!(
    hsla_basic,
    "a {\n  color: hsla(193, 67%, 99, .6);\n}\n",
    "a {\n  color: rgba(251, 253, 254, 0.6);\n}\n"
);
test!(
    hsl_named,
    "a {\n  color: hsl($hue: 193, $saturation: 67%, $luminance: 99);\n}\n",
    "a {\n  color: #fbfdfe;\n}\n"
);
test!(
    hsla_named,
    "a {\n  color: hsla($hue: 193, $saturation: 67%, $luminance: 99, $alpha: .6);\n}\n",
    "a {\n  color: rgba(251, 253, 254, 0.6);\n}\n"
);
test!(
    color_plus_ident,
    "a {\n  color: red + foo;\n}\n",
    "a {\n  color: redfoo;\n}\n"
);
test!(
    ident_plus_color,
    "a {\n  color: foo + red;\n}\n",
    "a {\n  color: foored;\n}\n"
);
test!(
    color_minus_ident,
    "a {\n  color: red - foo;\n}\n",
    "a {\n  color: red-foo;\n}\n"
);
test!(
    color_minus_dbl_quote_ident,
    "a {\n  color: red - \"foo\";\n}\n",
    "a {\n  color: red-\"foo\";\n}\n"
);
test!(
    color_minus_sgl_quote_ident,
    "a {\n  color: red - 'foo';\n}\n",
    "a {\n  color: red-\"foo\";\n}\n"
);
test!(
    color_minus_important,
    "a {\n  color: red - !important;\n}\n",
    "a {\n  color: red-!important;\n}\n"
);
test!(
    color_minus_null,
    "a {\n  color: red - null;\n}\n",
    "a {\n  color: red-;\n}\n"
);
test!(
    ident_minus_color,
    "a {\n  color: foo - red;\n}\n",
    "a {\n  color: foo-red;\n}\n"
);
test!(
    hue,
    "$a: hsl(193, 67%, 28%);\n\na {\n  color: hue($a);\n}\n",
    "a {\n  color: 193deg;\n}\n"
);
test!(
    saturation,
    "$a: hsl(193, 67%, 28%);\n\na {\n  color: saturation($a);\n}\n",
    "a {\n  color: 67%;\n}\n"
);
test!(
    saturation_2,
    "$a: hsl(1, 1, 10);\n\na {\n  color: saturation($a);\n}\n",
    "a {\n  color: 1%;\n}\n"
);
test!(
    lightness,
    "$a: hsl(193, 67%, 28%);\n\na {\n  color: lightness($a);\n}\n",
    "a {\n  color: 28%;\n}\n"
);
test!(
    invert_no_weight,
    "a {\n  color: invert(white);\n}\n",
    "a {\n  color: black;\n}\n"
);
test!(
    invert_weight_percent,
    "a {\n  color: invert(white, 20%);\n}\n",
    "a {\n  color: #cccccc;\n}\n"
);
test!(
    invert_weight_no_unit,
    "a {\n  color: invert(white, 20);\n}\n",
    "a {\n  color: #cccccc;\n}\n"
);