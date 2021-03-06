use crate::opengl::Transform;
use gl::types::*;

pub trait Renderer {
    fn clear(&self);
    fn render(&self);
}

/// An object that defines how it should be drawn by the renderer
pub trait Drawable {
    fn draw_with_mode(&self, mode: GLenum);
    fn draw(&self);
}

pub trait Transformable {
    fn get_trans(&self) -> Transform;
    fn trans(&mut self, transform: &Transform) {
        self.get_trans().add(transform);
    }
}
