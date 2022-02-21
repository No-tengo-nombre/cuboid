use gl;
use gl::types::*;
use crate::components;


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
pub fn draw(shape: &components::shape::Shape) {
    draw_mode(shape, gl::TRIANGLES);
}


/// Draws the given shape using triangles.
pub fn draw_mode(shape: &components::shape::Shape, mode: GLenum) {
    shape.use_material();
    shape.bind_vao();
    shape.bind_ebo();
    unsafe {
        gl::DrawElements(
            mode,
            shape.get_ebo_count().try_into().unwrap(),
            gl::UNSIGNED_INT,
            0 as *const _,
        );
    }
}
