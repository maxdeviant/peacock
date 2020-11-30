use sdl2::rect::Rect as SdlRect;

use crate::graphics::View;
use crate::Context;

/// Sets the title of the window.
pub fn set_title<G>(ctx: &mut Context<G>, title: &str) {
    ctx.canvas
        .window_mut()
        .set_title(title)
        .expect("Failed to set window title");
}

/// Sets a new view for the window.
pub fn set_view<G>(ctx: &mut Context<G>, view: &View) {
    ctx.canvas.set_viewport(SdlRect::new(
        view.center.x as i32,
        view.center.y as i32,
        view.size.x as u32,
        view.size.y as u32,
    ));
    ctx.canvas
        .set_scale(view.zoom, view.zoom)
        .expect("Failed to set scale");
}

/// Sets whether the mouse cursor is visible in the window.
pub fn set_mouse_cursor_visible<G>(ctx: &mut Context<G>, visible: bool) {
    ctx.sdl_context.mouse().show_cursor(visible);
}
