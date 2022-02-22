use crate::components::material;
use crate::core::{buffers, buffers::ebo::EBO, traits};
use gl::types::*;
use std::mem::size_of;

pub struct Shape<'a> {
    _vao: buffers::vao::VAO,
    _ebo: buffers::ebo::EBO,
    _material: &'a material::Material<'a>,
}

impl<'a> traits::Drawable for Shape<'a> {
    fn get_drawn(&self, mode: GLenum) {
        self.use_material();
        self.bind_vao();
        self.bind_ebo();
        unsafe {
            gl::DrawElements(
                mode,
                self.get_ebo_count().try_into().unwrap(),
                gl::UNSIGNED_INT,
                0 as *const _,
            );
        }
    }
}

impl<'a> Shape<'a> {
    pub fn new<T>(
        vertices: &[T],
        indices: &[u32],
        material: &'a material::Material,
        layouts: &[u32],
    ) -> Shape<'a> {
        return Shape::new_with_count(vertices, indices, material, layouts, 6);
    }
    pub fn new_with_count<T>(
        vertices: &[T],
        indices: &[u32],
        material: &'a material::Material,
        layouts: &[u32],
        count: u32,
    ) -> Shape<'a> {
        let vao = buffers::vao::new_typed::<T>((size_of::<T>() as u32) / 2);
        vao.bind();
        let vbo = buffers::vbo::new(vertices);
        let ebo = EBO::new(indices, count);
        for i in 0..layouts.len() {
            vao.link_vbo(&vbo, layouts[i]);
        }
        vao.unbind();
        vbo.unbind();
        ebo.unbind();
        return Shape {
            _vao: vao,
            _ebo: ebo,
            _material: material,
        };
    }

    pub fn set_vertices<T>(&mut self, vertices: &[T], layouts: &[u32]) {
        let vao = buffers::vao::new_typed::<T>((size_of::<T>() as u32) / 2);
        vao.bind();
        let vbo = buffers::vbo::new(vertices);
        for i in 0..layouts.len() {
            vao.link_vbo(&vbo, layouts[i]);
        }
        vao.unbind();
        vbo.unbind();
        self._vao = vao;
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

    pub fn get_ebo_count(&self) -> u32 {
        return self._ebo.get_count();
    }

    pub fn del(&self) {
        self._vao.del();
        self._ebo.del();
        self._material.del();
    }
}
