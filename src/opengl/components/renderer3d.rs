use crate::opengl::{assert_gl_is_loaded, Drawable, Renderer};
use gl;
use gl::types::*;

pub struct Renderer3D<'a> {
    _clear_color: [f32; 4],
    _items: Vec<(&'a dyn Drawable, GLenum)>,
}

impl<'a> Renderer for Renderer3D<'a> {
    fn clear(&self) {
        assert_gl_is_loaded();
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    fn render(&self) {
        for i in 0..self._items.len() {
            let drawable = self.get_item(i);
            self.draw_mode(drawable.0, drawable.1);
        }
    }
}

impl<'a> Renderer3D<'a> {
    pub fn new() -> Renderer3D<'a> {
        return Renderer3D {
            _clear_color: [0.0, 0.0, 0.0, 1.0],
            _items: vec![],
        };
    }

    pub fn add_item(&mut self, drawable: &'a dyn Drawable) {
        self.add_item_with_mode(drawable, gl::TRIANGLES);
    }

    pub fn add_item_with_mode(&mut self, drawable: &'a dyn Drawable, mode: GLenum) {
        self._items.push((drawable, mode));
    }

    pub fn clear_color(mut self, r: f32, g: f32, b: f32, a: f32) -> Renderer3D<'a> {
        assert_gl_is_loaded();
        self._clear_color = [r, g, b, a];
        unsafe {
            gl::ClearColor(r, g, b, a);
        }
        return self;
    }

    pub fn get_item(&self, index: usize) -> (&'a dyn Drawable, GLenum) {
        return self._items[index];
    }

    /// Draws the given shape using triangles.
    pub fn draw(&self, drawable: &dyn Drawable) {
        drawable.draw();
    }

    /// Draws the given shape using triangles.
    pub fn draw_mode(&self, drawable: &dyn Drawable, mode: GLenum) {
        drawable.draw_with_mode(mode);
        // drawable.draw();
    }

    pub fn set_polygon_mode(&self, face: GLenum, mode: GLenum) {
        assert_gl_is_loaded();
        unsafe {
            gl::PolygonMode(face, mode);
        }
    }
}
