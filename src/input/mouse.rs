use crate::{PeacockContext, Vector2f};

/// Returns the position of the mouse.
pub fn position(ctx: &PeacockContext) -> Vector2f {
    ctx.mouse.position
}
