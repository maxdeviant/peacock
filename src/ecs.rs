mod allocator;
mod entity;
mod entity_builder;
mod generation;
mod world;

pub(crate) use self::allocator::*;
pub use self::entity::*;
pub use self::entity_builder::*;
pub(crate) use self::generation::*;
pub(crate) use self::world::*;

use crate::Context;

pub trait Component: Send + Sync + 'static {}

pub fn create_entity(ctx: &mut Context) -> EntityBuilder {
    EntityBuilder::new(&mut ctx.world)
}

pub fn delete_entity(ctx: &mut Context, entity: Entity) {
    unimplemented!()
}
