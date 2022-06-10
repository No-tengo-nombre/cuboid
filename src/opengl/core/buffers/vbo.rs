use crate::opengl::assert_gl_is_loaded;
use gl;
use gl::types::*;
use std::mem::size_of;

/// An OpenGL Vertex Buffer Object.
#[derive(Copy, Clone)]
pub struct VBO<'a, T> {
    _id: u32,
    pub _vertices: &'a [T],
    pub _usage: GLenum,
}

impl<'a, T> VBO<'a, T> {
    pub fn new() -> VBO<'a, T> {
        return VBO {
            _id: 0,
            _vertices: &[],
            _usage: gl::STATIC_DRAW,
        };
    }

    pub fn vertices(mut self, vertices: &'a [T]) -> VBO<'a, T> {
        self._vertices = vertices;
        return self;
    }

    pub fn usage(mut self, usage: GLenum) -> VBO<'a, T> {
        self._usage = usage;
        return self;
    }

    /// Generates a new instance of a Vertex Buffer Object containing the given
    /// vertices, allowing the user to specify the usage.
    pub fn build(mut self) -> VBO<'a, T> {
        assert_gl_is_loaded();
        let mut vbo = 0;
        unsafe {
            // Generate the VBO
            gl::GenBuffers(1, &mut vbo);
            assert_ne!(vbo, 0);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            // Buffer the vertices
            self.buffer_data(self._vertices);

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
        self._id = vbo;
        return self;
    }

    pub fn buffer_data(&self, vertices: &'a [T]) {
        assert_gl_is_loaded();
        unsafe {
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * size_of::<T>()) as GLsizeiptr,
                vertices.as_ptr() as *const GLvoid,
                self._usage,
            );
        }
    }

    pub fn bind(&self) {
        assert_gl_is_loaded();
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self._id);
        }
    }

    pub fn unbind(&self) {
        assert_gl_is_loaded();
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }
}
