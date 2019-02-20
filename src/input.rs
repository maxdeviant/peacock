use hashbrown::HashSet;
use sfml::window::{Event as SfEvent, Key as SfKey};

use crate::{Context, Result};

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
            SfKey::F1 => Key::F1,
            SfKey::F2 => Key::F2,
            SfKey::F3 => Key::F3,
            SfKey::F4 => Key::F4,
            SfKey::F5 => Key::F5,
            SfKey::F6 => Key::F6,
            SfKey::F7 => Key::F7,
            SfKey::F8 => Key::F8,
            SfKey::F9 => Key::F9,
            SfKey::F10 => Key::F10,
            SfKey::F11 => Key::F11,
            SfKey::F12 => Key::F12,
            SfKey::Left => Key::Left,
            SfKey::Right => Key::Right,
            SfKey::Up => Key::Up,
            SfKey::Down => Key::Down,
            SfKey::Space => Key::Space,
            _ => Key::Unknown,
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
            last_pressed_keys: HashSet::with_capacity(16),
            pressed_keys: HashSet::with_capacity(16),
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
