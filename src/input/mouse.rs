use crate::{Context, Vector2f};

/// Returns the position of the mouse.
pub fn position<G>(ctx: &Context<G>) -> Vector2f {
    ctx.mouse.position
}
