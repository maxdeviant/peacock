use crate::ecs::Generation;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Entity(Generation, i32);

impl Entity {
    pub(crate) fn new(generation: Generation, id: i32) -> Self {
        Self(generation, id)
    }

    pub(crate) fn generation(&self) -> Generation {
        self.0
    }

    pub(crate) fn id(&self) -> i32 {
        self.1
    }
}
