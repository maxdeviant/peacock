use peacock::graphics::{self, Animation, DrawAnimationParams, Image, Rectangle, View};
use peacock::window;
use peacock::{Context, ContextBuilder, Result, State};

struct GameState {
    animation: Animation,
}

impl GameState {
    fn new(ctx: &mut Context) -> Result<Self> {
        let sprite_sheet = Image::from_file(ctx, "examples/res/0x72_dungeon_ii.png")?;

        let animation = Animation::new(
            sprite_sheet,
            vec![
                Rectangle::<i32>::new(128, 76, 15, 20),
                Rectangle::<i32>::new(144, 76, 15, 20),
                Rectangle::<i32>::new(160, 76, 15, 20),
                Rectangle::<i32>::new(176, 76, 15, 20),
            ],
            8,
        );

        Ok(Self { animation })
    }
}

impl State for GameState {
    fn update(&mut self, _ctx: &mut Context) -> Result<()> {
        self.animation.tick();

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context, _dt: f64) -> Result<()> {
        let mut view = View::new((0.0, 0.0).into(), (1920.0, 1080.0).into());
        view.set_zoom(8.0);

        window::set_view(ctx, &view);

        graphics::draw(
            ctx,
            &self.animation,
            &DrawAnimationParams {
                ..DrawAnimationParams::default()
            },
        )?;

        Ok(())
    }
}

fn main() -> Result<()> {
    ContextBuilder::new("Animation", 1920, 1080)
        .build()?
        .run_with_result(GameState::new)
}
