use std::fmt;

use sdl2::ttf::{Font as SdlFont, FontError as SdlFontError};

use crate::error::{AnyhowContext, Sdl2Error};
use crate::{PeacockContext, Result, SDL_TTF_CONTEXT};

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
    pub fn from_file(ctx: &mut PeacockContext, filename: &str, size: u16) -> Result<Self> {
        let _ = ctx;
        let font = SDL_TTF_CONTEXT
            .load_font(filename, size)
            .map_err(SdlFontError::SdlError)
            .map_err(Sdl2Error::FontError)
            .with_context(|| format!("Failed to create font from file: {}", filename))?;
        Ok(Self { font })
    }
}
