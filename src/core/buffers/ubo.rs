use crate::assert_gl_is_loaded;
use gl;
use gl::types::*;
use std::mem::size_of;

#[derive(Copy, Clone)]
pub struct UBO {
    _id: u32,
    pub size: u32,
    pub usage: GLenum,
}

impl UBO {
    pub fn new() -> UBO {
        return UBO {
            _id: 0,
            size: 0,
            usage: gl::STREAM_DRAW,
        };
    }

    pub fn size(mut self, size: u32) -> UBO {
        self.size = size;
        return self;
    }

    pub fn usage(mut self, usage: GLenum) -> UBO {
        self.usage = usage;
        return self;
    }

    pub fn build(mut self) -> UBO {
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
                self.size as GLsizeiptr,
                0 as *const GLvoid,
                self.usage,
            );

            gl::BindBuffer(gl::UNIFORM_BUFFER, 0);
        }

        self._id = ubo;
        return self;
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
