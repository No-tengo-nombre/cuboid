pub struct Material<'a> {
    _shader: &'a crate::shader::Shader,
}

impl<'a> Material<'a> {
    pub fn new(shader: &crate::shader::Shader) -> Material {
        return Material { _shader: shader };
    }

    pub fn del(&self) {
        self._shader.del();
    }

    pub fn use_program(&self) {
        self._shader.use_program();
    }

    pub fn get_shader(&self) -> &crate::shader::Shader {
        return self._shader;
    }
}
