use sdl2::ttf::FontError;
use thiserror::Error;

pub use anyhow::Result;

/// An error originating from SDL2.
#[derive(Debug, Error)]
pub enum Sdl2Error {
    /// An SDL2 font error.
    #[error("Encountered an SDL2 font error")]
    FontError(#[from] FontError),

    /// A generic SDL2 error.
    #[error("Encountered an SDL2 error: {0}")]
    ErrorMessage(String),
}
