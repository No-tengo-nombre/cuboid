use gl;
use gl::types::*;
use std::mem::{
    size_of,
};


pub struct EBO {
    _id: u32,
}


pub fn new<T>(indices: &[T]) -> EBO {
    let mut ebo = 0;
    unsafe {
        // Generate the VBO
        gl::GenBuffers(1, &mut ebo);
        assert_ne!(ebo, 0);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);

        // Buffer the vertices
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (indices.len() * size_of::<T>()) as GLsizeiptr,
            indices.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW,
        );
    }
    return EBO {_id: ebo};
}


pub fn new_with_usage<T>(indices: &[T], usage: GLenum) -> EBO {
    let mut ebo = 0;
    unsafe {
        // Generate the VBO
        gl::GenBuffers(1, &mut ebo);
        assert_ne!(ebo, 0);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);

        // Buffer the vertices
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (indices.len() * size_of::<T>()) as GLsizeiptr,
            indices.as_ptr() as *const GLvoid,
            usage,
        );
    }
    return EBO {_id: ebo};
}
