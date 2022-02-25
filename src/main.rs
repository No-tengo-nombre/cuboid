mod components;
mod core;
mod utils;

use glfw;
use glfw::{Action, Context, Key};

use crate::components::{camera::Camera, camera::OrthoCamera, camera::PerspectiveCamera, material::Material, renderer3d::Renderer3D, shape::Shape};
use crate::core::shader::Shader;
use crate::utils::{conversions, init, math::linalg, types};

const WINDOW_TITLE: &str = "Test Window";

fn main() {
    let mut delta;
    let mut fps;
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
        // [100.0, 100.0, 100.0, 0.0, 0.0, 0.0],
        // [100.0, 100.0, 200.0, 0.0, 0.0, 1.0],
        // [100.0, 200.0, 100.0, 0.0, 1.0, 0.0],
        // [100.0, 200.0, 200.0, 0.0, 1.0, 1.0],
        // [200.0, 100.0, 100.0, 1.0, 0.0, 0.0],
        // [200.0, 100.0, 200.0, 1.0, 0.0, 1.0],
        // [200.0, 200.0, 100.0, 1.0, 1.0, 0.0],
        // [200.0, 200.0, 200.0, 1.0, 1.0, 1.0],
        // [100.0, 100.0, 0.0, 0.0, 0.0, 0.0],
        // [100.0, 100.0, 0.0, 0.0, 0.0, 1.0],
        // [100.0, 200.0, 0.0, 0.0, 1.0, 0.0],
        // [100.0, 200.0, 0.0, 0.0, 1.0, 1.0],
        // [200.0, 100.0, 0.0, 1.0, 0.0, 0.0],
        // [200.0, 100.0, 0.0, 1.0, 0.0, 1.0],
        // [200.0, 200.0, 0.0, 1.0, 1.0, 0.0],
        // [200.0, 200.0, 0.0, 1.0, 1.0, 1.0],
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
    let mut renderer = Renderer3D::new();
    renderer.set_clear_color(0.0, 0.0, 0.0, 1.0);
    let shader = Shader::new(
        "test/resources/shaders/shader1/test.vert",
        "test/resources/shaders/shader1/test.frag",
    );
    let material = Material::new(&shader);

    let triangle = Shape::new_with_usage(&triangle_v, &triangle_i, &material, &[0, 1], gl::DYNAMIC_DRAW);
    let cube = Shape::new_with_usage(&cube_v, &cube_i, &material, &[0, 1], gl::DYNAMIC_DRAW);
    let mut wireframe = false;
    renderer.add_item_with_mode(&cube, gl::QUADS);
    renderer.add_item(&triangle);
    let mut camera_pos = [0.0, 0.0, 5.0];
    let speed = 0.05;
    
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
                glfw::WindowEvent::Key(Key::W, _, Action::Press, _) => {
                    camera_pos[1] += speed;
                }
                glfw::WindowEvent::Key(Key::S, _, Action::Press, _) => {
                    camera_pos[1] += -speed;
                }
                glfw::WindowEvent::Key(Key::D, _, Action::Press, _) => {
                    camera_pos[0] += speed;
                }
                glfw::WindowEvent::Key(Key::A, _, Action::Press, _) => {
                    camera_pos[0] += -speed;
                }
                _ => {}
            }
        }
        let time = glfw_instance.get_time() as f32;
        delta = time - prev_time;
        fps = 1.0 / delta;
        println!("Δt: {} ms  |  FPS: {}", delta * 1000.0, fps);
        // let r = ((2.5 * time) / 2.0 + 0.5).sin();
        // let g = ((2.5 * time + 2.0 * 3.1415 / 3.0) / 2.0 + 0.5).sin();
        // let b = ((2.5 * time - 2.0 * 3.1415 / 3.0) / 2.0 + 0.5).sin();
        let r = 1.0;
        let g = 1.0;
        let b = 1.0;

        let rot_speed = 10.0;

        // triangle_v = linalg::mat6_mul3(&triangle_v, &linalg::rot_mat_x(rot_speed * delta));
        // triangle_v = linalg::mat6_mul3(&triangle_v, &linalg::rot_mat_y(rot_speed * delta));
        // triangle_v = linalg::mat6_mul3(&triangle_v, &linalg::rot_mat_z(rot_speed * delta));
        // triangle.set_vertices(&triangle_v, &[0, 1]);

        // cube_v = linalg::mat6_mul3(&cube_v, &linalg::rot_mat_x(rot_speed * delta));
        // cube_v = linalg::mat6_mul3(&cube_v, &linalg::rot_mat_y(rot_speed * delta));
        // cube_v = linalg::mat6_mul3(&cube_v, &linalg::rot_mat_z(rot_speed * delta));
        // cube.set_vertices(&cube_v, &[0, 1]);

        let camera = OrthoCamera::new(
            &camera_pos,
            &[0.0, 0.0, 1.0],
            &[0.0, 1.0, 0.0],
        );

        material.get_shader().set_4f("timeColor", r, g, b, 1.0);
        material.get_shader().set_matrix4fv("view", &conversions::vec4_to_v4(&camera.look_at()));
        renderer.clear();
        renderer.render();
        window.swap_buffers();
        prev_time = time;
    }
}
