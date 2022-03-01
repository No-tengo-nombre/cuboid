use crate::utils::images::load;
use crate::utils::opengl::assert_gl_is_loaded;
use gl;
use stb_image_rust;

pub struct Texture2D {
    _id: u32,
}

impl Texture2D {
    pub fn new(path: &str) -> Texture2D {
        assert_gl_is_loaded();
        let mut id = 0;
        let (data, width, height) = load(path);

        unsafe {
            // Generate the texture
            gl::GenTextures(1, &mut id);
            assert_ne!(id, 0);
            gl::BindTexture(gl::TEXTURE_2D, id);
            
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);	
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                width,
                height,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                data as *const _,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
            
            stb_image_rust::c_runtime::free(data);
        }
        return Texture2D {_id: id};
    }

    pub fn new_empty() -> Texture2D {
        return Texture2D { _id: 0 };
    }

    pub fn create(&mut self, path: &str) {
        assert_gl_is_loaded();
        let mut id = 0;
        let (data, width, height) = load(path);

        unsafe {
            // Generate the texture
            gl::GenTextures(1, &mut id);
            assert_ne!(id, 0);
            gl::BindTexture(gl::TEXTURE_2D, id);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);	
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                width,
                height,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                data as *const _,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);

            stb_image_rust::c_runtime::free(data);
        }
        self._id = id;
    }

    pub fn bind(&self) {
        assert_gl_is_loaded();
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self._id);
        }
    }

    pub fn unbind(&self) {
        assert_gl_is_loaded();
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }
}
