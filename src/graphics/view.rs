use sfml::graphics::View as SfView;

use crate::Vector2f;

#[derive(Debug)]
pub struct View {
    center: Vector2f,
    size: Vector2f,
    rotation: f32,
    zoom: f32,
}

impl View {
    pub fn new(center: Vector2f, size: Vector2f) -> Self {
        Self {
            center,
            size,
            rotation: 0.0,
            zoom: 1.0,
        }
    }

    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
    }

    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom;
    }
}

impl From<&View> for SfView {
    fn from(view: &View) -> Self {
        let mut sf_view = SfView::default();
        sf_view.set_center(view.center);
        sf_view.set_size(view.size);
        sf_view.set_rotation(view.rotation);
        sf_view.zoom(view.zoom);
        sf_view
    }
}
