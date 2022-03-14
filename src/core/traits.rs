use gl::types::*;
use crate::core::Transform;

/// An object that defines how it should be drawn by the renderer
pub trait Drawable {
    fn get_drawn(&self, mode: GLenum);
}

pub trait Transformable {
    fn apply_transform(&self, transform: &Transform) -> dyn Transformable;
}
