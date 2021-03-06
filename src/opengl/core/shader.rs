use crate::opengl::assert_gl_is_loaded;
use crate::utils::str::name_to_ptr;
use crate::utils::types::V4;
use gl;
use gl::types::*;
use std::fs;

/// An OpenGL shader program.
#[derive(Copy, Clone)]
pub struct Shader {
    _id: u32,
}

impl Shader {
    pub fn new() -> Shader {
        assert_gl_is_loaded();
        unsafe {
            return Shader {
                _id: gl::CreateProgram(),
            };
        }
    }

    pub fn vertex(self, vertex_path: &str) -> Shader {
        assert_gl_is_loaded();
        // Making the vertex and fragment shaders
        let vertex_content = fs::read_to_string(vertex_path).expect("Error reading vertex shader.");
        let vertex_shader = Shader::make_vertex_shader(&vertex_content);
        Shader::verify_vertex_shader(&vertex_shader);

        unsafe {
            assert_ne!(self._id, 0);
            gl::AttachShader(self._id, vertex_shader);
            gl::LinkProgram(self._id);
            let mut success = 0;
            gl::GetProgramiv(self._id, gl::LINK_STATUS, &mut success);
            if success == 0 {
                let mut v: Vec<u8> = Vec::with_capacity(1024);
                let mut log_len = 0_i32;
                gl::GetProgramInfoLog(self._id, 1024, &mut log_len, v.as_mut_ptr().cast());
                v.set_len(log_len.try_into().unwrap());
                panic!("Program Link Error: {}", String::from_utf8_lossy(&v));
            }
            gl::DeleteShader(vertex_shader);
        }
        return self;
    }

    pub fn fragment(self, fragment_path: &str) -> Shader {
        assert_gl_is_loaded();
        // Making the vertex and fragment shaders
        let fragment_content =
            fs::read_to_string(fragment_path).expect("Error reading fragment shader.");
        let fragment_shader = Shader::make_fragment_shader(&fragment_content);
        Shader::verify_fragment_shader(&fragment_shader);

        unsafe {
            assert_ne!(self._id, 0);
            gl::AttachShader(self._id, fragment_shader);
            gl::LinkProgram(self._id);
            let mut success = 0;
            gl::GetProgramiv(self._id, gl::LINK_STATUS, &mut success);
            if success == 0 {
                let mut v: Vec<u8> = Vec::with_capacity(1024);
                let mut log_len = 0_i32;
                gl::GetProgramInfoLog(self._id, 1024, &mut log_len, v.as_mut_ptr().cast());
                v.set_len(log_len.try_into().unwrap());
                panic!("Program Link Error: {}", String::from_utf8_lossy(&v));
            }
            gl::DeleteShader(fragment_shader);
        }
        return self;
    }

    pub fn make_shader(content: &str, shader_type: GLenum) -> GLuint {
        assert_gl_is_loaded();
        let shader;
        unsafe {
            shader = gl::CreateShader(shader_type);
            assert_ne!(shader, 0);
            gl::ShaderSource(
                shader,
                1,
                &(content.as_bytes().as_ptr().cast()),
                &(content.len().try_into().unwrap()),
            );
            gl::CompileShader(shader);
        }
        return shader;
    }

    fn make_vertex_shader(content: &str) -> GLuint {
        return Shader::make_shader(content, gl::VERTEX_SHADER);
    }

    fn make_fragment_shader(content: &str) -> GLuint {
        return Shader::make_shader(content, gl::FRAGMENT_SHADER);
    }

    pub fn verify_shader(shader: &GLuint, message: &str) {
        assert_gl_is_loaded();
        let mut success = 0;
        unsafe {
            gl::GetShaderiv(*shader, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                let mut v: Vec<u8> = Vec::with_capacity(1024);
                let mut log_len = 0_i32;
                gl::GetShaderInfoLog(*shader, 1024, &mut log_len, v.as_mut_ptr().cast());
                v.set_len(log_len.try_into().unwrap());
                panic!("{} : {}", message, String::from_utf8_lossy(&v));
            }
        }
    }

    fn verify_vertex_shader(vertex_shader: &GLuint) {
        Shader::verify_shader(vertex_shader, "Vertex Compile Error");
    }

    fn verify_fragment_shader(fragment_shader: &GLuint) {
        Shader::verify_shader(fragment_shader, "Fragment Compile Error");
    }

    pub fn use_program(&self) {
        assert_gl_is_loaded();
        unsafe {
            gl::UseProgram(self._id);
        }
    }

    pub fn del(&self) {
        assert_gl_is_loaded();
        unsafe {
            gl::DeleteShader(self._id);
        }
    }

    pub fn set_1i(&self, name: &str, v0: i32) {
        assert_gl_is_loaded();
        unsafe {
            gl::Uniform1i(gl::GetUniformLocation(self._id, name_to_ptr(name)), v0);
        }
    }

    pub fn set_2i(&self, name: &str, v0: i32, v1: i32) {
        assert_gl_is_loaded();
        unsafe {
            gl::Uniform2i(gl::GetUniformLocation(self._id, name_to_ptr(name)), v0, v1);
        }
    }

    pub fn set_3i(&self, name: &str, v0: i32, v1: i32, v2: i32) {
        assert_gl_is_loaded();
        unsafe {
            gl::Uniform3i(
                gl::GetUniformLocation(self._id, name_to_ptr(name)),
                v0,
                v1,
                v2,
            );
        }
    }

    pub fn set_4i(&self, name: &str, v0: i32, v1: i32, v2: i32, v3: i32) {
        assert_gl_is_loaded();
        unsafe {
            gl::Uniform4i(
                gl::GetUniformLocation(self._id, name_to_ptr(name)),
                v0,
                v1,
                v2,
                v3,
            );
        }
    }

    pub fn set_1f(&self, name: &str, v0: f32) {
        assert_gl_is_loaded();
        unsafe {
            gl::Uniform1f(gl::GetUniformLocation(self._id, name_to_ptr(name)), v0);
        }
    }

    pub fn set_2f(&self, name: &str, v0: f32, v1: f32) {
        assert_gl_is_loaded();
        unsafe {
            gl::Uniform2f(gl::GetUniformLocation(self._id, name_to_ptr(name)), v0, v1);
        }
    }

    pub fn set_3f(&self, name: &str, v0: f32, v1: f32, v2: f32) {
        assert_gl_is_loaded();
        unsafe {
            gl::Uniform3f(
                gl::GetUniformLocation(self._id, name_to_ptr(name)),
                v0,
                v1,
                v2,
            );
        }
    }

    pub fn set_4f(&self, name: &str, v0: f32, v1: f32, v2: f32, v3: f32) {
        assert_gl_is_loaded();
        unsafe {
            gl::Uniform4f(
                gl::GetUniformLocation(self._id, name_to_ptr(name)),
                v0,
                v1,
                v2,
                v3,
            );
        }
    }

    pub fn set_matrix4fv(&self, name: &str, value: &[V4; 4]) {
        assert_gl_is_loaded();
        unsafe {
            gl::UniformMatrix4fv(
                gl::GetUniformLocation(self._id, name_to_ptr(name)),
                1,
                gl::FALSE,
                value.as_ptr().cast(),
            );
        }
    }

    pub fn get_id(&self) -> u32 {
        return self._id;
    }
}
