mod controller;

use gl;
use glfw;
use glfw::Context;

use controller::Controller;
use cuboid::components::{
    Camera,
    OrthoCamera,
    PerspectiveCamera,
    Material,
    Renderer3D,
    Shape
};
use cuboid::core::Shader;
use cuboid::io::CameraController;
use cuboid::utils::{init, math::linalg, types};

const WINDOW_TITLE: &str = "Hello world triangle";

fn main() {
    let mut triangle_v: Vec<types::V6> = vec![
        [-0.75, -0.75, 0.0, 1.0, 0.0, 0.0],
        [0.75, -0.75, 0.0, 0.0, 1.0, 0.0],
        [0.0, 0.75, 0.0, 0.0, 0.0, 1.0],
    ];

    let triangle_i: Vec<u32> = vec![0, 1, 2];

    let (mut window, events, mut glfw_instance) =
        init::init_glfw(1000, 1000, WINDOW_TITLE, glfw::WindowMode::Windowed);
    init::init_gl(&mut window);
    let mut renderer = Renderer3D::new();
    renderer.set_clear_color(0.0, 0.0, 0.0, 1.0);
    let shader = Shader::new(
        "examples/hello_world/resources/shaders/test.vert",
        "examples/hello_world/resources/shaders/test.frag",
    );
    let material = Material::new(&shader);

    let triangle = Shape::new_with_usage(
        &triangle_v,
        &triangle_i,
        &material,
        &[0, 1],
        gl::STATIC_DRAW,
    );
    renderer.add_item(&triangle);
    let mut camera_pos = [0.0, 0.0, 20.0];
    let mut camera_dir = [0.0, 0.0, 1.0];
    let mut camera_up = [0.0, 1.0, 0.0];

    let mut camera = PerspectiveCamera::new(
        &camera_pos,
        &camera_dir,
        &camera_up,
        -1.0,
        1.0,
        -1.0,
        1.0,
        1.0,
        1000.0,
    );

    let cam_mov_speed = 0.1;
    let cam_rot_speed = 1.0;

    let mut wireframe = false;
    let mut controller = Controller::new();

    while !window.should_close() {
        controller.poll_window_events(&mut glfw_instance, &events);
        if controller.esc_pressed {
            window.set_should_close(true);
        }

        if wireframe != controller.wireframe {
            if controller.wireframe {
                renderer.set_polygon_mode(gl::FRONT_AND_BACK, gl::LINE);
                println!("LINE")
            } else {
                renderer.set_polygon_mode(gl::FRONT_AND_BACK, gl::FILL);
                println!("FILL")
            }
            wireframe = controller.wireframe;
        }

        renderer.clear();
        renderer.render();
        window.swap_buffers();
    }
}
