use glam::{Mat4, Vec2, Vec3};

use crate::graphics::Transform;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Camera2D {
    pub transform: Transform,
    pub zoom:      f32,
    pub viewport:  Vec2,
    pub near:      f32,
    pub far:       f32,
}

impl Camera2D {
    pub fn new(transform: Transform, zoom: f32, viewport: Vec2, near: f32, far: f32) -> Self {
        Self { transform, zoom, viewport, near, far }
    }

    pub fn move_by(&mut self, delta: Vec2) {
        self.transform.translation += delta.extend(0.0);
    }

    pub fn move_vertically(&mut self, delta: f32) {
        self.transform.translation.y += delta;
    }

    pub fn move_horizontally(&mut self, delta: f32) {
        self.transform.translation.x += delta;
    }

    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self
    }

    pub fn with_zoom(mut self, zoom: f32) -> Self {
        self.zoom = zoom;
        self
    }

    pub fn with_viewport(mut self, viewport: Vec2) -> Self {
        self.viewport = viewport;
        self
    }

    pub fn projection_matrix(&self) -> Mat4 {
        let position = self.transform.translation;
        let half_w = self.viewport.x * 0.5 / self.zoom;
        let half_h = self.viewport.y * 0.5 / self.zoom;
        Mat4::orthographic_rh_gl(
            -half_w + position.x,
            half_w + position.x,
            -half_h + position.y,
            half_h + position.y,
            self.near,
            self.far,
        )
    }

    pub fn unproject(&self, ndc: Vec2) -> Vec3 {
        let half_w = self.viewport.x * 0.5 / self.zoom;
        let half_h = self.viewport.y * 0.5 / self.zoom;

        let x = ndc.x * half_w + self.transform.translation.x;
        let y = ndc.y * half_h + self.transform.translation.y;

        Vec3::new(x, y, 0.0)
    }
}

impl Default for Camera2D {
    fn default() -> Self {
        Self {
            transform: Transform::IDENTITY,
            zoom:      1.0,
            viewport:  Vec2::new(0.0, 0.0),
            near:      -1000.0,
            far:       1000.0,
        }
    }
}
