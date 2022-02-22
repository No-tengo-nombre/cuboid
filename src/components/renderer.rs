use crate::core::traits::Drawable;
use gl;
use gl::types::*;

pub struct Renderer {
    _clear_color: [f32; 4],
}

impl Renderer {
    pub fn new() -> Renderer {
        return Renderer {
            _clear_color: [0.0, 0.0, 0.0, 1.0],
        };
    }

    pub fn clear(&self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }
    pub fn set_clear_color(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self._clear_color = [r, g, b, a];
        unsafe {
            gl::ClearColor(r, g, b, a);
        }
    }
    /// Draws the given shape using triangles.
    pub fn draw(&self, drawable: &dyn Drawable) {
        self.draw_mode(drawable, gl::TRIANGLES);
    }
    /// Draws the given shape using triangles.
    pub fn draw_mode(&self, drawable: &dyn Drawable, mode: GLenum) {
        drawable.get_drawn(mode);
    }
}
