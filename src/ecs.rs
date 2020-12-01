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

use crate::PeacockContext;

pub trait Component: Send + Sync + 'static {}

pub fn create_entity(ctx: &mut PeacockContext) -> EntityBuilder {
    EntityBuilder::new(&mut ctx.world)
}

pub fn entities(ctx: &PeacockContext) -> Vec<Entity> {
    ctx.world.entities.clone()
}

pub fn has_component<T: Component>(ctx: &mut PeacockContext, entity: Entity) -> bool {
    ctx.world.has_component::<T>(entity)
}

pub fn get_component<T: Component>(ctx: &PeacockContext, entity: Entity) -> Option<&T> {
    ctx.world.get_component(entity)
}

pub fn get_component_mut<T: Component>(ctx: &mut PeacockContext, entity: Entity) -> Option<&mut T> {
    ctx.world.get_component_mut(entity)
}

pub fn delete_entity(ctx: &mut PeacockContext, entity: Entity) {
    unimplemented!()
}
