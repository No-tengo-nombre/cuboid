use ogl33::{
    glGenVertexArrays,
    glBindVertexArray,
    glDeleteBuffers,
    glVertexAttribPointer,
    glEnableVertexAttribArray,
    GL_FLOAT,
    GL_FALSE,
};
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
            glBindVertexArray(self._id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            glBindVertexArray(0);
        }
    }

    pub fn del(&self) {
        unsafe {
            glDeleteBuffers(1, &self._id);
        }
    }

    pub fn link_vbo(&self, vbo: buffers::vbo::VBO, layout: u32) {
        vbo.bind();
        unsafe {
            glVertexAttribPointer(
                layout,
                3,
                GL_FLOAT,
                GL_FALSE,
                self._size.try_into().unwrap(),
                ((self._size as u32) * layout) as *const _,
            );
            glEnableVertexAttribArray(layout);
        }
        vbo.unbind();
    }
}


pub fn new() -> VAO {
    let mut vao = 0;
    unsafe {
        glGenVertexArrays(1, &mut vao);
        assert_ne!(vao, 0);
        glBindVertexArray(vao);
    }
    return VAO {_id: vao, _size: size_of::<types::Vertex>()};
}


pub fn new_sized(size: usize) -> VAO {
    let mut vao = 0;
    unsafe {
        glGenVertexArrays(1, &mut vao);
        assert_ne!(vao, 0);
        glBindVertexArray(vao);
    }
    return VAO {_id: vao, _size: size};
}
