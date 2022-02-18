use crate::core;


pub struct Material<'a> {
    _shader: &'a core::shader::Shader,
}


impl<'a> Material<'a> {
    pub fn del(&self) {
        self._shader.del();
    }


    pub fn use_program(&self) {
        self._shader.use_program();
    }
}


pub fn new(shader: &core::shader::Shader) -> Material {
    return Material {_shader: shader};
}
