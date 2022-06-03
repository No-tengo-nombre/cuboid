use crate::shader::Shader;

#[derive(Copy, Clone)]
pub struct Material {
    pub shader: Shader,
}

impl Material {
    pub fn new() -> Material {
        return Material {
            shader: Shader::new(),
        };
    }

    pub fn shader(mut self, shader: &Shader) -> Material {
        self.shader = *shader;
        return self;
    }

    pub fn del(&self) {
        self.shader.del();
    }

    pub fn use_program(&self) {
        self.shader.use_program();
    }
}
