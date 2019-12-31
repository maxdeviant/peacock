pub mod mouse;

use hashbrown::HashSet;
use sdl2::event::Event;
use sdl2::keyboard::Keycode as SdlKeycode;

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

impl From<SdlKeycode> for Key {
    fn from(key: SdlKeycode) -> Self {
        match key {
            SdlKeycode::A => Key::A,
            SdlKeycode::B => Key::B,
            SdlKeycode::C => Key::C,
            SdlKeycode::D => Key::D,
            SdlKeycode::E => Key::E,
            SdlKeycode::F => Key::F,
            SdlKeycode::G => Key::G,
            SdlKeycode::H => Key::H,
            SdlKeycode::I => Key::I,
            SdlKeycode::J => Key::J,
            SdlKeycode::K => Key::K,
            SdlKeycode::L => Key::L,
            SdlKeycode::M => Key::M,
            SdlKeycode::N => Key::N,
            SdlKeycode::O => Key::O,
            SdlKeycode::P => Key::P,
            SdlKeycode::Q => Key::Q,
            SdlKeycode::R => Key::R,
            SdlKeycode::S => Key::S,
            SdlKeycode::T => Key::T,
            SdlKeycode::U => Key::U,
            SdlKeycode::V => Key::V,
            SdlKeycode::W => Key::W,
            SdlKeycode::X => Key::X,
            SdlKeycode::Y => Key::Y,
            SdlKeycode::Z => Key::Z,
            SdlKeycode::Num0 => Key::Num0,
            SdlKeycode::Num1 => Key::Num1,
            SdlKeycode::Num2 => Key::Num2,
            SdlKeycode::Num3 => Key::Num3,
            SdlKeycode::Num4 => Key::Num4,
            SdlKeycode::Num5 => Key::Num5,
            SdlKeycode::Num6 => Key::Num6,
            SdlKeycode::Num7 => Key::Num7,
            SdlKeycode::Num8 => Key::Num8,
            SdlKeycode::Num9 => Key::Num9,
            SdlKeycode::F1 => Key::F1,
            SdlKeycode::F2 => Key::F2,
            SdlKeycode::F3 => Key::F3,
            SdlKeycode::F4 => Key::F4,
            SdlKeycode::F5 => Key::F5,
            SdlKeycode::F6 => Key::F6,
            SdlKeycode::F7 => Key::F7,
            SdlKeycode::F8 => Key::F8,
            SdlKeycode::F9 => Key::F9,
            SdlKeycode::F10 => Key::F10,
            SdlKeycode::F11 => Key::F11,
            SdlKeycode::F12 => Key::F12,
            SdlKeycode::Left => Key::Left,
            SdlKeycode::Right => Key::Right,
            SdlKeycode::Up => Key::Up,
            SdlKeycode::Down => Key::Down,
            SdlKeycode::Space => Key::Space,
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
        Event::KeyDown { keycode, .. } => {
            if let Some(keycode) = keycode {
                ctx.keyboard.pressed_keys.insert(keycode.into());
            }
        }
        Event::KeyUp { keycode, .. } => {
            if let Some(keycode) = keycode {
                ctx.keyboard.pressed_keys.remove(&keycode.into());
            }
        }
        Event::MouseMotion { x, y, .. } => {
            ctx.mouse.position = Vector2f::new(x as f32, y as f32);
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
