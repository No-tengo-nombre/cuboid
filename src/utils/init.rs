use gl;
use glfw::Context;

/// Initializes a GLFW window, setting it as the current one.
pub fn init_glfw(
    width: u32,
    height: u32,
    title: &str,
    mode: glfw::WindowMode,
) -> (
    glfw::Window,
    std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>,
    glfw::Glfw,
) {
    let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    let (mut window, events) = glfw
        .create_window(width, height, title, mode)
        .expect("Error creating GLFW window");
    window.make_current();
    window.set_key_polling(true);
    return (window, events, glfw);
}

/// Initializes the OpenGL functions. This must be run, or the program will
/// SegFault and crash.
pub fn init_gl(window: &mut glfw::Window) {
    gl::load_with(|s| window.get_proc_address(s) as *const _);
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }
}
