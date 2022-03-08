use crate::utils::images::load;
use crate::utils::opengl::assert_gl_is_loaded;
use gl;
use stb_image_rust;

#[derive(Copy, Clone)]
pub struct Texture2D {
    _id: u32,
}

impl Texture2D {
    pub fn new(path: &str) -> Texture2D {
        assert_gl_is_loaded();
        let mut id = 0;
        let (img, width, height) = load(path);
        // println!("{}", img.width());
        // let img_ptr = img.to_rgba8().as_mut_ptr();

        unsafe {
            println!("DSAGSGDGSG");
            println!("{}", *img);
        }

        unsafe {
            // Generate the texture
            gl::GenTextures(1, &mut id);
            assert_ne!(id, 0);
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
                // img_ptr as *const _,
                // img.to_rgba8().as_mut_ptr() as *const _,
                img as *const _,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
            stb_image_rust::c_runtime::free(img);
        }
        return Texture2D { _id: id };
    }

    pub fn new_empty() -> Texture2D {
        return Texture2D { _id: 0 };
    }

    pub fn create(&mut self, path: &str) {
        assert_gl_is_loaded();
        let mut id = 0;
        let (img, width, height) = load(path);
        // let img_ptr = img.to_rgba8().as_mut_ptr();

        unsafe {
            // Generate the texture
            gl::GenTextures(1, &mut id);
            assert_ne!(id, 0);
            gl::BindTexture(gl::TEXTURE_2D, id);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                gl::LINEAR_MIPMAP_LINEAR as i32,
            );
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
                // img_ptr as *const _,
                img as *const _,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);

            stb_image_rust::c_runtime::free(img);
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
