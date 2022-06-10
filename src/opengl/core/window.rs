use gl;
use glfw;
use glfw::Context;
use std::sync::mpsc::Receiver;

pub static mut GL_IS_LOADED: bool = false;
pub const GL_NOT_LOADED_MSG: &str = "
ERROR: OpenGL is not currently loaded. Make sure to initalize it at the beginning of the program
by running init::init_gl(&window), where window is the instance of glfw::Window that you are
using.
";

#[derive(Copy, Clone)]
pub enum WindowMode {
    Windowed,
    FullScreen(u32),
}

pub struct Window {
    _width: u32,
    _height: u32,
    _title: String,
    _mode: WindowMode,
    _glfw_window: glfw::Window,
    _events: Receiver<(f64, glfw::WindowEvent)>,
    _glfw_instance: glfw::Glfw,
}

impl Window {
    pub fn new() -> Window {
        let empty_instance = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        let (window, receiver) = empty_instance
            .create_window(0, 0, "", glfw::WindowMode::Windowed)
            .expect("");
        return Window {
            _width: 640,
            _height: 480,
            _title: "Hello World!".to_string(),
            _mode: WindowMode::Windowed,
            _glfw_window: window,
            _events: receiver,
            _glfw_instance: empty_instance,
        };
    }

    pub fn width(mut self, width: u32) -> Window {
        self._width = width;
        return self;
    }

    pub fn height(mut self, height: u32) -> Window {
        self._height = height;
        return self;
    }

    pub fn dimensions(mut self, width: u32, height: u32) -> Window {
        self._width = width;
        self._height = height;
        return self;
    }

    pub fn title(mut self, title: &str) -> Window {
        self._title = title.to_string();
        return self;
    }

    pub fn mode(mut self, mode: WindowMode) -> Window {
        self._mode = mode;
        return self;
    }

    pub fn windowed(mut self) -> Window {
        self._mode = WindowMode::Windowed;
        return self;
    }

    pub fn fullscreen(mut self, monitor: u32) -> Window {
        self._mode = WindowMode::FullScreen(monitor);
        return self;
    }

    pub fn build(mut self) -> Window {
        let (mut window, events, glfw_inst) =
            init_glfw(self._width, self._height, &self._title, self._mode);
        init_gl(&mut window);
        self._glfw_window = window;
        self._events = events;
        self._glfw_instance = glfw_inst;
        return self;
    }

    // The following are functions delegated to the fields of the struct

    pub fn should_close(&self) -> bool {
        return self._glfw_window.should_close();
    }

    pub fn set_should_close(&mut self, condition: bool) {
        self._glfw_window.set_should_close(condition);
    }

    pub fn poll_events(&mut self) -> &Receiver<(f64, glfw::WindowEvent)> {
        self._glfw_instance.poll_events();
        return &self._events;
    }

    pub fn swap_buffers(&mut self) {
        self._glfw_window.swap_buffers();
    }
}

/// Initializes a GLFW window, setting it as the current one.
fn init_glfw(
    width: u32,
    height: u32,
    title: &str,
    mode: WindowMode,
) -> (glfw::Window, Receiver<(f64, glfw::WindowEvent)>, glfw::Glfw) {
    println!("{}", glfw::get_version_string());

    let mut glfw_inst = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw_inst.window_hint(glfw::WindowHint::CenterCursor(true));
    // glfw_inst.window_hint(glfw::WindowHint::ContextVersion(4, 2));
    // glfw_inst.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    // glfw_inst.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    println!("{}", glfw::get_version_string());

    let (mut window, events) = match mode {
        WindowMode::Windowed => glfw_inst
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("Error creating GLFW window"),
        WindowMode::FullScreen(_) => glfw_inst
            .with_primary_monitor(|temp_glfw, m| {
                temp_glfw.create_window(
                    width,
                    height,
                    title,
                    m.map_or(glfw::WindowMode::Windowed, |m| {
                        glfw::WindowMode::FullScreen(m)
                    }),
                )
            })
            .expect("Error creating GLFW window"),
    };

    window.set_key_polling(true);
    window.set_cursor_mode(glfw::CursorMode::Normal);
    window.set_cursor_enter_polling(true);
    window.set_mouse_button_polling(true);
    window.set_cursor_pos_polling(true);
    window.make_current();

    return (window, events, glfw_inst);
}

/// Initializes the OpenGL functions. This must be run, or the program will
/// SegFault and crash.
pub fn init_gl(window: &mut glfw::Window) {
    unsafe {
        GL_IS_LOADED = true;
    }
    gl::load_with(|s| window.get_proc_address(s) as *const _);
    enable_depth_test();
}

fn enable_depth_test() {
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }
}

/// This function checks that OpenGL is loaded, and should be called at the beginning of
/// every unsafe OpenGL function call.
pub fn assert_gl_is_loaded() {
    unsafe {
        if !GL_IS_LOADED {
            println!("{GL_NOT_LOADED_MSG}");
            std::process::exit(0);
        }
    }
}
