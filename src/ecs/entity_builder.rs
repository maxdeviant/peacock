use std::any::{Any, TypeId};
use std::collections::HashMap;

use crate::ecs::{Component, Entity, World};

pub struct EntityBuilder<'a> {
    world: &'a mut World,
    components: HashMap<TypeId, Box<dyn Any>>,
}

impl<'a> EntityBuilder<'a> {
    pub(crate) fn new(world: &'a mut World) -> Self {
        Self {
            world,
            components: HashMap::new(),
        }
    }

    pub fn with<T: Component>(mut self, component: T) -> Self {
        self.components
            .insert(TypeId::of::<T>(), Box::new(component));
        self
    }

    pub fn build(self) -> Entity {
        let entity = self.world.allocator.allocate();

        for (component_type, component) in self.components {
            self.world
                .attach_component(entity, component_type, component);
        }

        self.world.entities.push(entity);

        entity
    }
}
