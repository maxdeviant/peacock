use std::ops::Add;

use sfml::graphics::IntRect as SfIntRect;

/// A rectangle.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Rectangle<T> {
    pub x: T,
    pub y: T,
    pub width: T,
    pub height: T,
}

impl<T> Rectangle<T>
where
    T: Copy + Add<Output = T>,
{
    /// Creates a new [`Rectangle`].
    pub fn new(x: T, y: T, width: T, height: T) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn left(&self) -> T {
        self.x
    }

    pub fn top(&self) -> T {
        self.y
    }

    pub fn right(&self) -> T {
        self.x + self.width
    }

    pub fn bottom(&self) -> T {
        self.y + self.height
    }
}

impl From<Rectangle<i32>> for SfIntRect {
    fn from(rectangle: Rectangle<i32>) -> Self {
        SfIntRect::new(rectangle.x, rectangle.y, rectangle.width, rectangle.height)
    }
}
