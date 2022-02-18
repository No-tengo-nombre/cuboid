use std::mem::{
    size_of,
};
use gl;
use gl::types::*;


pub struct VBO {
    _id: u32,
}


impl VBO {
    pub fn get_id(&self) -> u32 {
        return self._id;
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self._id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    pub fn del(&self) {
        unsafe {
            gl::DeleteBuffers(1, &self._id);
        }
    }
}


pub fn new<T>(vertices: &[T]) -> VBO {
    let mut vbo = 0;
    unsafe {
        // Generate the VBO
        gl::GenBuffers(1, &mut vbo);
        assert_ne!(vbo, 0);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        // Buffer the vertices
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * size_of::<T>()) as GLsizeiptr,
            vertices.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW,
        );
    }
    return VBO {_id: vbo};
}


pub fn new_with_usage<T>(vertices: &[T], usage: GLenum) -> VBO {
    let mut vbo = 0;
    unsafe {
        // Generate the VBO
        gl::GenBuffers(1, &mut vbo);
        assert_ne!(vbo, 0);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        // Buffer the vertices
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * size_of::<T>()) as GLsizeiptr,
            vertices.as_ptr() as *const GLvoid,
            usage,
        );
    }
    return VBO {_id: vbo};
}
