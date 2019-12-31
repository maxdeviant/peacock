use crate::graphics::Font;

#[derive(Debug)]
pub struct Text<'a> {
    pub(crate) string: &'a str,
    pub(crate) font: &'a Font,
    pub(crate) size: u32,
}

impl<'a> Text<'a> {
    pub fn new(string: &'a str, font: &'a Font, size: u32) -> Self {
        Self { string, font, size }
    }
}
