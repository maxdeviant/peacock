pub mod mouse;

use hashbrown::HashSet;
use sdl2::event::Event;

use crate::{Context, Result, Vector2f};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Key {
    /// An unknown key.
    Unknown,

    /// The `A` key.
    A,

    /// The `B` key.
    B,

    /// The `C` key.
    C,

    /// The `D` key.
    D,

    /// The `E` key.
    E,

    /// The `F` key.
    F,

    /// The `G` key.
    G,

    /// The `H` key.
    H,

    /// The `I` key.
    I,

    /// The `J` key.
    J,

    /// The `K` key.
    K,

    /// The `L` key.
    L,

    /// The `M` key.
    M,

    /// The `N` key.
    N,

    /// The `O` key.
    O,

    /// The `P` key.
    P,

    /// The `Q` key.
    Q,

    /// The `R` key.
    R,

    /// The `S` key.
    S,

    /// The `T` key.
    T,

    /// The `U` key.
    U,

    /// The `V` key.
    V,

    /// The `W` key.
    W,

    /// The `X` key.
    X,

    /// The `Y` key.
    Y,

    /// The `Z` key.
    Z,

    /// The `0` key.
    Num0,

    /// The `1` key.
    Num1,

    /// The `2` key.
    Num2,

    /// The `3` key.
    Num3,

    /// The `4` key.
    Num4,

    /// The `5` key.
    Num5,

    /// The `6` key.
    Num6,

    /// The `7` key.
    Num7,

    /// The `8` key.
    Num8,

    /// The `9` key.
    Num9,

    /// The `F1` key.
    F1,

    /// The `F2` key.
    F2,

    /// The `F3` key.
    F3,

    /// The `F4` key.
    F4,

    /// The `F5` key.
    F5,

    /// The `F6` key.
    F6,

    /// The `F7` key.
    F7,

    /// The `F8` key.
    F8,

    /// The `F9` key.
    F9,

    /// The `F10` key.
    F10,

    /// The `F11` key.
    F11,

    /// The `F12` key.
    F12,

    /// The left arrow.
    Left,

    /// The right arrow.
    Right,

    /// The up arrow.
    Up,

    /// The down arrow.
    Down,

    /// The `Space` key.
    Space,
}

pub(crate) struct KeyboardContext {
    last_pressed_keys: HashSet<Key>,
    pressed_keys: HashSet<Key>,
}

impl KeyboardContext {
    pub(crate) fn new() -> Self {
        Self {
            last_pressed_keys: HashSet::with_capacity(256),
            pressed_keys: HashSet::with_capacity(256),
        }
    }
}

pub(crate) struct MouseContext {
    position: Vector2f,
}

impl MouseContext {
    pub(crate) fn new() -> Self {
        Self {
            position: Vector2f::ZERO,
        }
    }
}

pub(crate) fn handle_event(ctx: &mut Context, event: Event) -> Result<()> {
    match event {
        _ => {}
    };

    Ok(())
}

pub(crate) fn cleanup_after_state_update(ctx: &mut Context) {
    ctx.keyboard.last_pressed_keys = ctx.keyboard.pressed_keys.clone();
}

pub fn is_key_down(ctx: &Context, key: Key) -> bool {
    ctx.keyboard.pressed_keys.contains(&key)
}

pub fn is_key_up(ctx: &Context, key: Key) -> bool {
    !ctx.keyboard.pressed_keys.contains(&key)
}

pub fn was_key_pressed(ctx: &Context, key: Key) -> bool {
    !ctx.keyboard.last_pressed_keys.contains(&key) && is_key_down(ctx, key)
}

pub fn was_key_released(ctx: &Context, key: Key) -> bool {
    ctx.keyboard.last_pressed_keys.contains(&key) && is_key_up(ctx, key)
}
