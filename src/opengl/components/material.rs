use crate::opengl::Shader;

#[derive(Copy, Clone)]
pub struct Material {
    pub _shader: Shader,
}

impl Material {
    pub fn new() -> Material {
        return Material {
            _shader: Shader::new(),
        };
    }

    pub fn shader(mut self, shader: &Shader) -> Material {
        self._shader = *shader;
        return self;
    }

    pub fn del(&self) {
        self._shader.del();
    }

    pub fn use_program(&self) {
        self._shader.use_program();
    }
}
