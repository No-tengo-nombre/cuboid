mod components;
mod core;
mod utils;

use glfw;
use glfw::{Action, Context, Key};

use crate::utils::{init, types};

const WINDOW_TITLE: &str = "Test Window";

fn main() {
        let triangle_v: Vec<types::V6> = vec![
        [-0.5, -0.5, 0.0, 1.0, 0.0, 0.0],
        [-0.5, 0.5, 0.0, 0.0, 1.0, 0.0],
        [0.5, 0.5, 0.0, 0.0, 0.0, 1.0],
        [0.5, -0.5, 0.0, 1.0, 1.0, 1.0],
    ];

    let triangle_i: Vec<u32> = vec![0, 1, 2, 0, 2, 3];

    let (mut window, events, mut glfw_instance) =
        init::init_glfw(800, 600, WINDOW_TITLE, glfw::WindowMode::Windowed);
    init::init_gl(&mut window);
    components::renderer::set_clear_color(0.0, 0.0, 0.0, 1.0);
    let shader = core::shader::new(
        "test/resources/shaders/shader1/test.vert",
        "test/resources/shaders/shader1/test.frag",
    );
    let material = components::material::new(&shader);

    let triangle = components::shape::new(&triangle_v, &triangle_i, &material);
    let mut wireframe = false;
    while !window.should_close() {
        glfw_instance.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true);
                }
                glfw::WindowEvent::Key(Key::Space, _, Action::Press, _) => {
                    if !wireframe {
                        unsafe {
                            gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
                        }
                    } else {
                        unsafe {
                            gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
                        }
                    }
                    wireframe = !wireframe;
                }
                _ => {}
            }
        }
        components::renderer::clear();
        components::renderer::draw(&triangle);

        let r = ((5.0 * (glfw_instance.get_time() as f32)) / 2.0 + 0.5).sin();
        let g = ((5.0 * (glfw_instance.get_time() as f32) + 2.0 * 3.1415 / 3.0) / 2.0 + 0.5).sin();
        let b = ((5.0 * (glfw_instance.get_time() as f32) - 2.0 * 3.1415 / 3.0) / 2.0 + 0.5).sin();

        material.get_shader().set_4f("timeColor", r, g, b, 1.0);

        window.swap_buffers();
    }

    triangle.del();
}
