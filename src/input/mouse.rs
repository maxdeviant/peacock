use crate::{Context, Vector2f};

/// Returns the position of the mouse.
pub fn position(ctx: &Context) -> Vector2f {
    ctx.mouse.position
}
