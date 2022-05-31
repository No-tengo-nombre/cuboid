// use gl;
// use glfw::Context;
// use std::sync::mpsc::Receiver;

// pub static mut GL_IS_LOADED: bool = false;

// pub enum WindowMode {
//     Windowed,
//     FullScreen,
// }

// /// Initializes a GLFW window, setting it as the current one.
// pub fn init_glfw(
//     width: u32,
//     height: u32,
//     title: &str,
//     mode: WindowMode,
// ) -> (glfw::Window, Receiver<(f64, glfw::WindowEvent)>, glfw::Glfw) {
//     println!("{}", glfw::get_version_string());

//     let mut glfw_inst = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

//     glfw_inst.window_hint(glfw::WindowHint::CenterCursor(true));
//     glfw_inst.window_hint(glfw::WindowHint::ContextVersion(4, 2));
//     glfw_inst.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
//     glfw_inst.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

//     println!("{}", glfw::get_version_string());

//     let (mut window, events) = match mode {
//         WindowMode::Windowed => {
//             glfw_inst
//                 .create_window(width, height, title, glfw::WindowMode::Windowed)
//                 .expect("Error creating GLFW window")
//         },
//         WindowMode::FullScreen => {
//             glfw_inst.with_primary_monitor(|temp_glfw, m| {
//                 temp_glfw.create_window(width, height, title, m.map_or(glfw::WindowMode::Windowed, |m| glfw::WindowMode::FullScreen(m)))
//             }).expect("Error creating GLFW window")
//         },
//     };

//     window.set_key_polling(true);
//     window.set_cursor_mode(glfw::CursorMode::Normal);
//     window.set_cursor_enter_polling(true);
//     window.set_mouse_button_polling(true);
//     window.set_cursor_pos_polling(true);
//     window.make_current();

//     return (window, events, glfw_inst);
// }

// /// Initializes the OpenGL functions. This must be run, or the program will
// /// SegFault and crash.
// pub fn init_gl(window: &mut glfw::Window) {
//     unsafe {
//         GL_IS_LOADED = true;
//     }
//     gl::load_with(|s| window.get_proc_address(s) as *const _);
//     enable_depth_test();
// }

// fn enable_depth_test() {
//     unsafe {
//         gl::Enable(gl::DEPTH_TEST);
//     }
// }
