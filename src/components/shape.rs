use crate::components::{Material, Texture2D};
use crate::{
    assert_gl_is_loaded,
    buffers::{EBO, VAO, VBO},
    Drawable,
};
use gl::types::*;
use std::mem::size_of;

pub struct Shape<'a, T> {
    _vao: VAO,
    _ebo: EBO<'a>,
    pub material: Material,
    pub texture: Texture2D,
    pub vertices: &'a [T],
    pub indices: &'a [u32],
    pub layouts: &'a [u32],
    pub count: u32,
    pub usage: GLenum,
}

impl<'a, T> Drawable for Shape<'a, T> {
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

// impl<'a> traits::Transformable for Shape<'a> {
//     fn apply_transform(&self, transform: &Transform) -> Self {
//
//     }
// }

impl<'a, T> Drop for Shape<'a, T> {
    fn drop(&mut self) {
        self.del();
    }
}

// impl<'a, T> Shape<'a, T> {
    // pub fn new<T>(
    //     vertices: &[T],
    //     indices: &[u32],
    //     material: &'a Material,
    //     layouts: &[u32],
    // ) -> Shape<'a> {
    //     return Shape::new_with_count(
    //         vertices,
    //         indices,
    //         material,
    //         layouts,
    //         indices.len().try_into().unwrap(),
    //     );
    // }
    // pub fn new_with_count<T>(
    //     vertices: &[T],
    //     indices: &[u32],
    //     material: &'a Material,
    //     layouts: &[u32],
    //     count: u32,
    // ) -> Shape<'a> {
    //     return Shape::new_with_count_usage(
    //         vertices,
    //         indices,
    //         material,
    //         layouts,
    //         count,
    //         gl::STATIC_DRAW,
    //     );
    // }

    // pub fn new_with_usage<T>(
    //     vertices: &[T],
    //     indices: &[u32],
    //     material: &'a Material,
    //     layouts: &[u32],
    //     usage: GLenum,
    // ) -> Shape<'a> {
    //     return Shape::new_with_count_usage(
    //         vertices,
    //         indices,
    //         material,
    //         layouts,
    //         indices.len().try_into().unwrap(),
    //         usage,
    //     );
    // }

    // pub fn new_with_count_usage<T>(
    //     vertices: &[T],
    //     indices: &[u32],
    //     material: &'a Material,
    //     layouts: &[u32],
    //     count: u32,
    //     usage: GLenum,
    // ) -> Shape<'a> {
    //     let vao = VAO::new_typed::<T>((size_of::<T>() as u32) / 2);
    //     vao.bind();
    //     let vbo = VBO::new_with_usage(vertices, usage);
    //     let ebo = EBO::new_with_usage(indices, count, usage);
    //     for i in 0..layouts.len() {
    //         vao.link_vbo(&vbo, layouts[i]);
    //     }
    //     vao.unbind();
    //     vbo.unbind();
    //     ebo.unbind();
    //     let empty_texture = Texture2D::new_empty();
    //     return Shape {
    //         _vao: vao,
    //         _ebo: ebo,
    //         _material: material,
    //         _texture: empty_texture,
    //     };
    // }

    // pub struct Shape<'a, T> {
    //     _vao: VAO,
    //     _ebo: EBO<'a>,
    //     pub material: Material,
    //     pub texture: Texture2D,
    //     pub vertices: &'a [T],
    //     pub indices: &'a [u32],
    //     pub layouts: &'a [u32],
    //     pub count: u32,
    //     pub usage: GLenum,
    // }

impl<'a, T> Shape<'a, T> {
    pub fn new() -> Shape<'a, T> {
        return Shape {
            _vao: VAO::new(),
            _ebo: EBO::new(),
            material: Material::new(),
            texture: Texture2D::new(),
            vertices: &[],
            indices: &[],
            layouts: &[],
            count: 0,
            usage: gl::STATIC_DRAW,
        };
    }

