mod core;

use glfw::{Action, Context, Key};

use crate::core::buffers;
use crate::core::materials;
use crate::core::utils::{
    types,
    init,
    wrappers,
};


const WINDOW_TITLE: &str = "Triangle: Draw Arrays";

const VERTICES: [types::Vertex3; 3] =
    [
        [-0.5, -0.5, 0.0],
        [0.5, -0.5, 0.0],
        [0.0, 0.5, 0.0],
    ];


fn main() {
    let (mut window, events, mut glfw) = init::init_glfw(800, 600, WINDOW_TITLE, glfw::WindowMode::Windowed);
    init::init_gl(&mut window);

    let vao = buffers::vao::new();
    let vbo = buffers::vbo::new(&VERTICES);
    vao.link_vbo(vbo, 0);
    
    wrappers::set_clear_color(0.0, 0.0, 0.0, 1.0);
    let shader = materials::shader::new(
        "test/resources/shaders/shader1/test.vert",
        "test/resources/shaders/shader1/test.frag",
    );
    shader.use_program();
    
    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true);
                },
                _ => {},
            }
        }
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
        window.swap_buffers();
    }
}