use std::mem::{
    size_of_val,
};
use ogl33::{
    glGenBuffers,
    glBindBuffer,
    glBufferData,
    glDeleteBuffers,
    GLenum,
    GL_ARRAY_BUFFER,
    GL_STATIC_DRAW,
};

struct VBO {
    id: u32,
}


impl VBO {
    fn new(vertices: &[f64]) -> VBO {
        let mut vbo = 0;
        unsafe {
            // Generate the VBO
            glGenBuffers(1, &mut vbo);
            assert_ne!(vbo, 0);
            glBindBuffer(GL_ARRAY_BUFFER, vbo);

            // Buffer the vertices
            glBufferData(
                GL_ARRAY_BUFFER,
                size_of_val(&vertices) as isize,
                vertices.as_ptr().cast(),
                GL_STATIC_DRAW,
            );
        }
        return VBO {id: vbo};
    }

    fn new_with_usage(vertices: &[f64], usage: GLenum) -> VBO {
        let mut vbo = 0;
        unsafe {
            // Generate the VBO
            glGenBuffers(1, &mut vbo);
            assert_ne!(vbo, 0);
            glBindBuffer(GL_ARRAY_BUFFER, vbo);

            // Buffer the vertices
            glBufferData(
                GL_ARRAY_BUFFER,
                size_of_val(&vertices) as isize,
                vertices.as_ptr().cast(),
                usage,
            );
        }
        return VBO {id: vbo};
    }

    fn bind(&self) {
        unsafe {
            glBindBuffer(GL_ARRAY_BUFFER, self.id);
        }
    }

    fn unbind() {
        unsafe {
            glBindBuffer(GL_ARRAY_BUFFER, 0);
        }
    }

    fn del(&self) {
        unsafe {
            glDeleteBuffers(1, &self.id);
        }
    }
}
