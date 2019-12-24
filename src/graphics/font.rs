use crate::error::Error;
use crate::Result;

/// A font.
#[derive(Debug)]
pub struct Font {
    font: SfFont,
}

impl Font {
    /// Creates a new [`Font`] from a file.
    pub fn from_file(filename: &str) -> Result<Self> {
        let font = SfFont::from_file(filename).ok_or(Error)?;
        Ok(Self { font })
    }
}
