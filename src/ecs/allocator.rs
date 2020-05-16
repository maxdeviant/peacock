use std::collections::VecDeque;

use crate::ecs::{Entity, Generation, ZeroableGeneration};

pub(crate) struct Allocator {
    generations: Vec<Option<Generation>>,
    cache: VecDeque<i32>,
    next_id: i32,
}

impl Allocator {
    pub fn new() -> Self {
        Self {
            generations: Vec::new(),
            cache: VecDeque::new(),
            next_id: 0,
        }
    }

    pub fn allocate(&mut self) -> Entity {
        let id = self.cache.pop_front().unwrap_or_else(|| {
            self.next_id += 1;
            self.next_id
        });

        self.update_generation_length(id as usize);
        let generation = self.generations[id as usize].raise();
        self.generations[id as usize] = Some(generation);

        Entity::new(generation, id)
    }

    pub fn is_alive(&self, entity: Entity) -> bool {
        entity.generation() == self.get_generation(entity)
    }

    pub fn kill(&mut self, entity: Entity) {
        self.kill_many(std::iter::once(entity));
    }

    pub fn kill_many<I: IntoIterator<Item = Entity>>(&mut self, entities: I) {
        for entity in entities {
            if !self.is_alive(entity) {
                panic!("Wrong generation")
            }

            let id = entity.id();

            self.update_generation_length(id as usize);

            self.generations[id as usize] = Some(self.generations[id as usize].die());

            self.cache.push_back(id);
        }
    }

    fn get_generation(&self, entity: Entity) -> Generation {
        if self.generations.len() <= entity.id() as usize {
            return Generation::one();
        }

        let generation = self.generations[entity.id() as usize];
        generation.unwrap_or_else(Generation::one)
    }

    fn update_generation_length(&mut self, desired_length: usize) {
        if self.generations.len() <= desired_length {
            self.generations.resize(desired_length + 1, None);
        }
    }
}
