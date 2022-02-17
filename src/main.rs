mod core;

use beryllium::*;
use ogl33::*;

use crate::core::buffers;
use crate::core::materials;
use crate::core::utils::{
    types,
    init,
    gl,
};


const WINDOW_TITLE: &str = "Triangle: Draw Arrays";

const VERTICES: [types::Vertex; 3] =
    [
        [-0.5, -0.5, 0.0],
        [0.5, -0.5, 0.0],
        [0.0, 0.5, 0.0],
    ];


fn main() {
    let (sdl, win) = init::set_up_window(WINDOW_TITLE);
    init::load_gl(&win);

    let vao = buffers::vao::new();
    let vbo = buffers::vbo::new(&VERTICES);
    vao.link_vbo(vbo, 0);
    
    gl::set_clear_color(0.0, 0.0, 0.0, 1.0);
    let shader = materials::shader::new(
        "test/resources/shaders/shader1/test.vert",
        "test/resources/shaders/shader1/test.frag",
    );
    shader.use_program();

    'main_loop: loop {
        // handle events this frame
        while let Some(event) = sdl.poll_events().and_then(Result::ok) {
            match event {
                Event::Quit(_) => break 'main_loop,
                _ => (),
            }
        }
        // now the events are clear.

        // here's where we could change the world state if we had some.

        // and then draw!
        unsafe {
            glClear(GL_COLOR_BUFFER_BIT);
            glDrawArrays(GL_TRIANGLES, 0, 3);
        }
        win.swap_window();
    }
}