use crate::components::{Material, Texture2D};
use crate::core::{buffers::EBO, buffers::VAO, buffers::VBO, traits};
use crate::utils::opengl::assert_gl_is_loaded;
use gl::types::*;
use std::mem::size_of;

pub struct Shape<'a> {
    _vao: VAO,
    _ebo: EBO,
    _material: &'a Material<'a>,
    _texture: Texture2D,
}

impl<'a> traits::Drawable for Shape<'a> {
    fn get_drawn(&self, mode: GLenum) {
        self.use_material();
        self.bind_vao();
        self.bind_ebo();
        self.bind_texture();
        assert_gl_is_loaded();
        unsafe {
            gl::DrawElements(
                mode,
                self.get_ebo_count().try_into().unwrap(),
                gl::UNSIGNED_INT,
                0 as *const _,
            );
        }
        self.unbind_vao();
        self.unbind_ebo();
        self.unbind_texture();
    }
}

impl<'a> Drop for Shape<'a> {
    fn drop(&mut self) {
        self.del();
    }
}

impl<'a> Shape<'a> {
    pub fn new<T>(
        vertices: &[T],
        indices: &[u32],
        material: &'a Material,
        layouts: &[u32],
    ) -> Shape<'a> {
        return Shape::new_with_count(
            vertices,
            indices,
            material,
            layouts,
            indices.len().try_into().unwrap(),
        );
    }
    pub fn new_with_count<T>(
        vertices: &[T],
        indices: &[u32],
        material: &'a Material,
        layouts: &[u32],
        count: u32,
    ) -> Shape<'a> {
        return Shape::new_with_count_usage(
            vertices,
            indices,
            material,
            layouts,
            count,
            gl::STATIC_DRAW,
        );
    }

    pub fn new_with_usage<T>(
        vertices: &[T],
        indices: &[u32],
        material: &'a Material,
        layouts: &[u32],
        usage: GLenum,
    ) -> Shape<'a> {
        return Shape::new_with_count_usage(
            vertices,
            indices,
            material,
            layouts,
            indices.len().try_into().unwrap(),
            usage,
        );
    }

    pub fn new_with_count_usage<T>(
        vertices: &[T],
        indices: &[u32],
        material: &'a Material,
        layouts: &[u32],
        count: u32,
        usage: GLenum,
    ) -> Shape<'a> {
        let vao = VAO::new_typed::<T>((size_of::<T>() as u32) / 2);
        vao.bind();
        let vbo = VBO::new_with_usage(vertices, usage);
        let ebo = EBO::new_with_usage(indices, count, usage);
        for i in 0..layouts.len() {
            vao.link_vbo(&vbo, layouts[i]);
        }
        vao.unbind();
        vbo.unbind();
        ebo.unbind();
        let empty_texture = Texture2D::new_empty();
        return Shape {
            _vao: vao,
            _ebo: ebo,
            _material: material,
            _texture: empty_texture,
        };
    }

    pub fn set_texture(&mut self, tex: &Texture2D) {
        self._texture = *tex;
    }

    pub fn set_texture_path(&mut self, path: &str) {
        self._texture = Texture2D::new(path);
    }

    pub fn set_vertices<T>(&self, vertices: &[T], layouts: &[u32]) {
        self._vao.bind();
        let vbo = VBO::new(vertices);
        for i in 0..layouts.len() {
            self._vao.link_vbo(&vbo, layouts[i]);
        }
        self._vao.unbind();
        vbo.unbind();
    }

    pub fn use_material(&self) {
        self._material.use_program();
    }

    pub fn bind_vao(&self) {
        self._vao.bind();
    }

    pub fn bind_ebo(&self) {
        self._ebo.bind();
    }

    pub fn bind_texture(&self) {
        self._texture.bind();
    }

    pub fn unbind_vao(&self) {
        self._vao.unbind();
    }

    pub fn unbind_ebo(&self) {
        self._ebo.unbind();
    }

    pub fn unbind_texture(&self) {
        self._texture.unbind();
    }

    pub fn get_ebo_count(&self) -> u32 {
        return self._ebo.get_count();
    }

    pub fn del(&self) {
        self._vao.del();
        self._ebo.del();
        self._material.del();
    }
}
