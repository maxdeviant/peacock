use peacock::Result;
use peacock::{ContextBuilder, State};

type Context<'ctx> = peacock::ContextArgs<'ctx, ()>;

struct HelloWorldExample;

impl State for HelloWorldExample {
    type Context = ();

    fn update(&mut self, _ctx: Context) -> Result<()> {
        Ok(())
    }

    fn draw(&mut self, _ctx: Context, _dt: f64) -> Result<()> {
        Ok(())
    }
}

fn main() -> Result<()> {
    ContextBuilder::new("Hello, world!", 1920, 1080)
        .build_empty()?
        .run(&mut HelloWorldExample)
}
