use crate::components::{Material, Texture2D};
use crate::{
    assert_gl_is_loaded,
    buffers::{EBO, VAO, VBO},
    Drawable,
};
use gl::types::*;

/// Container for a drawable shape
pub struct Shape<'a, T> {
    _vao: VAO<'a>,
    _ebo: EBO<'a>,
    pub _material: Material,
    pub _texture: Texture2D,
    pub _vertices: &'a [T],
    pub _indices: &'a [u32],
    pub _layouts: &'a [u32],
    pub _count: u32,
    pub _usage: GLenum,
    pub _vert_sizes: &'a [u32],
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
                self._count.try_into().unwrap(),
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

impl<'a, T> Shape<'a, T> {
    pub fn new() -> Shape<'a, T> {
        return Shape {
            _vao: VAO::new(),
            _ebo: EBO::new(),
            _material: Material::new(),
            _texture: Texture2D::new(),
            _vertices: &[],
            _indices: &[],
            _layouts: &[],
            _count: 0,
            _usage: gl::STATIC_DRAW,
            _vert_sizes: &[],
        };
    }

    pub fn material(mut self, material: &Material) -> Shape<'a, T> {
        self._material = *material;
        return self;
    }

    pub fn texture(mut self, texture: &Texture2D) -> Shape<'a, T> {
        self._texture = *texture;
        return self;
    }

    pub fn vertices(mut self, vertices: &'a [T]) -> Shape<'a, T> {
        self._vertices = vertices;
        return self;
    }

    pub fn indices(mut self, indices: &'a [u32]) -> Shape<'a, T> {
        self._indices = indices;
        self._count = indices.len().try_into().unwrap();
        return self;
    }

    pub fn layouts(mut self, layouts: &'a [u32]) -> Shape<'a, T> {
        self._layouts = layouts;
        return self;
    }

    pub fn count(mut self, count: u32) -> Shape<'a, T> {
        self._count = count;
        return self;
    }

    pub fn usage(mut self, usage: GLenum) -> Shape<'a, T> {
        self._usage = usage;
        return self;
    }

    pub fn vert_sizes(mut self, vert_sizes: &'a [u32]) -> Shape<'a, T> {
        self._vert_sizes = vert_sizes;
        return self;
    }

    pub fn build(mut self) -> Shape<'a, T> {
        let vao = VAO::new()
            .stride_from_type::<T>()
            .sizes(self._vert_sizes)
            .build();
        vao.bind();
        let vbo = VBO::new()
            .vertices(self._vertices)
            .usage(self._usage)
            .build();
        let ebo = EBO::new()
            .indices(self._indices)
            .count(self._count)
            .usage(self._usage)
            .build();

        for l in self._layouts {
            vao.link_vbo(&vbo, *l);
        }

        vao.unbind();
        vbo.unbind();
        ebo.unbind();
        self._vao = vao;
        self._ebo = ebo;

        return self;
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

    pub fn del(&self) {
        self._vao.del();
        self._ebo.del();
        self._material.del();
    }
}
