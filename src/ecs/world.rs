use core::any::{Any, TypeId};
use std::collections::HashMap;
use std::iter::FromIterator;

use crate::ecs::{Allocator, Component, Entity, EntityBuilder};

pub(crate) struct World {
    pub(crate) entities: Vec<Entity>,
    components: HashMap<TypeId, Vec<Option<Box<dyn Any>>>>,
    pub(crate) allocator: Allocator,
}

impl World {
    pub(crate) const DEFAULT_STORAGE_CAPACITY: usize = 256;

    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            components: HashMap::new(),
            allocator: Allocator::new(),
        }
    }

    pub fn create_entity<'a>(&'a mut self) -> EntityBuilder<'a> {
        EntityBuilder::new(self)
    }

    pub fn kill_entity(&mut self, entity: Entity) {
        self.entities.retain(|e| *e != entity);

        let all_component_types = Vec::from_iter(self.components.keys().map(Clone::clone));
        for component_type in all_component_types {
            self.remove_component(entity, component_type);
        }

        self.allocator.kill(entity);
    }

    pub fn has_component<T: Component>(&self, entity: Entity) -> bool {
        self.get_component::<T>(entity).is_some()
    }

    pub fn get_component<T: Component>(&self, entity: Entity) -> Option<&T> {
        let component_type = TypeId::of::<T>();
        let components = self.components.get(&component_type)?;

        components[entity.id() as usize]
            .as_ref()
            .and_then(|component| component.downcast_ref::<T>())
    }

    pub fn get_component_mut<T: Component>(&mut self, entity: Entity) -> Option<&mut T> {
        let component_type = TypeId::of::<T>();
        let components = self.components.get_mut(&component_type)?;

        components[entity.id() as usize]
            .as_mut()
            .and_then(|component| component.downcast_mut::<T>())
    }

    pub fn attach_component(
        &mut self,
        entity: Entity,
        component_type: TypeId,
        component: Box<dyn Component>,
    ) {
        let components = self
            .components
            .entry(component_type)
            .or_insert(Vec::with_capacity(Self::DEFAULT_STORAGE_CAPACITY));

        if components.len() <= entity.id() as usize {
            components.resize_with(
                components.len() + Self::DEFAULT_STORAGE_CAPACITY,
                Default::default,
            );
        }

        components[entity.id() as usize] = Some(Box::new(component));
    }

    pub fn remove_component(&mut self, entity: Entity, component_type: TypeId) {
        if !self.components.contains_key(&component_type) {
            return;
        }

        let components = self.components.get_mut(&component_type).unwrap();
        if components.len() <= entity.id() as usize {
            return;
        }

        components[entity.id() as usize] = None;
    }
}
