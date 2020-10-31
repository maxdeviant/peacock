use std::fmt;

use anyhow::Context as AnyhowContext;
use sdl2::ttf::{Font as SdlFont, FontError as SdlFontError};

use crate::error::Sdl2Error;
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
        let _ = ctx;
        let font = SDL_TTF_CONTEXT
            .load_font(filename, 16)
            .map_err(SdlFontError::SdlError)
            .map_err(Sdl2Error::FontError)
            .with_context(|| format!("Failed to create font from file: {}", filename))?;
        Ok(Self { font })
    }
}
