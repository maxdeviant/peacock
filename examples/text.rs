use peacock::graphics::{self, DrawTextParams, Font, Text};
use peacock::Result;
use peacock::{Context, ContextBuilder, State, Vector2f};

struct GameState {
    font: Font,
}

impl GameState {
    fn new() -> Self {
        let font =
            Font::from_file("examples/res/Roboto-Regular.ttf").expect("Could not load font!");

        Self { font }
    }
}

impl State for GameState {
    fn update(&mut self, _ctx: &mut Context) -> Result<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context, _dt: f64) -> Result<()> {
        let text = Text::new(
            "Hello, world!\n\nI hope you enjoy using Peacock!",
            &self.font,
            24,
        );
        graphics::draw_text(
            ctx,
            &text,
            DrawTextParams {
                position: Vector2f::new(10.0, 10.0),
            },
        );

        Ok(())
    }
}

fn main() -> Result<()> {
    ContextBuilder::new("Text", 1920, 1080)
        .build()?
        .run(&mut GameState::new())
}
