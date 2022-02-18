use gl;
use std::mem::{
    size_of,
};

use crate::core::buffers;
use crate::core::utils::{
    types,
};


pub struct VAO {
    _id: u32,
    _size: usize,
}


impl VAO {
    pub fn get_id(&self) -> u32 {
        return self._id;
    }

    pub fn get_size(&self) -> usize {
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

    pub fn link_vbo(&self, vbo: buffers::vbo::VBO, layout: u32) {
        vbo.bind();
        unsafe {
            gl::VertexAttribPointer(
                layout,
                3,
                gl::FLOAT,
                gl::FALSE,
                self._size.try_into().unwrap(),
                ((self._size as u32) * layout) as *const _,
            );
            gl::EnableVertexAttribArray(layout);
        }
        vbo.unbind();
    }
}


pub fn new() -> VAO {
    let mut vao = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        assert_ne!(vao, 0);
        gl::BindVertexArray(vao);
    }
    return VAO {_id: vao, _size: size_of::<types::Vertex>()};
}


pub fn new_sized(size: usize) -> VAO {
    let mut vao = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        assert_ne!(vao, 0);
        gl::BindVertexArray(vao);
    }
    return VAO {_id: vao, _size: size};
}
