use crate::assert_gl_is_loaded;
use gl;
use gl::types::*;
use std::mem::size_of;

pub struct UBO {
    _id: u32,
}

impl UBO {
    pub fn new(size: u32) -> UBO {
        return UBO::new_with_usage(size, gl::STREAM_DRAW);
    }

    pub fn new_with_usage(size: u32, usage: GLenum) -> UBO {
        assert_gl_is_loaded();
        let mut ubo = 0;
        unsafe {
            // Generate the UBO
            gl::GenBuffers(1, &mut ubo);
            assert_ne!(ubo, 0);
            gl::BindBuffer(gl::UNIFORM_BUFFER, ubo);

            // Buffer empty data at the beginning
            gl::BufferData(
                gl::UNIFORM_BUFFER,
                size as GLsizeiptr,
                0 as *const GLvoid,
                usage,
            );

            gl::BindBuffer(gl::UNIFORM_BUFFER, 0);
        }
        return UBO { _id: ubo };
    }

    /// Binds the UBO
    pub fn bind(&self) {
        assert_gl_is_loaded();
        unsafe {
            gl::BindBuffer(gl::UNIFORM_BUFFER, self._id);
        }
    }

    /// Uninds the UBO
    pub fn unbind(&self) {
        assert_gl_is_loaded();
        unsafe {
            gl::BindBuffer(gl::UNIFORM_BUFFER, 0);
        }
    }

    /// Binds the UBO to the specified index in memory. It's important to note that, by default,
    /// the index 0 is reserved for the camera.
    pub fn bind_index(&self, index: u32) {
        assert_gl_is_loaded();
        unsafe {
            gl::BindBufferBase(gl::UNIFORM_BUFFER, index, self._id);
        }
    }

    /// Buffers the given data with a certain offset.
    pub fn buffer_data<T>(&self, offset: u32, data: &[T]) {
        assert_gl_is_loaded();
        unsafe {
            self.bind();
            gl::BufferSubData(
                gl::UNIFORM_BUFFER,
                offset.try_into().unwrap(),
                (data.len() * size_of::<T>()) as GLsizeiptr,
                data.as_ptr() as *const GLvoid,
            );
            self.unbind();
        }
    }
}
