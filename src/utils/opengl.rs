use crate::utils::init::GL_IS_LOADED;

const GL_NOT_LOADED_MSG: &str = "
ERROR: OpenGL is not currently loaded. Make sure to initalize it at the beginning of the program
by running `init::init_gl(&window)`, where `window: glfw::Window`.
";

/// The following function checks that OpenGL is loaded, and should be called at the beginning of
/// every unsafe OpenGL function call.
pub fn assert_gl_is_loaded() {
    unsafe {
        if !GL_IS_LOADED {
            println!("{GL_NOT_LOADED_MSG}");
            std::process::exit(-1);
        }
    }
}
