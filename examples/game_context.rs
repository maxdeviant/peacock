use peacock::graphics::{self, DrawTextParams, Font, Text};
use peacock::{ContextBuilder, Result, State, Vector2f};

type Context = peacock::Context<GameContext>;

struct GameContext {
    font: Font,
}

impl GameContext {
    fn new(ctx: &mut peacock::Context<()>) -> Result<Self> {
        Ok(Self {
            font: Font::from_file(ctx, "examples/res/Roboto-Regular.ttf", 24)?,
        })
    }
}

struct GameContextExample {
    message: Text,
}

impl GameContextExample {
    fn new(ctx: &mut Context) -> Result<Self> {
        Ok(Self {
            message: Text::new(ctx, "Hello", &ctx.game().font)?,
        })
    }
}

impl State for GameContextExample {
    type Context = GameContext;

    fn update(&mut self, _ctx: &mut Context) -> Result<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context, _dt: f64) -> Result<()> {
        graphics::draw(
            ctx,
            &self.message,
            &DrawTextParams {
                position: Vector2f::ZERO,
                ..Default::default()
            },
        )?;

        Ok(())
    }
}

fn main() -> Result<()> {
    ContextBuilder::new("Game Context", 1920, 1080)
        .build(GameContext::new)?
        .run_with_result(GameContextExample::new)
}
