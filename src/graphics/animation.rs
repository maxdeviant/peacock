use crate::graphics::{self, DrawImageParams, Drawable, Image, Rectangle};
use crate::Context;

#[derive(Debug)]
pub struct Animation {
    texture: Image,

    /// The frames in the animation.
    frames: Vec<Rectangle<i32>>,

    /// The length of a frame, in ticks.
    frame_length: i32,

    /// The index of the current animation frame.
    current_frame: usize,
    timer: i32,
}

impl Animation {
    pub fn new(texture: Image, frames: Vec<Rectangle<i32>>, frame_length: i32) -> Self {
        Self {
            texture,
            frames,
            frame_length,
            current_frame: 0,
            timer: 0,
        }
    }

    pub fn tick(&mut self) {
        self.timer += 1;

        if self.timer >= self.frame_length {
            self.current_frame = (self.current_frame + 1) % self.frames.len();
            self.timer = 0;
        }
    }

    /// Restarts the animation from the beginning.
    pub fn restart(&mut self) {
        self.current_frame = 0;
        self.timer = 0;
    }
}

impl Drawable for Animation {
    fn draw(&self, ctx: &mut Context) {
        graphics::draw_image(
            ctx,
            &self.texture,
            DrawImageParams {
                clip_rect: Some(self.frames[self.current_frame]),
                ..Default::default()
            },
        )
    }
}
