use std::fmt;

use sdl2::ttf::Font as SdlFont;

use crate::{Context, Result, SDL_TTF_CONTEXT};

/// A font.
pub struct Font {
    pub(crate) font: SdlFont<'static, 'static>,
}

impl fmt::Debug for Font {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Font")
    }
}

impl Font {
    /// Creates a new [`Font`] from a file.
    pub fn from_file(ctx: &mut Context, filename: &str) -> Result<Self> {
        let font = SDL_TTF_CONTEXT.load_font(filename, 16).unwrap();
        Ok(Self { font })
    }
}
