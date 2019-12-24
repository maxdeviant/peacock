/// An RGBA color.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Color {
    /// The red component.
    pub r: u8,

    /// The green component.
    pub g: u8,

    /// The blue component.
    pub b: u8,

    /// The alpha component.
    pub a: u8,
}

macro_rules! predefined_rgb {
    ($name:ident, $r:expr, $g:expr, $b:expr) => {
        /// A predefined RGB [`Color`].
        pub const $name: Self = Self {
            r: $r,
            g: $g,
            b: $b,
            a: 255,
        };
    };
}

macro_rules! predefined_rgba {
    ($name:ident, $r:expr, $g:expr, $b:expr, $a:expr) => {
        /// A predefined RGBA [`Color`].
        pub const $name: Self = Self {
            r: $r,
            g: $g,
            b: $b,
            a: $a,
        };
    };
}

impl Color {
    /// Creates a new [`Color`] with RGB components.
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    /// Creates a new [`Color`] with RGBA components.
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    predefined_rgba!(TRANSPARENT, 0, 0, 0, 0);

    predefined_rgb!(ALICE_BLUE, 240, 248, 255);
    predefined_rgb!(ANTIQUE_WHITE, 250, 235, 215);
    predefined_rgb!(AQUA, 0, 255, 255);
    predefined_rgb!(AQUAMARINE, 127, 255, 212);
    predefined_rgb!(AZURE, 240, 255, 255);
    predefined_rgb!(BEIGE, 245, 245, 220);
    predefined_rgb!(BISQUE, 255, 228, 196);
    predefined_rgb!(BLACK, 0, 0, 0);
    predefined_rgb!(BLANCHED_ALMOND, 255, 235, 205);
    predefined_rgb!(BLUE_VIOLET, 138, 43, 226);
    predefined_rgb!(BLUE, 0, 0, 255);
    predefined_rgb!(BROWN, 165, 42, 42);
    predefined_rgb!(BURLYWOOD, 222, 184, 135);
    predefined_rgb!(CADET_BLUE, 95, 158, 160);
    predefined_rgb!(CHARTREUSE, 127, 255, 0);
    predefined_rgb!(CHOCOLATE, 210, 105, 30);
    predefined_rgb!(CORAL, 255, 127, 80);
    predefined_rgb!(CORNFLOWER_BLUE, 100, 149, 237);
    predefined_rgb!(CORNSILK, 255, 248, 220);
    predefined_rgb!(CRIMSON, 220, 20, 60);
    predefined_rgb!(CYAN, 0, 255, 255);
    predefined_rgb!(DARK_BLUE, 0, 0, 139);
    predefined_rgb!(DARK_CYAN, 0, 139, 139);
    predefined_rgb!(DARK_GOLDENROD, 184, 134, 11);
    predefined_rgb!(DARK_GRAY, 169, 169, 169);
    predefined_rgb!(DARK_GREEN, 0, 100, 0);
    predefined_rgb!(DARK_KHAKI, 189, 183, 107);
    predefined_rgb!(DARK_MAGENTA, 139, 0, 139);
    predefined_rgb!(DARK_OLIVE_GREEN, 85, 107, 47);
    predefined_rgb!(DARK_ORANGE, 255, 140, 0);
    predefined_rgb!(DARK_ORCHID, 153, 50, 204);
    predefined_rgb!(DARK_RED, 139, 0, 0);
    predefined_rgb!(DARK_SALMON, 233, 150, 122);
    predefined_rgb!(DARK_SEA_GREEN, 143, 188, 143);
    predefined_rgb!(DARK_SLATE_BLUE, 72, 61, 139);
    predefined_rgb!(DARK_SLATE_GRAY, 47, 79, 79);
    predefined_rgb!(DARK_TURQUOISE, 0, 206, 209);
    predefined_rgb!(DARK_VIOLET, 148, 0, 211);
    predefined_rgb!(DEEP_PINK, 255, 20, 147);
    predefined_rgb!(DEEP_SKY_BLUE, 0, 191, 255);
    predefined_rgb!(DIM_GRAY, 105, 105, 105);
    predefined_rgb!(DODGER_BLUE, 30, 144, 255);
    predefined_rgb!(FIREBRICK, 178, 34, 34);
    predefined_rgb!(FLORAL_WHITE, 255, 250, 240);
    predefined_rgb!(FOREST_GREEN, 34, 139, 34);
    predefined_rgb!(FUCHSIA, 255, 0, 255);
    predefined_rgb!(GAINSBORO, 220, 220, 220);
    predefined_rgb!(GHOST_WHITE, 248, 248, 255);
    predefined_rgb!(GOLD, 255, 215, 0);
    predefined_rgb!(GOLDENROD, 218, 165, 32);
    predefined_rgb!(GRAY, 128, 128, 128);
    predefined_rgb!(GREEN_YELLOW, 173, 255, 47);
    predefined_rgb!(GREEN, 0, 128, 0);
    predefined_rgb!(HONEYDEW, 240, 255, 240);
    predefined_rgb!(HOT_PINK, 255, 105, 180);
    predefined_rgb!(INDIAN_RED, 205, 92, 92);
    predefined_rgb!(INDIGO, 75, 0, 130);
    predefined_rgb!(IVORY, 255, 255, 240);
    predefined_rgb!(KHAKI, 240, 230, 140);
    predefined_rgb!(LAVENDER_BLUSH, 255, 240, 245);
    predefined_rgb!(LAVENDER, 230, 230, 250);
    predefined_rgb!(LAWN_GREEN, 124, 252, 0);
    predefined_rgb!(LEMON_CHIFFON, 255, 250, 205);
    predefined_rgb!(LIGHT_BLUE, 173, 216, 230);
    predefined_rgb!(LIGHT_CORAL, 240, 128, 128);
    predefined_rgb!(LIGHT_CYAN, 224, 255, 255);
    predefined_rgb!(LIGHT_GOLDENROD_YELLOW, 250, 250, 210);
    predefined_rgb!(LIGHT_GRAY, 211, 211, 211);
    predefined_rgb!(LIGHT_GREEN, 144, 238, 144);
    predefined_rgb!(LIGHT_PINK, 255, 182, 193);
    predefined_rgb!(LIGHT_SALMON, 255, 160, 122);
    predefined_rgb!(LIGHT_SEA_GREEN, 32, 178, 170);
    predefined_rgb!(LIGHT_SKY_BLUE, 135, 206, 250);
    predefined_rgb!(LIGHT_SLATE_GRAY, 119, 136, 144);
    predefined_rgb!(LIGHT_STEEL_BLUE, 176, 196, 222);
    predefined_rgb!(LIGHT_YELLOW, 255, 255, 224);
    predefined_rgb!(LIME_GREEN, 50, 205, 50);
    predefined_rgb!(LIME, 0, 255, 0);
    predefined_rgb!(LINEN, 250, 240, 230);
    predefined_rgb!(MAGENTA, 255, 0, 255);
    predefined_rgb!(MAROON, 128, 0, 0);
    predefined_rgb!(MEDIUM_AQUAMARINE, 102, 205, 170);
    predefined_rgb!(MEDIUM_BLUE, 0, 0, 205);
    predefined_rgb!(MEDIUM_ORCHID, 186, 85, 211);
    predefined_rgb!(MEDIUM_PURPLE, 147, 112, 219);
    predefined_rgb!(MEDIUM_SEA_GREEN, 60, 179, 113);
    predefined_rgb!(MEDIUM_SLATE_BLUE, 123, 104, 238);
    predefined_rgb!(MEDIUM_SPRING_GREEN, 0, 250, 154);
    predefined_rgb!(MEDIUM_TURQUOISE, 72, 209, 204);
    predefined_rgb!(MEDIUM_VIOLET_RED, 199, 21, 133);
    predefined_rgb!(MIDNIGHT_BLUE, 25, 25, 112);
    predefined_rgb!(MINT_CREAM, 245, 255, 250);
    predefined_rgb!(MISTY_ROSE, 255, 228, 225);
    predefined_rgb!(MOCCASIN, 255, 228, 181);
    predefined_rgb!(NAVAJO_WHITE, 255, 222, 173);
    predefined_rgb!(NAVY, 0, 0, 128);
    predefined_rgb!(OLD_LACE, 253, 245, 230);
    predefined_rgb!(OLIVE_DRAB, 107, 142, 35);
    predefined_rgb!(OLIVE, 128, 128, 0);
    predefined_rgb!(ORANGE_RED, 255, 69, 0);
    predefined_rgb!(ORANGE, 255, 165, 0);
    predefined_rgb!(ORCHID, 218, 112, 214);
    predefined_rgb!(PALE_GOLDENROD, 238, 232, 170);
    predefined_rgb!(PALE_GREEN, 152, 251, 152);
    predefined_rgb!(PALE_TURQUOISE, 175, 238, 238);
    predefined_rgb!(PALE_VIOLET_RED, 219, 112, 147);
    predefined_rgb!(PAPAYA_WHIP, 255, 239, 213);
    predefined_rgb!(PEACH_PUFF, 255, 218, 185);
    predefined_rgb!(PERU, 205, 133, 63);
    predefined_rgb!(PINK, 255, 192, 203);
    predefined_rgb!(PLUM, 221, 160, 221);
    predefined_rgb!(POWDER_BLUE, 176, 224, 230);
    predefined_rgb!(PURPLE, 128, 0, 128);
    predefined_rgb!(RED, 255, 0, 0);
    predefined_rgb!(ROSY_BROWN, 188, 143, 143);
    predefined_rgb!(ROYAL_BLUE, 65, 105, 225);
    predefined_rgb!(SADDLE_BROWN, 139, 69, 19);
    predefined_rgb!(SALMON, 250, 128, 114);
    predefined_rgb!(SANDY_BROWN, 244, 164, 96);
    predefined_rgb!(SEA_GREEN, 46, 139, 87);
    predefined_rgb!(SEASHELL, 255, 245, 238);
    predefined_rgb!(SIENNA, 160, 82, 45);
    predefined_rgb!(SILVER, 192, 192, 192);
    predefined_rgb!(SKY_BLUE, 135, 206, 235);
    predefined_rgb!(SLATE_BLUE, 106, 90, 205);
    predefined_rgb!(SLATE_GRAY, 112, 128, 144);
    predefined_rgb!(SNOW, 255, 250, 250);
    predefined_rgb!(SPRING_GREEN, 0, 255, 127);
    predefined_rgb!(STEEL_BLUE, 70, 130, 180);
    predefined_rgb!(TAN, 210, 180, 140);
    predefined_rgb!(TEAL, 0, 128, 128);
    predefined_rgb!(THISTLE, 216, 191, 216);
    predefined_rgb!(TOMATO, 255, 99, 71);
    predefined_rgb!(TURQUOISE, 64, 224, 208);
    predefined_rgb!(VIOLET, 238, 130, 238);
    predefined_rgb!(WHEAT, 245, 222, 179);
    predefined_rgb!(WHITE_SMOKE, 245, 245, 245);
    predefined_rgb!(WHITE, 255, 255, 255);
    predefined_rgb!(YELLOW_GREEN, 154, 205, 50);
    predefined_rgb!(YELLOW, 255, 255, 0);
}
