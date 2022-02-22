use gl::types::*;

/// An object that defines how it should be drawn by the renderer
pub trait Drawable {
    fn get_drawn(&self, mode: GLenum);
}
