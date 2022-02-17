use std::mem::{
    size_of,
};
use ogl33::{
    glGenBuffers,
    glBindBuffer,
    glBufferData,
    glDeleteBuffers,
    GLsizeiptr,
    GLenum,
    GLvoid,
    GL_ARRAY_BUFFER,
    GL_STATIC_DRAW,
};


pub struct VBO {
    _id: u32,
}


impl VBO {
    pub fn get_id(&self) -> u32 {
        return self._id;
    }

    pub fn bind(&self) {
        unsafe {
            glBindBuffer(GL_ARRAY_BUFFER, self._id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            glBindBuffer(GL_ARRAY_BUFFER, 0);
        }
    }

    pub fn del(&self) {
        unsafe {
            glDeleteBuffers(1, &self._id);
        }
    }
}


pub fn new<T>(vertices: &[T]) -> VBO {
    let mut vbo = 0;
    unsafe {
        // Generate the VBO
        glGenBuffers(1, &mut vbo);
        assert_ne!(vbo, 0);
        glBindBuffer(GL_ARRAY_BUFFER, vbo);

        // Buffer the vertices
        glBufferData(
            GL_ARRAY_BUFFER,
            (vertices.len() * size_of::<T>()) as GLsizeiptr,
            vertices.as_ptr() as *const GLvoid,
            GL_STATIC_DRAW,
        );
    }
    return VBO {_id: vbo};
}


pub fn new_with_usage<T>(vertices: &[T], usage: GLenum) -> VBO {
    let mut vbo = 0;
    unsafe {
        // Generate the VBO
        glGenBuffers(1, &mut vbo);
        assert_ne!(vbo, 0);
        glBindBuffer(GL_ARRAY_BUFFER, vbo);

        // Buffer the vertices
        glBufferData(
            GL_ARRAY_BUFFER,
            (vertices.len() * size_of::<T>()) as GLsizeiptr,
            vertices.as_ptr() as *const GLvoid,
            usage,
        );
    }
    return VBO {_id: vbo};
}
