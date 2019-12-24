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
