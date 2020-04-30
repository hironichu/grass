//! A big dictionary of named colors and their
//! corresponding RGBA values

use bimap::BiMap;
use once_cell::sync::Lazy;

pub(crate) static NAMED_COLORS: Lazy<BiMap<&'static str, [u8; 4]>> = Lazy::new(|| {
    let mut m = BiMap::with_capacity(150);
    m.insert("aliceblue", [0xF0, 0xF8, 0xFF, 0xFF]);
    m.insert("antiquewhite", [0xFA, 0xEB, 0xD7, 0xFF]);
    m.insert("aqua", [0x00, 0xFF, 0xFF, 0xFF]);
    m.insert("aquamarine", [0x7F, 0xFF, 0xD4, 0xFF]);
    m.insert("azure", [0xF0, 0xFF, 0xFF, 0xFF]);
    m.insert("beige", [0xF5, 0xF5, 0xDC, 0xFF]);
    m.insert("bisque", [0xFF, 0xE4, 0xC4, 0xFF]);
    m.insert("black", [0x00, 0x00, 0x00, 0xFF]);
    m.insert("blanchedalmond", [0xFF, 0xEB, 0xCD, 0xFF]);
    m.insert("blue", [0x00, 0x00, 0xFF, 0xFF]);
    m.insert("blueviolet", [0x8A, 0x2B, 0xE2, 0xFF]);
    m.insert("brown", [0xA5, 0x2A, 0x2A, 0xFF]);
    m.insert("burlywood", [0xDE, 0xB8, 0x87, 0xFF]);
    m.insert("cadetblue", [0x5F, 0x9E, 0xA0, 0xFF]);
    m.insert("chartreuse", [0x7F, 0xFF, 0x00, 0xFF]);
    m.insert("chocolate", [0xD2, 0x69, 0x1E, 0xFF]);
    m.insert("coral", [0xFF, 0x7F, 0x50, 0xFF]);
    m.insert("cornflowerblue", [0x64, 0x95, 0xED, 0xFF]);
    m.insert("cornsilk", [0xFF, 0xF8, 0xDC, 0xFF]);
    m.insert("crimson", [0xDC, 0x14, 0x3C, 0xFF]);
    m.insert("darkblue", [0x00, 0x00, 0x8B, 0xFF]);
    m.insert("darkcyan", [0x00, 0x8B, 0x8B, 0xFF]);
    m.insert("darkgoldenrod", [0xB8, 0x86, 0x0B, 0xFF]);
    m.insert("darkgray", [0xA9, 0xA9, 0xA9, 0xFF]);
    m.insert("darkgreen", [0x00, 0x64, 0x00, 0xFF]);
    m.insert("darkkhaki", [0xBD, 0xB7, 0x6B, 0xFF]);
    m.insert("darkmagenta", [0x8B, 0x00, 0x8B, 0xFF]);
    m.insert("darkolivegreen", [0x55, 0x6B, 0x2F, 0xFF]);
    m.insert("darkorange", [0xFF, 0x8C, 0x00, 0xFF]);
    m.insert("darkorchid", [0x99, 0x32, 0xCC, 0xFF]);
    m.insert("darkred", [0x8B, 0x00, 0x00, 0xFF]);
    m.insert("darksalmon", [0xE9, 0x96, 0x7A, 0xFF]);
    m.insert("darkseagreen", [0x8F, 0xBC, 0x8F, 0xFF]);
    m.insert("darkslateblue", [0x48, 0x3D, 0x8B, 0xFF]);
    m.insert("darkslategray", [0x2F, 0x4F, 0x4F, 0xFF]);
    m.insert("darkturquoise", [0x00, 0xCE, 0xD1, 0xFF]);
    m.insert("darkviolet", [0x94, 0x00, 0xD3, 0xFF]);
    m.insert("deeppink", [0xFF, 0x14, 0x93, 0xFF]);
    m.insert("deepskyblue", [0x00, 0xBF, 0xFF, 0xFF]);
    m.insert("dimgray", [0x69, 0x69, 0x69, 0xFF]);
    m.insert("dodgerblue", [0x1E, 0x90, 0xFF, 0xFF]);
    m.insert("firebrick", [0xB2, 0x22, 0x22, 0xFF]);
    m.insert("floralwhite", [0xFF, 0xFA, 0xF0, 0xFF]);
    m.insert("forestgreen", [0x22, 0x8B, 0x22, 0xFF]);
    m.insert("fuchsia", [0xFF, 0x00, 0xFF, 0xFF]);
    m.insert("gainsboro", [0xDC, 0xDC, 0xDC, 0xFF]);
    m.insert("ghostwhite", [0xF8, 0xF8, 0xFF, 0xFF]);
    m.insert("gold", [0xFF, 0xD7, 0x00, 0xFF]);
    m.insert("goldenrod", [0xDA, 0xA5, 0x20, 0xFF]);
    m.insert("gray", [0x80, 0x80, 0x80, 0xFF]);
    m.insert("green", [0x00, 0x80, 0x00, 0xFF]);
    m.insert("greenyellow", [0xAD, 0xFF, 0x2F, 0xFF]);
    m.insert("honeydew", [0xF0, 0xFF, 0xF0, 0xFF]);
    m.insert("hotpink", [0xFF, 0x69, 0xB4, 0xFF]);
    m.insert("indianred", [0xCD, 0x5C, 0x5C, 0xFF]);
    m.insert("indigo", [0x4B, 0x00, 0x82, 0xFF]);
    m.insert("ivory", [0xFF, 0xFF, 0xF0, 0xFF]);
    m.insert("khaki", [0xF0, 0xE6, 0x8C, 0xFF]);
    m.insert("lavender", [0xE6, 0xE6, 0xFA, 0xFF]);
    m.insert("lavenderblush", [0xFF, 0xF0, 0xF5, 0xFF]);
    m.insert("lawngreen", [0x7C, 0xFC, 0x00, 0xFF]);
    m.insert("lemonchiffon", [0xFF, 0xFA, 0xCD, 0xFF]);
    m.insert("lightblue", [0xAD, 0xD8, 0xE6, 0xFF]);
    m.insert("lightcoral", [0xF0, 0x80, 0x80, 0xFF]);
    m.insert("lightcyan", [0xE0, 0xFF, 0xFF, 0xFF]);
    m.insert("lightgoldenrodyellow", [0xFA, 0xFA, 0xD2, 0xFF]);
    m.insert("lightgray", [0xD3, 0xD3, 0xD3, 0xFF]);
    m.insert("lightgreen", [0x90, 0xEE, 0x90, 0xFF]);
    m.insert("lightpink", [0xFF, 0xB6, 0xC1, 0xFF]);
    m.insert("lightsalmon", [0xFF, 0xA0, 0x7A, 0xFF]);
    m.insert("lightseagreen", [0x20, 0xB2, 0xAA, 0xFF]);
    m.insert("lightskyblue", [0x87, 0xCE, 0xFA, 0xFF]);
    m.insert("lightslategray", [0x77, 0x88, 0x99, 0xFF]);
    m.insert("lightsteelblue", [0xB0, 0xC4, 0xDE, 0xFF]);
    m.insert("lightyellow", [0xFF, 0xFF, 0xE0, 0xFF]);
    m.insert("lime", [0x00, 0xFF, 0x00, 0xFF]);
    m.insert("limegreen", [0x32, 0xCD, 0x32, 0xFF]);
    m.insert("linen", [0xFA, 0xF0, 0xE6, 0xFF]);
    m.insert("maroon", [0x80, 0x00, 0x00, 0xFF]);
    m.insert("mediumaquamarine", [0x66, 0xCD, 0xAA, 0xFF]);
    m.insert("mediumblue", [0x00, 0x00, 0xCD, 0xFF]);
    m.insert("mediumorchid", [0xBA, 0x55, 0xD3, 0xFF]);
    m.insert("mediumpurple", [0x93, 0x70, 0xDB, 0xFF]);
    m.insert("mediumseagreen", [0x3C, 0xB3, 0x71, 0xFF]);
    m.insert("mediumslateblue", [0x7B, 0x68, 0xEE, 0xFF]);
    m.insert("mediumspringgreen", [0x00, 0xFA, 0x9A, 0xFF]);
    m.insert("mediumturquoise", [0x48, 0xD1, 0xCC, 0xFF]);
    m.insert("mediumvioletred", [0xC7, 0x15, 0x85, 0xFF]);
    m.insert("midnightblue", [0x19, 0x19, 0x70, 0xFF]);
    m.insert("mintcream", [0xF5, 0xFF, 0xFA, 0xFF]);
    m.insert("mistyrose", [0xFF, 0xE4, 0xE1, 0xFF]);
    m.insert("moccasin", [0xFF, 0xE4, 0xB5, 0xFF]);
    m.insert("navajowhite", [0xFF, 0xDE, 0xAD, 0xFF]);
    m.insert("navy", [0x00, 0x00, 0x80, 0xFF]);
    m.insert("oldlace", [0xFD, 0xF5, 0xE6, 0xFF]);
    m.insert("olive", [0x80, 0x80, 0x00, 0xFF]);
    m.insert("olivedrab", [0x6B, 0x8E, 0x23, 0xFF]);
    m.insert("orange", [0xFF, 0xA5, 0x00, 0xFF]);
    m.insert("orangered", [0xFF, 0x45, 0x00, 0xFF]);
    m.insert("orchid", [0xDA, 0x70, 0xD6, 0xFF]);
    m.insert("palegoldenrod", [0xEE, 0xE8, 0xAA, 0xFF]);
    m.insert("palegreen", [0x98, 0xFB, 0x98, 0xFF]);
    m.insert("paleturquoise", [0xAF, 0xEE, 0xEE, 0xFF]);
    m.insert("palevioletred", [0xDB, 0x70, 0x93, 0xFF]);
    m.insert("papayawhip", [0xFF, 0xEF, 0xD5, 0xFF]);
    m.insert("peachpuff", [0xFF, 0xDA, 0xB9, 0xFF]);
    m.insert("peru", [0xCD, 0x85, 0x3F, 0xFF]);
    m.insert("pink", [0xFF, 0xC0, 0xCB, 0xFF]);
    m.insert("plum", [0xDD, 0xA0, 0xDD, 0xFF]);
    m.insert("powderblue", [0xB0, 0xE0, 0xE6, 0xFF]);
    m.insert("purple", [0x80, 0x00, 0x80, 0xFF]);
    m.insert("rebeccapurple", [0x66, 0x33, 0x99, 0xFF]);
    m.insert("red", [0xFF, 0x00, 0x00, 0xFF]);
    m.insert("rosybrown", [0xBC, 0x8F, 0x8F, 0xFF]);
    m.insert("royalblue", [0x41, 0x69, 0xE1, 0xFF]);
    m.insert("saddlebrown", [0x8B, 0x45, 0x13, 0xFF]);
    m.insert("salmon", [0xFA, 0x80, 0x72, 0xFF]);
    m.insert("sandybrown", [0xF4, 0xA4, 0x60, 0xFF]);
    m.insert("seagreen", [0x2E, 0x8B, 0x57, 0xFF]);
    m.insert("seashell", [0xFF, 0xF5, 0xEE, 0xFF]);
    m.insert("sienna", [0xA0, 0x52, 0x2D, 0xFF]);
    m.insert("silver", [0xC0, 0xC0, 0xC0, 0xFF]);
    m.insert("skyblue", [0x87, 0xCE, 0xEB, 0xFF]);
    m.insert("slateblue", [0x6A, 0x5A, 0xCD, 0xFF]);
    m.insert("slategray", [0x70, 0x80, 0x90, 0xFF]);
    m.insert("snow", [0xFF, 0xFA, 0xFA, 0xFF]);
    m.insert("springgreen", [0x00, 0xFF, 0x7F, 0xFF]);
    m.insert("steelblue", [0x46, 0x82, 0xB4, 0xFF]);
    m.insert("tan", [0xD2, 0xB4, 0x8C, 0xFF]);
    m.insert("teal", [0x00, 0x80, 0x80, 0xFF]);
    m.insert("thistle", [0xD8, 0xBF, 0xD8, 0xFF]);
    m.insert("tomato", [0xFF, 0x63, 0x47, 0xFF]);
    m.insert("turquoise", [0x40, 0xE0, 0xD0, 0xFF]);
    m.insert("violet", [0xEE, 0x82, 0xEE, 0xFF]);
    m.insert("wheat", [0xF5, 0xDE, 0xB3, 0xFF]);
    m.insert("white", [0xFF, 0xFF, 0xFF, 0xFF]);
    m.insert("whitesmoke", [0xF5, 0xF5, 0xF5, 0xFF]);
    m.insert("yellow", [0xFF, 0xFF, 0x00, 0xFF]);
    m.insert("yellowgreen", [0x9A, 0xCD, 0x32, 0xFF]);
    m.insert("transparent", [0x00, 0x00, 0x00, 0x00]);
    m
});
