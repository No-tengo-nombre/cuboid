mod components;
mod core;
mod utils;

use glfw;
use glfw::{Action, Context, Key};

use crate::components::{material::Material, renderer2d::Renderer2D, shape::Shape};
use crate::core::shader::Shader;
use crate::utils::{init, math::linalg, types};

const WINDOW_TITLE: &str = "Test Window";

fn main() {
    let mut delta;
    let mut prev_time = 0.0;
    let mut triangle_v: Vec<types::V6> = vec![
        [-0.75, -0.75, 0.0, 1.0, 0.0, 0.0],
        [0.75, -0.75, 0.0, 0.0, 1.0, 0.0],
        [0.0, 0.75, 0.0, 0.0, 0.0, 1.0],
    ];

    let triangle_i: Vec<u32> = vec![0, 1, 2];

    let mut cube_v: Vec<types::V6> = vec![
        [-0.5, -0.5, -0.5, 0.0, 0.0, 0.0],
        [-0.5, -0.5, 0.5, 0.0, 0.0, 1.0],
        [-0.5, 0.5, -0.5, 0.0, 1.0, 0.0],
        [-0.5, 0.5, 0.5, 0.0, 1.0, 1.0],
        [0.5, -0.5, -0.5, 1.0, 0.0, 0.0],
        [0.5, -0.5, 0.5, 1.0, 0.0, 1.0],
        [0.5, 0.5, -0.5, 1.0, 1.0, 0.0],
        [0.5, 0.5, 0.5, 1.0, 1.0, 1.0],
    ];

    // Quads indices
    let cube_i: Vec<u32> = vec![
        0, 1, 3, 2,
        0, 4, 6, 2,
        4, 5, 7, 6,
        1, 5, 7, 3,
        0, 1, 5, 4,
        2, 3, 7, 6,
    ];

    let (mut window, events, mut glfw_instance) =
        init::init_glfw(1000, 1000, WINDOW_TITLE, glfw::WindowMode::Windowed);
    init::init_gl(&mut window);
    let mut renderer = Renderer2D::new();
    renderer.set_clear_color(0.0, 0.0, 0.0, 1.0);
    let shader = Shader::new(
        "test/resources/shaders/shader1/test.vert",
        "test/resources/shaders/shader1/test.frag",
    );
    let material = Material::new(&shader);

    let triangle = Shape::new(&triangle_v, &triangle_i, &material, &[0, 1]);
    let cube = Shape::new(&cube_v, &cube_i, &material, &[0, 1]);
    let mut wireframe = false;
    renderer.add_item_with_mode(&cube, 1, gl::QUADS);
    renderer.add_item(&triangle, 0);

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
        let time = glfw_instance.get_time() as f32;
        delta = time - prev_time;
        let r = ((5.0 * time) / 2.0 + 0.5).sin();
        let g = ((5.0 * time + 2.0 * 3.1415 / 3.0) / 2.0 + 0.5).sin();
        let b = ((5.0 * time - 2.0 * 3.1415 / 3.0) / 2.0 + 0.5).sin();
        // let r = 1.0;
        // let g = 1.0;
        // let b = 1.0;

        // triangle_v = linalg::mat6_mul3(&triangle_v, &linalg::rot_mat_x(45.0 * delta));
        // triangle_v = linalg::mat6_mul3(&triangle_v, &linalg::rot_mat_y(45.0 * delta));
        triangle_v = linalg::mat6_mul3(&triangle_v, &linalg::rot_mat_z(45.0 * delta));
        triangle.set_vertices(&triangle_v, &[0, 1]);

        cube_v = linalg::mat6_mul3(&cube_v, &linalg::rot_mat_x(45.0 * delta));
        cube_v = linalg::mat6_mul3(&cube_v, &linalg::rot_mat_y(45.0 * delta));
        // cube_v = linalg::mat6_mul3(&cube_v, &linalg::rot_mat_z(5.0 * delta));
        cube.set_vertices(&cube_v, &[0, 1]);

        material.get_shader().set_4f("timeColor", r, g, b, 1.0);
        renderer.clear();
        renderer.render();
        window.swap_buffers();
        prev_time = time;
    }
}
