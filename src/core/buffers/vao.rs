use crate::assert_gl_is_loaded;
use crate::buffers::{VBO};
use gl;
use std::mem::size_of;

/// An OpenGL Vertex Array Object.
#[derive(Copy, Clone)]
pub struct VAO {
    _id: u32,
    pub stride: u32,
    pub size: u32,
}

impl VAO {
    pub fn new() -> VAO {
        return VAO {
            _id: 0,
            stride: 0,
            size: 0,
        };
    }

    pub fn stride(mut self, stride: u32) -> VAO {
        self.stride = stride;
        return self;
    }

    pub fn stride_from_type<T>(mut self) -> VAO {
        self.stride = size_of::<T>() as u32;
        return self;
    }

    pub fn size(mut self, size: u32) -> VAO {
        self.size = size;
        return self;
    }

    pub fn build(mut self) -> VAO {
        assert_gl_is_loaded();
        let mut vao = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            assert_ne!(vao, 0);
            gl::BindVertexArray(vao);
        }
        self._id = vao;
        return self;
    }

    pub fn new_typed<T>(size: u32) -> VAO {
        return VAO::new_sized(size_of::<T>(), size);
    }

    /// Generates a new instance of a Vertex Array Object, allowing the user to
    /// specify the size of the data.
    pub fn new_sized(stride: usize, size: u32) -> VAO {
        assert_gl_is_loaded();
        let mut vao = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            assert_ne!(vao, 0);
            gl::BindVertexArray(vao);
        }
        return VAO {
            _id: vao,
            stride: stride as u32,
            size: size,
        };
    }

    pub fn bind(&self) {
        assert_gl_is_loaded();
        unsafe {
            gl::BindVertexArray(self._id);
        }
    }

    pub fn unbind(&self) {
        assert_gl_is_loaded();
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    pub fn del(&self) {
        assert_gl_is_loaded();
        unsafe {
            gl::DeleteBuffers(1, &self._id);
        }
    }

    pub fn link_vbo<T>(&self, vbo: &VBO<T>, layout: u32) {
        assert_gl_is_loaded();
        vbo.bind();
        unsafe {
            gl::VertexAttribPointer(
                layout,
                3,
                gl::FLOAT,
                gl::FALSE,
                self.stride.try_into().unwrap(),
                (self.size * layout) as *const _,
            );
            gl::EnableVertexAttribArray(layout);
        }
        vbo.unbind();
    }
}
