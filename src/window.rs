use sfml::graphics::{RenderTarget as SfRenderTarget, View as SfView};

use crate::graphics::View;
use crate::Context;

/// Sets the title of the window.
pub fn set_title(ctx: &mut Context, title: &str) {
    ctx.window.set_title(title)
}

/// Sets a new view for the window.
pub fn set_view(ctx: &mut Context, view: &View) {
    let view: SfView = view.into();
    ctx.window.set_view(&view);
}

/// Sets whether the mouse cursor is visible in the window.
pub fn set_mouse_cursor_visible(ctx: &mut Context, visible: bool) {
    ctx.window.set_mouse_cursor_visible(visible);
}
