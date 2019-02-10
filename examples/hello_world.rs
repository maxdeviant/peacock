use peacock::error::Result;
use peacock::graphics::{self, Color};
use peacock::{Context, ContextBuilder, State};

struct GameState;

impl State for GameState {
    fn update(&mut self, _ctx: &mut Context) -> Result<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context, _dt: f64) -> Result<()> {
        graphics::clear(ctx, &Color::rgb(100, 149, 237));
        Ok(())
    }
}

fn main() -> Result<()> {
    ContextBuilder::new("Hello, world!", 1920, 1080)
        .build()?
        .run(&mut GameState)
}