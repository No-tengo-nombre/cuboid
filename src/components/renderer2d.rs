use crate::core::traits::Drawable;
use gl;
use gl::types::*;

pub struct Renderer2D<'a> {
    _clear_color: [f32; 4],
    _layers: Vec<Vec<(&'a dyn Drawable, GLenum)>>,
}

impl<'a> Renderer2D<'a> {
    pub fn new() -> Renderer2D<'a> {
        return Renderer2D {
            _clear_color: [0.0, 0.0, 0.0, 1.0],
            // Really ugly code, TODO: Fix the layers
            _layers: vec![vec![], vec![], vec![], vec![]],
        };
    }

    pub fn add_item(&mut self, drawable: &'a dyn Drawable, layer: usize) {
        self.add_item_with_mode(drawable, layer, gl::TRIANGLES);
    }

    pub fn add_item_with_mode(&mut self, drawable: &'a dyn Drawable, layer: usize, mode: GLenum) {
        self._layers[layer].push((drawable, mode));
    }

    pub fn clear(&self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }
    pub fn set_clear_color(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self._clear_color = [r, g, b, a];
        unsafe {
            gl::ClearColor(r, g, b, a);
        }
    }

    pub fn get_layer(&self, layer: usize) -> &Vec<(&dyn Drawable, GLenum)> {
        return &self._layers[layer];
    }

    pub fn render(&self) {
        for i in 0..self._layers.len() {
            let layer = self.get_layer(i);
            for j in 0..layer.len() {
                let drawable = &layer[j];
                self.draw_mode(drawable.0, drawable.1);
            }
        }
    }

    /// Draws the given shape using triangles.
    pub fn draw(&self, drawable: &dyn Drawable) {
        self.draw_mode(drawable, gl::TRIANGLES);
    }

    /// Draws the given shape using triangles.
    pub fn draw_mode(&self, drawable: &dyn Drawable, mode: GLenum) {
        drawable.get_drawn(mode);
    }
}
