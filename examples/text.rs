use peacock::graphics::{self, DrawTextParams, Font, Text};
use peacock::Result;
use peacock::{ContextBuilder, State, Vector2f};

type Context = peacock::Context<()>;

struct TextExample {
    greeting_message: Text,
    centered_text: Text,
}

impl TextExample {
    fn new(ctx: &mut Context) -> Result<Self> {
        let font = Font::from_file(ctx, "examples/res/Roboto-Regular.ttf", 24)?;

        let greeting_message = Text::new(
            ctx,
            "Hello, world!\n\nI hope you enjoy using Peacock!",
            &font,
        )?;

        let centered_text = Text::new(ctx, "This text is centered on the screen", &font)?;

        Ok(Self {
            greeting_message,
            centered_text,
        })
    }
}

impl State for TextExample {
    type Context = ();

    fn update(&mut self, _ctx: &mut Context) -> Result<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context, _dt: f64) -> Result<()> {
        graphics::draw(
            ctx,
            &self.greeting_message,
            &DrawTextParams {
                position: Vector2f::new(10.0, 10.0),
                ..Default::default()
            },
        )?;

        graphics::draw(
            ctx,
            &self.centered_text,
            &DrawTextParams {
                position: Vector2f::new((1920 / 2 - self.centered_text.size().x / 2) as f32, 50.0),
                ..Default::default()
            },
        )?;

        Ok(())
    }
}

fn main() -> Result<()> {
    ContextBuilder::new("Text", 1920, 1080)
        .build_empty()?
        .run_with_result(TextExample::new)
}
