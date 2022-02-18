use gl;
use gl::types::*;


/// Wrapper for the glClearColor function.
pub fn set_clear_color(r: GLfloat, g: GLfloat, b: GLfloat, a: GLfloat) {
    unsafe {
        gl::ClearColor(r, g, b, a);
    }
}
