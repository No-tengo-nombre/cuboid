use crate::assert_gl_is_loaded;
use gl;
use gl::types::*;
use std::mem::size_of;

/// An OpenGL Element Buffer Object.
#[derive(Copy, Clone)]
pub struct EBO<'a, T> {
    _id: u32,
    pub count: u32,
    pub indices: &'a [T],
    pub usage: GLenum,
}

impl<'a, T> EBO<'a, T> {
    pub fn new() -> EBO<'a, T> {
        return EBO {
            _id: 0,
            count: 0,
            indices: &[],
            usage: gl::STATIC_DRAW,
        };
    }

    pub fn count(mut self, count: u32) -> EBO<'a, T> {
        self.count = count;
        return self;
    }

    pub fn indices(mut self, indices: &'a [T]) -> EBO<'a, T> {
        self.indices = indices;
        return self;
    }

    pub fn usage(mut self, usage: GLenum) -> EBO<'a, T> {
        self.usage = usage;
        return self;
    }

    pub fn build(mut self) -> EBO<'a, T> {
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
                (self.indices.len() * size_of::<T>()) as GLsizeiptr,
                self.indices.as_ptr() as *const GLvoid,
                self.usage,
            );
        }
        self._id = ebo;
        return self;
    }

    /// Generates a new instance of an Element Buffer Object containing the given
    /// indices, asuming the usage `GL_STATIC_DRAW`.
    pub fn new_ebo(indices: &'a [T], count: u32) -> EBO<'a, T> {
        return EBO::new().count(count).indices(indices).build();
        // return EBO::new_with_usage(indices, count, gl::STATIC_DRAW);
    }

    /// Generates a new instance of an Element Buffer Object containing the given
    /// indices, allowing the user to specify the usage.
    // pub fn new_with_usage<T>(indices: &[T], count: u32, usage: GLenum) -> EBO {
    //     assert_gl_is_loaded();
    //     let mut ebo = 0;
    //     unsafe {
    //         // Generate the VBO
    //         gl::GenBuffers(1, &mut ebo);
    //         assert_ne!(ebo, 0);
    //         gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);

    //         // Buffer the vertices
    //         gl::BufferData(
    //             gl::ELEMENT_ARRAY_BUFFER,
    //             (indices.len() * size_of::<T>()) as GLsizeiptr,
    //             indices.as_ptr() as *const GLvoid,
    //             usage,
    //         );
    //     }
    //     return EBO {
    //         _id: ebo,
    //         _count: count,
    //     };
    // }

    // pub fn get_count(&self) -> u32 {
    //     return self._count;
    // }

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
