use crate::Context;

/// Sets the title of the window.
pub fn set_title(ctx: &mut Context, title: &str) {
    ctx.window.set_title(title)
}
