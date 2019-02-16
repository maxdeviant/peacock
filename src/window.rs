use crate::graphics::ViewRef;
use crate::Context;

use sfml::graphics::RenderTarget;

/// Sets the title of the window.
pub fn set_title(ctx: &mut Context, title: &str) {
    ctx.window.set_title(title)
}

/// Sets a new view for the window.
pub fn set_view(ctx: &mut Context, view: &ViewRef) {
    ctx.window.set_view(view);
}
