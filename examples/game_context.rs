use peacock::graphics::{self, DrawTextParams, Font, Text};
use peacock::{ContextBuilder, PeacockContext, Result, State, Vector2f};

type Context<'ctx> = peacock::ContextArgs<'ctx, GameContext>;

struct GameContext {
    font: Font,
}

impl GameContext {
    fn new(ctx: &mut PeacockContext) -> Result<Self> {
        Ok(Self {
            font: Font::from_file(ctx, "examples/res/Roboto-Regular.ttf", 24)?,
        })
    }
}

struct GameContextExample {
    message: Text,
}

impl GameContextExample {
    fn new(ctx: Context) -> Result<Self> {
        Ok(Self {
            message: Text::new(ctx.ctx, "Hello", &ctx.game.font)?,
        })
    }
}

impl State for GameContextExample {
    type Context = GameContext;

    fn update(&mut self, _ctx: Context) -> Result<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: Context, _dt: f64) -> Result<()> {
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
