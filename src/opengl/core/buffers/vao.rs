use crate::opengl::assert_gl_is_loaded;
use crate::opengl::buffers::VBO;
use gl;
use std::mem::size_of;

/// An OpenGL Vertex Array Object.
#[derive(Copy, Clone)]
pub struct VAO<'a> {
    _id: u32,
    pub _stride: u32,
    pub _size: u32,
    pub _sizes: &'a [u32],
}

impl<'a> VAO<'a> {
    pub fn new() -> VAO<'a> {
        return VAO {
            _id: 0,
            _stride: 0,
            _size: size_of::<f32>() as u32,
            _sizes: &[],
        };
    }

    pub fn stride(mut self, stride: u32) -> VAO<'a> {
        self._stride = stride;
        return self;
    }

    pub fn stride_from_type<T>(mut self) -> VAO<'a> {
        self._stride = size_of::<T>() as u32;
        return self;
    }

    pub fn size(mut self, size: u32) -> VAO<'a> {
        self._size = size;
        return self;
    }

    pub fn sizes(mut self, sizes: &'a [u32]) -> VAO<'a> {
        self._sizes = sizes;
        return self;
    }

    pub fn build(mut self) -> VAO<'a> {
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
            if *self._sizes == [] {
                gl::VertexAttribPointer(
                    layout,
                    3,
                    gl::FLOAT,
                    gl::FALSE,
                    self._stride.try_into().unwrap(),
                    (3 * self._size * layout) as *const _,
                );
            } else {
                gl::VertexAttribPointer(
                    layout,
                    self._sizes[layout as usize].try_into().unwrap(),
                    gl::FLOAT,
                    gl::FALSE,
                    self._stride.try_into().unwrap(),
                    (self._sizes[layout as usize] * self._size * layout) as *const _,
                );
            }
            gl::EnableVertexAttribArray(layout);
        }
        vbo.unbind();
    }
}
