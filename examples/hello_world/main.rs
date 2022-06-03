mod controller;

use gl;
use glfw;
use glfw::Context;

use controller::Controller;
use cuboid::components::{
    Material,
    Renderer3D,
    Shape
};
use cuboid::Shader;
use cuboid::io::CameraController;
use cuboid::utils::types;

const WINDOW_TITLE: &str = "Hello world triangle";

fn main() {
    // Initialization of the window
    let (mut window, events, mut glfw_instance) = cuboid::Window::new()
        .dimensions(1000, 1000)
        .title(WINDOW_TITLE)
        .windowed()
        .build();
    let mut renderer = Renderer3D::new();
    renderer.set_clear_color(0.0, 0.0, 0.0, 1.0);

    // Define a material
    let shader = Shader::new()
        .vertex("examples/hello_world/resources/shaders/test.vert")
        .fragment("examples/hello_world/resources/shaders/test.frag");

    let material = Material::new().shader(&shader);

    // Creation of the components
    let triangle_v: Vec<types::V6> = vec![
        [-0.75, -0.75, 0.0, 1.0, 0.0, 0.0],
        [0.75, -0.75, 0.0, 0.0, 1.0, 0.0],
        [0.0, 0.75, 0.0, 0.0, 0.0, 1.0],
    ];
    let triangle_i: Vec<u32> = vec![0, 1, 2];
    let triangle = Shape::new()
        .vertices(&triangle_v)
        .indices(&triangle_i)
        .material(&material)
        .layouts(&[0, 1])
        .build();

    // Add the item to the renderer
    renderer.add_item(&triangle);

    // Making a basic controller for the example
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
