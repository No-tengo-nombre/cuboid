use ogl33::{
    glClearColor,
    GLfloat,
};


pub fn set_clear_color(r: GLfloat, g: GLfloat, b: GLfloat, a: GLfloat) {
    unsafe {
        glClearColor(r, g, b, a);
    }
}
