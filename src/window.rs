use crate::Context;

/// Sets the title of the window.
pub fn set_title(ctx: &mut Context, title: &str) {
    ctx.canvas.window_mut().set_title(title);
}

/// Sets whether the mouse cursor is visible in the window.
pub fn set_mouse_cursor_visible(ctx: &mut Context, visible: bool) {
    ctx.sdl_context.mouse().show_cursor(visible);
}
