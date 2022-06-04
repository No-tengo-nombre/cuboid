use crate::assert_gl_is_loaded;
use gl;
use gl::types::*;
use std::mem::size_of;

/// An OpenGL Element Buffer Object.
#[derive(Copy, Clone)]
pub struct EBO<'a> {
    _id: u32,
    pub _count: u32,
    pub _indices: &'a [u32],
    pub _usage: GLenum,
}

impl<'a> EBO<'a> {
    pub fn new() -> EBO<'a> {
        return EBO {
            _id: 0,
            _count: 0,
            _indices: &[],
            _usage: gl::STATIC_DRAW,
        };
    }

    pub fn count(mut self, count: u32) -> EBO<'a> {
        self._count = count;
        return self;
    }

    pub fn indices(mut self, indices: &'a [u32]) -> EBO<'a> {
        self._indices = indices;
        return self;
    }

    pub fn usage(mut self, usage: GLenum) -> EBO<'a> {
        self._usage = usage;
        return self;
    }

    pub fn build(mut self) -> EBO<'a> {
        assert_gl_is_loaded();
        let mut ebo = 0;
        unsafe {
            // Generate the VBO
            gl::GenBuffers(1, &mut ebo);
            assert_ne!(ebo, 0);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);

            // Buffer the vertices
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (self._indices.len() * size_of::<u32>()) as GLsizeiptr,
                self._indices.as_ptr() as *const GLvoid,
                self._usage,
            );
        }
        self._id = ebo;
        return self;
    }

    pub fn bind(&self) {
        assert_gl_is_loaded();
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self._id);
        }
    }

    pub fn unbind(&self) {
        assert_gl_is_loaded();
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }
    }

    pub fn del(&self) {
        assert_gl_is_loaded();
        unsafe {
            gl::DeleteBuffers(1, &self._id);
        }
    }
}