    pub fn material(mut self, material: &Material) -> Shape<'a, T> {
        self.material = *material;
        return self;
    }

    pub fn texture(mut self, texture: &Texture2D) -> Shape<'a, T> {
        self.texture = *texture;
        return self;
    }

    pub fn vertices(mut self, vertices: &'a [T]) -> Shape<'a, T> {
        self.vertices = vertices;
        return self;
    }

    pub fn indices(mut self, indices: &'a [u32]) -> Shape<'a, T> {
        self.indices = indices;
        self.count = indices.len().try_into().unwrap();
        return self;
    }

    pub fn layouts(mut self, layouts: &'a [u32]) -> Shape<'a, T> {
        self.layouts = layouts;
        return self;
    }

    pub fn count(mut self, count: u32) -> Shape<'a, T> {
        self.count = count;
        return self;
    }

    pub fn usage(mut self, usage: GLenum) -> Shape<'a, T> {
        self.usage = usage;
        return self;
    }

    pub fn build(mut self) -> Shape<'a, T> {
        let vao = VAO::new()
            .stride_from_type::<T>()
            .size(size_of::<T>() as u32 / 2)
            .build();
        vao.bind();
        let vbo = VBO::new()
            .vertices(self.vertices)
            .usage(self.usage)
            .build();
        let ebo = EBO::new()
            .indices(self.indices)
            .count(self.count)
            .usage(self.usage)
            .build();

        for i in 0..self.layouts.len() {
            vao.link_vbo(&vbo, self.layouts[i]);
        }
        vao.unbind();
        vbo.unbind();
        ebo.unbind();
        self._vao = vao;
        self._ebo = ebo;

        return self;
    }
    
    // pub fn new_with_count_usage<T>(
    //     vertices: &[T],
    //     indices: &[u32],
    //     material: &'a Material,
    //     layouts: &[u32],
    //     count: u32,
    //     usage: GLenum,
    // ) -> Shape<'a> {
    //     let vao = VAO::new_typed::<T>((size_of::<T>() as u32) / 2);
    //     vao.bind();
    //     let vbo = VBO::new_with_usage(vertices, usage);
    //     let ebo = EBO::new_with_usage(indices, count, usage);
    //     for i in 0..layouts.len() {
    //         vao.link_vbo(&vbo, layouts[i]);
    //     }
    //     vao.unbind();
    //     vbo.unbind();
    //     ebo.unbind();
    //     let empty_texture = Texture2D::new();
    //     return Shape {
    //         _vao: vao,
    //         _ebo: ebo,
    //         material: material,
    //         texture: empty_texture,
    //     };
    // }

    // pub fn set_texture(&mut self, tex: &Texture2D) {
    //     self._texture = *tex;
    // }

    // pub fn texture_from_path(&mut self, path: &str) {
    //     self.texture = Texture2D::new().from_path(path);
    // }

    // pub fn set_vertices<T>(&self, vertices: &[T], layouts: &[u32]) {
    //     self._vao.bind();
    //     let vbo = VBO::new().vertices(vertices).build();
    //     for i in 0..layouts.len() {
    //         self._vao.link_vbo(&vbo, layouts[i]);
    //     }
    //     self._vao.unbind();
    //     vbo.unbind();
    // }

    pub fn use_material(&self) {
        self.material.use_program();
    }

    pub fn bind_vao(&self) {
        self._vao.bind();
    }

    pub fn bind_ebo(&self) {
        self._ebo.bind();
    }

    pub fn bind_texture(&self) {
        self.texture.bind();
    }

    pub fn unbind_vao(&self) {
        self._vao.unbind();
    }

    pub fn unbind_ebo(&self) {
        self._ebo.unbind();
    }

    pub fn unbind_texture(&self) {
        self.texture.unbind();
    }

    pub fn get_ebo_count(&self) -> u32 {
        return self._ebo.count;
    }

    pub fn del(&self) {
        self._vao.del();
        self._ebo.del();
        self.material.del();
    }
}
