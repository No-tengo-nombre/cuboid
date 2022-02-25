use crate::components::material;
use crate::core::{buffers::ebo::EBO, buffers::vao::VAO, buffers::vbo::VBO, traits};
use gl::types::*;
use std::mem::size_of;

pub struct Shape<'a> {
    _vao: VAO,
    _ebo: EBO,
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

impl<'a> Drop for Shape<'a> {
    fn drop(&mut self) {
        self.del();
    }
}

impl<'a> Shape<'a> {
    pub fn new<T>(
        vertices: &[T],
        indices: &[u32],
        material: &'a material::Material,
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
        material: &'a material::Material,
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
        // let vao = VAO::new_typed::<T>((size_of::<T>() as u32) / 2);
        // vao.bind();
        // let vbo = VBO::new(vertices);
        // let ebo = EBO::new(indices, count);
        // for i in 0..layouts.len() {
        //     vao.link_vbo(&vbo, layouts[i]);
        // }
        // vao.unbind();
        // vbo.unbind();
        // ebo.unbind();
        // return Shape {
        //     _vao: vao,
        //     _ebo: ebo,
        //     _material: material,
        // };
    }

    pub fn new_with_usage<T>(
            vertices: &[T],
            indices: &[u32],
            material: &'a material::Material,
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
        material: &'a material::Material,
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
        return Shape {
            _vao: vao,
            _ebo: ebo,
            _material: material,
        };
    }

    pub fn set_vertices<T>(&self, vertices: &[T], layouts: &[u32]) {
        // let vao = VAO::new_typed::<T>((size_of::<T>() as u32) / 2);
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

    pub fn get_ebo_count(&self) -> u32 {
        return self._ebo.get_count();
    }

    pub fn del(&self) {
        self._vao.del();
        self._ebo.del();
        self._material.del();
    }
}
