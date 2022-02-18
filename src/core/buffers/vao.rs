use gl;
use std::mem::{
    size_of,
};

use crate::core::buffers;
use crate::core::utils::{
    types,
};


/// An OpenGL Vertex Array Object.
pub struct VAO {
    _id: u32,
    _stride: i32,
    _size: u32,
}


impl VAO {
    pub fn get_id(&self) -> u32 {
        return self._id;
    }

    pub fn get_stride(&self) -> i32 {
        return self._stride;
    }

    pub fn get_size(&self) -> u32 {
        return self._size;
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self._id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    pub fn del(&self) {
        unsafe {
            gl::DeleteBuffers(1, &self._id);
        }
    }

    pub fn link_vbo(&self, vbo: &buffers::vbo::VBO, layout: u32) {
        vbo.bind();
        unsafe {
            gl::VertexAttribPointer(
                layout,
                3,
                gl::FLOAT,
                gl::FALSE,
                self._stride,
                (self._size * layout) as *const _,
            );
            gl::EnableVertexAttribArray(layout);
        }
        vbo.unbind();
    }
}


/// Generates a new instance of a Vertex Array Object, asuming the size of the
/// data is that of `types::Vertex` (which is defined in this package).
pub fn new() -> VAO {
    return new_typed::<types::V6>(3);
}


pub fn new_typed<T>(size: u32) -> VAO {
    return new_sized(size_of::<T>(), size);
}


/// Generates a new instance of a Vertex Array Object, allowing the user to
/// specify the size of the data.
pub fn new_sized(stride: usize, size: u32) -> VAO {
    let mut vao = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        assert_ne!(vao, 0);
        gl::BindVertexArray(vao);
    }
    return VAO {_id: vao, _stride: stride as i32, _size: size};
}
