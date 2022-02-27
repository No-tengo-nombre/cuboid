use crate::core::buffers;
use crate::utils::opengl::assert_gl_is_loaded;
use gl;
use std::mem::size_of;

/// An OpenGL Vertex Array Object.
pub struct VAO {
    _id: u32,
    _stride: i32,
    _size: u32,
}

impl VAO {
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
            _stride: stride as i32,
            _size: size,
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

    pub fn link_vbo(&self, vbo: &buffers::vbo::VBO, layout: u32) {
        assert_gl_is_loaded();
        vbo.bind();
        unsafe {
            gl::VertexAttribPointer(
                layout,
                3,
                gl::FLOAT,
                gl::FALSE,
                self._stride,
                (self._size * layout) as *const _,
            );
            gl::EnableVertexAttribArray(layout);
        }
        vbo.unbind();
    }
}
