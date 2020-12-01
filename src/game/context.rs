use std::ops::Deref;
use std::ptr::NonNull;
use std::sync::Arc;

use crate::Result;

struct ContextAllocation<G> {
    data: Vec<u8>,
    game: G,
}

#[derive(Clone)]
pub struct LendedGame<G> {
    ptr: NonNull<G>,
    token: Arc<()>,
}

impl<G> Deref for LendedGame<G> {
    type Target = G;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr.as_ptr() }
    }
}

pub(crate) struct GameContext<G> {
    allocation: Box<ContextAllocation<G>>,
    lended_games: Arc<()>,
}

impl<G> GameContext<G> {
    pub fn new(game: G) -> Self {
        Self {
            allocation: Box::new(ContextAllocation {
                data: Vec::new(),
                game,
            }),
            lended_games: Arc::new(()),
        }
    }

    pub fn game(&self) -> LendedGame<G> {
        LendedGame {
            ptr: NonNull::from(&self.allocation.game),
            token: self.lended_games.clone(),
        }
    }

    pub fn game_mut(&mut self) -> &mut G {
        &mut self.allocation.game
    }

    pub fn collect(&self) -> Result<()> {
        assert!(Arc::strong_count(&self.lended_games) == 1);

        Ok(())
    }
}
