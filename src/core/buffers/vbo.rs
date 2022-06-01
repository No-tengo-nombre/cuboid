use crate::assert_gl_is_loaded;
use gl;
use gl::types::*;
use std::mem::size_of;

/// An OpenGL Vertex Buffer Object.
#[derive(Copy, Clone)]
pub struct VBO {
    _id: u32,
}

impl VBO {
    /// Generates a new instance of a Vertex Buffer Object containing the given
    /// vertices, with the usage `GL_STATIC_DRAW`.
    pub fn new<T>(vertices: &[T]) -> VBO {
        return VBO::new_with_usage(vertices, gl::STATIC_DRAW);
    }

    /// Generates a new instance of a Vertex Buffer Object containing the given
    /// vertices, allowing the user to specify the usage.
    pub fn new_with_usage<T>(vertices: &[T], usage: GLenum) -> VBO {
        assert_gl_is_loaded();
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
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
        return VBO { _id: vbo };
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
