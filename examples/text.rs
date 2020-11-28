use peacock::graphics::{self, DrawTextParams, Font, Text};
use peacock::Result;
use peacock::{Context, ContextBuilder, State, Vector2f};

struct GameState {
    greeting_message: Text,
}

impl GameState {
    fn new(ctx: &mut Context) -> Result<Self> {
        let font = Font::from_file(ctx, "examples/res/Roboto-Regular.ttf", 24)?;

        let greeting_message = Text::new(
            ctx,
            "Hello, world!\n\nI hope you enjoy using Peacock!",
            &font,
        )?;

        Ok(Self { greeting_message })
    }
}

impl State for GameState {
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

        Ok(())
    }
}

fn main() -> Result<()> {
    ContextBuilder::new("Text", 1920, 1080)
        .build()?
        .run_with_result(GameState::new)
}
