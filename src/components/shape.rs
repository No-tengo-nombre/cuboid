use crate::core::buffers;
use crate::components::material;
use std::mem::{
    size_of,
};


pub struct Shape<'a> {
    _vao: buffers::vao::VAO,
    _ebo: buffers::ebo::EBO,
    _material: &'a material::Material<'a>,
}


impl<'a> Shape<'a> {
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


pub fn new<'a, T>(vertices: &[T], indices: &[u32], material: &'a material::Material) -> Shape<'a> {
    return new_with_count(vertices, indices, material, 6);
}


pub fn new_with_count<'a, T>(vertices: &[T], indices: &[u32], material: &'a material::Material, count: u32) -> Shape<'a> {
    let vao = buffers::vao::new_typed::<T>((size_of::<T>() as u32) / 2);
    vao.bind();
    let vbo = buffers::vbo::new(vertices);
    let ebo = buffers::ebo::new(indices, count);

    vao.link_vbo(&vbo, 0);
    vao.link_vbo(&vbo, 1);

    vao.unbind();
    vbo.unbind();
    ebo.unbind();

    return Shape {
        _vao: vao,
        _ebo: ebo,
        _material: material,
    };
}
