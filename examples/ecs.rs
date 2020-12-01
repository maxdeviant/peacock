use peacock::ecs::{self, Component, Entity};
use peacock::graphics::{self, DrawImageParams, Image, Rectangle};
use peacock::input::{self, Key};
use peacock::Result;
use peacock::{ContextBuilder, State, Vector2f};

type Context<'ctx> = peacock::ContextArgs<'ctx, ()>;

#[derive(Debug)]
struct Transform {
    pub position: Vector2f,
}

impl Component for Transform {}

#[derive(Debug)]
struct StaticSprite {
    pub source: Rectangle<i32>,
}

impl Component for StaticSprite {}

struct EcsExample {
    sprite_sheet: Image,
    player: Entity,
}

impl EcsExample {
    fn new(ctx: Context) -> Result<Self> {
        let sprite_sheet = Image::from_file(ctx.ctx, "examples/res/0x72_dungeon_ii.png")?;

        let player = ecs::create_entity(ctx.ctx)
            .with(Transform {
                position: Vector2f::new(0.0, 0.0),
            })
            .with(StaticSprite {
                source: Rectangle::<i32>::new(128, 76, 15, 20),
            })
            .build();

        Ok(Self {
            sprite_sheet,
            player,
        })
    }
}

impl State for EcsExample {
    type Context = ();
    fn update(&mut self, ctx: Context) -> Result<()> {
        let direction = {
            let mut direction = Vector2f::ZERO;

            if input::is_key_down(ctx.ctx, Key::A) {
                direction += -Vector2f::UNIT_X
            }

            if input::is_key_down(ctx.ctx, Key::D) {
                direction += Vector2f::UNIT_X;
            }

            if input::is_key_down(ctx.ctx, Key::W) {
                direction += -Vector2f::UNIT_Y;
            }

            if input::is_key_down(ctx.ctx, Key::S) {
                direction += Vector2f::UNIT_Y;
            }

            if direction == Vector2f::ZERO {
                direction
            } else {
                direction.normalize()
            }
        };

        if let Some(transform) = ecs::get_component_mut::<Transform>(ctx.ctx, self.player) {
            let speed = 10.0;

            transform.position += direction * speed;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: Context, _dt: f64) -> Result<()> {
        for entity in ecs::entities(ctx.ctx) {
            let transform = ecs::get_component::<Transform>(ctx.ctx, entity);
            let static_sprite = ecs::get_component::<StaticSprite>(ctx.ctx, entity);

            match (transform, static_sprite) {
                (Some(transform), Some(static_sprite)) => {
                    let draw_params = DrawImageParams {
                        position: transform.position,
                        clip_rect: Some(static_sprite.source),
                        scale: Some(Vector2f::new(8.0, 8.0)),
                        ..Default::default()
                    };

                    graphics::draw(ctx, &self.sprite_sheet, &draw_params)?;
                }
                _ => {}
            };
        }

        Ok(())
    }
}

fn main() -> Result<()> {
    ContextBuilder::new("Entity Component System", 1920, 1080)
        .build_empty()?
        .run_with_result(EcsExample::new)
}
