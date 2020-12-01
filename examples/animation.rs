use peacock::graphics::{self, Animation, DrawAnimationParams, Image, Rectangle, View};
use peacock::window;
use peacock::{ContextBuilder, Result, State};

type Context<'ctx> = peacock::ContextArgs<'ctx, ()>;

struct AnimationExample {
    animation: Animation,
}

impl AnimationExample {
    fn new(ctx: Context) -> Result<Self> {
        let sprite_sheet = Image::from_file(ctx.ctx, "examples/res/0x72_dungeon_ii.png")?;

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

impl<'ctx> State for AnimationExample {
    type Context = ();

    fn update(&mut self, _ctx: Context) -> Result<()> {
        self.animation.tick();

        Ok(())
    }

    fn draw(&mut self, ctx: Context, _dt: f64) -> Result<()> {
        let mut view = View::new((0.0, 0.0).into(), (1920.0, 1080.0).into());
        view.set_zoom(8.0);

        window::set_view(ctx.ctx, &view);

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
        .build_empty()?
        .run_with_result(AnimationExample::new)
}
