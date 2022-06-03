use crate::shader::Shader;

pub struct Material {
    pub shader: Shader,
}

impl Material {
    // pub fn new(shader: &crate::shader::Shader) -> Material {
    //     return Material { _shader: shader };
    // }

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
