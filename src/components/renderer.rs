use gl;
use gl::types::*;
// use crate::components;


pub trait Drawable {
    fn get_drawn(&self, mode: GLenum);
}


pub fn clear() {
    unsafe {
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
}


pub fn set_clear_color(r: f32, g: f32, b: f32, a: f32) {
    unsafe {
        gl::ClearColor(r, g, b, a);
    }
}


/// Draws the given shape using triangles.
pub fn draw(drawable: &dyn Drawable) {
    draw_mode(drawable, gl::TRIANGLES);
}


/// Draws the given shape using triangles.
pub fn draw_mode(drawable: &dyn Drawable, mode: GLenum) {
    drawable.get_drawn(mode);

}
