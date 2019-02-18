use rand::rngs::ThreadRng;
use rand::{self, Rng};

use peacock::graphics::{self, DrawImageParams, Image, Rectangle};
use peacock::input::Key;
use peacock::time;
use peacock::window;
use peacock::{Context, ContextBuilder, Result, State, Vector2f};

const WIDTH: i32 = 1920;
const HEIGHT: i32 = 1080;

const INITIAL_ORCS: usize = 100;
const ORC_WIDTH: i32 = 11;
const ORC_HEIGHT: i32 = 15;

const GRAVITY: f32 = 0.5;

struct Orc {
    position: Vector2f,
    velocity: Vector2f,
}

impl Orc {
    fn new(rng: &mut ThreadRng) -> Self {
        let velocity = Vector2f::new(rng.gen::<f32>() * 5.0, rng.gen::<f32>() * 5.0 - 2.5);

        Self {
            position: Vector2f::ZERO,
            velocity,
        }
    }
}

struct GameState {
    rng: ThreadRng,
    sprite_sheet: Image,
    orcs: Vec<Orc>,
    max_x: f32,
    max_y: f32,
    spawn_timer: i32,
}

impl GameState {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        let sprite_sheet = Image::from_file("examples/res/0x72_dungeon_ii.png")
            .expect("Could not load sprite sheet!");
        let mut orcs = Vec::with_capacity(INITIAL_ORCS);

        for _ in 0..INITIAL_ORCS {
            orcs.push(Orc::new(&mut rng));
        }

        Self {
            rng,
            sprite_sheet,
            orcs,
            max_x: (WIDTH - ORC_WIDTH) as f32,
            max_y: (HEIGHT - ORC_HEIGHT) as f32,
            spawn_timer: 0,
        }
    }
}

impl State for GameState {
    fn update(&mut self, _ctx: &mut Context) -> Result<()> {
        if self.spawn_timer > 0 {
            self.spawn_timer -= 1;
        }

        if Key::Space.is_pressed() && self.spawn_timer == 0 {
            for _ in 0..INITIAL_ORCS {
                self.orcs.push(Orc::new(&mut self.rng));
            }

            self.spawn_timer = 10;
        }

        for orc in &mut self.orcs {
            orc.position += orc.velocity;
            orc.velocity.y += GRAVITY;

            if orc.position.x > self.max_x {
                orc.velocity.x *= -1.0;
                orc.position.x = self.max_x;
            } else if orc.position.x < 0.0 {
                orc.velocity.x *= -1.0;
                orc.position.x = 0.0;
            }

            if orc.position.y > self.max_y {
                orc.velocity.y *= -0.8;
                orc.position.y = self.max_y;

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
            graphics::draw_image(
                ctx,
                &self.sprite_sheet,
                DrawImageParams {
                    position: orc.position,
                    clip_rect: Some(Rectangle::<i32>::new(372, 241, ORC_WIDTH, ORC_HEIGHT)),
                    ..Default::default()
                },
            );
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
        .build()?
        .run(&mut GameState::new())
}
