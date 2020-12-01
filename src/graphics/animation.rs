use crate::graphics::{Color, DrawImageParams, Drawable, Image, Rectangle};
use crate::{Context, ContextArgs, Result, Vector2f};

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

/// The parameters for drawing an [`Animation`] to the current render target.
#[derive(Debug, Default)]
pub struct DrawAnimationParams {
    /// The position at which to draw the [`Animation`].
    pub position: Vector2f,

    pub color: Option<Color>,

    pub scale: Option<Vector2f>,
}

impl<G> Drawable<G> for Animation {
    type Params = DrawAnimationParams;

    fn draw(&self, ctx: &mut ContextArgs<G>, params: &DrawAnimationParams) -> Result<()> {
        self.texture.draw(
            ctx,
            &DrawImageParams {
                clip_rect: Some(self.frames[self.current_frame]),
                position: params.position,
                color: params.color,
                scale: params.scale,
            },
        )
    }
}
