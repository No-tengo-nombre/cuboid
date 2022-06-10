use crate::opengl::assert_gl_is_loaded;
use crate::utils::images::load;
use gl;

#[derive(Copy, Clone)]
pub struct Texture2D {
    _id: u32,
}

impl Texture2D {
    pub fn new() -> Texture2D {
        return Texture2D { _id: 0 };
    }

    pub fn from_path(path: &str) -> Texture2D {
        return Texture2D::new().load_from_path(path);
    }

    pub fn load_from_path(mut self, path: &str) -> Texture2D {
        assert_gl_is_loaded();
        let mut id = 0;
        let (img, width, height) = load(path);
        let data = img.to_rgba8().into_vec();

        unsafe {
            // Generate the texture
            gl::GenTextures(1, &mut id);
            assert_ne!(id, 0);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, id);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA.try_into().unwrap(),
                width,
                height,
                0,
                gl::RGBA.try_into().unwrap(),
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const _,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
        self._id = id;
        return self;
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
