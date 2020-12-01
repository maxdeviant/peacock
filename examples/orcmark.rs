use rand::rngs::ThreadRng;
use rand::{self, Rng};

use peacock::graphics::{self, DrawImageParams, Image, Rectangle};
use peacock::input::{self, Key};
use peacock::time;
use peacock::window;
use peacock::{ContextBuilder, Result, State, Vector2f};

type Context<'ctx> = peacock::ContextArgs<'ctx, ()>;

const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;

const INITIAL_ORCS: usize = 100;
const ORC_SCALE: f32 = 2.0;

const ORC_GRUNT_WIDTH: i32 = 11;
const ORC_GRUNT_HEIGHT: i32 = 16;

const ORC_SHAMAN_WIDTH: i32 = 11;
const ORC_SHAMAN_HEIGHT: i32 = 15;

const GRAVITY: f32 = 0.5;

enum OrcKind {
    Grunt,
    Shaman,
}

struct Orc {
    kind: OrcKind,
    position: Vector2f,
    velocity: Vector2f,
}

impl Orc {
    fn new(rng: &mut ThreadRng) -> Self {
        let kind = if rng.gen::<bool>() {
            OrcKind::Grunt
        } else {
            OrcKind::Shaman
        };

        let velocity = Vector2f::new(rng.gen::<f32>() * 5.0, rng.gen::<f32>() * 5.0 - 2.5);

        Self {
            kind,
            position: Vector2f::ZERO,
            velocity,
        }
    }
}

struct OrcMarkExample {
    rng: ThreadRng,
    sprite_sheet: Image,
    orcs: Vec<Orc>,
    spawn_timer: i32,
}

impl OrcMarkExample {
    fn new(ctx: &mut Context) -> Result<Self> {
        let mut rng = rand::thread_rng();
        let sprite_sheet = Image::from_file(ctx, "examples/res/0x72_dungeon_ii.png")?;
        let mut orcs = Vec::with_capacity(INITIAL_ORCS);

        for _ in 0..INITIAL_ORCS {
            orcs.push(Orc::new(&mut rng));
        }

        Ok(Self {
            rng,
            sprite_sheet,
            orcs,
            spawn_timer: 0,
        })
    }
}

impl State for OrcMarkExample {
    type Context = ();

    fn update(&mut self, ctx: &mut Context) -> Result<()> {
        if self.spawn_timer > 0 {
            self.spawn_timer -= 1;
        }

        if input::is_key_down(ctx, Key::Space) && self.spawn_timer == 0 {
            for _ in 0..INITIAL_ORCS {
                self.orcs.push(Orc::new(&mut self.rng));
            }

            self.spawn_timer = 10;
        }

        for orc in &mut self.orcs {
            let (orc_width, orc_height) = match orc.kind {
                OrcKind::Grunt => (ORC_GRUNT_WIDTH, ORC_GRUNT_HEIGHT),
                OrcKind::Shaman => (ORC_SHAMAN_WIDTH, ORC_SHAMAN_HEIGHT),
            };

            let max_x = WIDTH as f32 - (orc_width as f32 * ORC_SCALE);
            let max_y = HEIGHT as f32 - (orc_height as f32 * ORC_SCALE);

            orc.position += orc.velocity;
            orc.velocity.y += GRAVITY;

            if orc.position.x > max_x {
                orc.velocity.x *= -1.0;
                orc.position.x = max_x;
            } else if orc.position.x < 0.0 {
                orc.velocity.x *= -1.0;
                orc.position.x = 0.0;
            }

            if orc.position.y > max_y {
                orc.velocity.y *= -0.8;
                orc.position.y = max_y;

                if self.rng.gen::<bool>() {
                    orc.velocity.y -= 3.0 + (self.rng.gen::<f32>() * 4.0);
                }
            } else if orc.position.y < 0.0 {
                orc.velocity.y = 0.0;
                orc.position.y = 0.0;
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context, _dt: f64) -> Result<()> {
        for orc in &self.orcs {
            let clip_rect = match orc.kind {
                OrcKind::Grunt => {
                    Rectangle::<i32>::new(372, 208, ORC_GRUNT_WIDTH, ORC_GRUNT_HEIGHT)
                }
                OrcKind::Shaman => {
                    Rectangle::<i32>::new(372, 241, ORC_SHAMAN_WIDTH, ORC_SHAMAN_HEIGHT)
                }
            };

            graphics::draw(
                ctx,
                &self.sprite_sheet,
                &DrawImageParams {
                    position: orc.position,
                    clip_rect: Some(clip_rect),
                    scale: Some(Vector2f::new(ORC_SCALE, ORC_SCALE)),
                    ..Default::default()
                },
            )?;
        }

        window::set_title(
            ctx,
            &format!(
                "OrcMark - {} orcs - {:.0} FPS",
                self.orcs.len(),
                time::get_fps(ctx)
            ),
        );

        Ok(())
    }
}

fn main() -> Result<()> {
    ContextBuilder::new("OrcMark", WIDTH, HEIGHT)
        .build_empty()?
        .run_with_result(OrcMarkExample::new)
}
