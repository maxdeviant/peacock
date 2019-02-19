use std::collections::HashSet;

use sfml::window::{Event as SfEvent, Key as SfKey};

use crate::{Context, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Key {
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

    /// The `Space` key.
    Space,
    // TODO: Finish adding keys.
}

impl From<SfKey> for Key {
    fn from(key: SfKey) -> Self {
        match key {
            SfKey::A => Key::A,
            SfKey::B => Key::B,
            SfKey::C => Key::C,
            SfKey::D => Key::D,
            SfKey::E => Key::E,
            SfKey::F => Key::F,
            SfKey::G => Key::G,
            SfKey::H => Key::H,
            SfKey::I => Key::I,
            SfKey::J => Key::J,
            SfKey::K => Key::K,
            SfKey::L => Key::L,
            SfKey::M => Key::M,
            SfKey::N => Key::N,
            SfKey::O => Key::O,
            SfKey::P => Key::P,
            SfKey::Q => Key::Q,
            SfKey::R => Key::R,
            SfKey::S => Key::S,
            SfKey::T => Key::T,
            SfKey::U => Key::U,
            SfKey::V => Key::V,
            SfKey::W => Key::W,
            SfKey::X => Key::X,
            SfKey::Y => Key::Y,
            SfKey::Z => Key::Z,
            SfKey::Num0 => Key::Num0,
            SfKey::Num1 => Key::Num1,
            SfKey::Num2 => Key::Num2,
            SfKey::Num3 => Key::Num3,
            SfKey::Num4 => Key::Num4,
            SfKey::Num5 => Key::Num5,
            SfKey::Num6 => Key::Num6,
            SfKey::Num7 => Key::Num7,
            SfKey::Num8 => Key::Num8,
            SfKey::Num9 => Key::Num9,
            SfKey::Space => Key::Space,
            _ => unreachable!(),
        }
    }
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

pub(crate) fn handle_event(ctx: &mut Context, event: SfEvent) -> Result<()> {
    match event {
        SfEvent::KeyPressed { code: key, .. } => {
            ctx.keyboard.pressed_keys.insert(key.into());
        }
        SfEvent::KeyReleased { code: key, .. } => {
            ctx.keyboard.pressed_keys.remove(&key.into());
        }
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
