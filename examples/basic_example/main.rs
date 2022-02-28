// mod components;
// mod core;
// mod example;
// mod io;
// mod utils;


use glfw;
use glfw::{Action, Context, Key};

use shiny::components::{
    camera::Camera,
    camera::OrthoCamera,
    camera::PerspectiveCamera,
    material::Material,
    renderer3d::Renderer3D,
    shape::Shape
};
use shiny::core::shader::Shader;
use shiny::io::cam_controller::CameraController;
use shiny::utils::{conversions, init, math::linalg, types};

const WINDOW_TITLE: &str = "Basic Example";

///////////////////////////////////////////////////////////////////////////////////////////////////
//|================================| Camera controller |========================================|//
///////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Controller {
    pub esc_pressed: bool,
    pub wireframe: bool,
    pub w_pressed: bool,
    pub s_pressed: bool,
    pub a_pressed: bool,
    pub d_pressed: bool,
    pub up_pressed: bool,
    pub down_pressed: bool,
    pub left_pressed: bool,
    pub right_pressed: bool,
    pub l_button_pressed: bool,
    pub r_button_pressed: bool,
}

impl CameraController for Controller {
    fn handle_key_event(
        &mut self,
        key: glfw::Key,
        scancode: glfw::Scancode,
        action: glfw::Action,
        modifiers: glfw::Modifiers,
    ) {
        match action {
            glfw::Action::Press => match key {
                glfw::Key::W => self.w_pressed = true,
                glfw::Key::A => self.a_pressed = true,
                glfw::Key::S => self.s_pressed = true,
                glfw::Key::D => self.d_pressed = true,
                glfw::Key::Up => self.up_pressed = true,
                glfw::Key::Left => self.left_pressed = true,
                glfw::Key::Down => self.down_pressed = true,
                glfw::Key::Right => self.right_pressed = true,
                glfw::Key::Escape => self.esc_pressed = true,
                glfw::Key::Space => self.wireframe = !self.wireframe,
                _ => {}
            },
            glfw::Action::Release => match key {
                glfw::Key::W => self.w_pressed = false,
                glfw::Key::A => self.a_pressed = false,
                glfw::Key::S => self.s_pressed = false,
                glfw::Key::D => self.d_pressed = false,
                glfw::Key::Up => self.up_pressed = false,
                glfw::Key::Left => self.left_pressed = false,
                glfw::Key::Down => self.down_pressed = false,
                glfw::Key::Right => self.right_pressed = false,
                glfw::Key::Escape => self.esc_pressed = false,
                _ => {}
            },
            _ => {}
        }
    }
    fn handle_mouse_button_event(
        &mut self,
        mouse_button: glfw::MouseButton,
        action: glfw::Action,
        modifiers: glfw::Modifiers,
    ) {
    }
}

impl Controller {
    pub fn new() -> Controller {
        return Controller {
            esc_pressed: false,
            wireframe: false,
            w_pressed: false,
            s_pressed: false,
            a_pressed: false,
            d_pressed: false,
            up_pressed: false,
            down_pressed: false,
            left_pressed: false,
            right_pressed: false,
            l_button_pressed: false,
            r_button_pressed: false,
        };
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
//|===================================| Main function |=========================================|//
///////////////////////////////////////////////////////////////////////////////////////////////////

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

    let triangle = Shape::new_with_usage(
        &triangle_v,
        &triangle_i,
        &material,
        &[0, 1],
        gl::DYNAMIC_DRAW,
    );
    let cube = Shape::new_with_usage(&cube_v, &cube_i, &material, &[0, 1], gl::DYNAMIC_DRAW);
    renderer.add_item_with_mode(&cube, gl::QUADS);
    renderer.add_item(&triangle);
    let mut camera_pos = [0.0, 0.0, 2.0];
    let mut camera_dir = [0.0, 0.0, 1.0];
    let mut camera_up = [0.0, 1.0, 0.0];
    let mut camera_right;

    let cam_mov_speed = 0.005;
    let cam_rot_speed = 1.0;

    let mut wireframe = false;
    let mut controller = Controller::new();

    while !window.should_close() {
        controller.poll_window_events(&mut glfw_instance, &events);
        if controller.esc_pressed {
            window.set_should_close(true);
        }
        let time = glfw_instance.get_time() as f32;
        delta = time - prev_time;
        fps = 1.0 / delta;

        camera_right = linalg::normalize_v3(&linalg::cross_v3(&camera_dir, &camera_up));

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

        // Camera movement control
        if controller.w_pressed {
            camera_pos[0] -= cam_mov_speed * camera_dir[0];
            camera_pos[1] -= cam_mov_speed * camera_dir[1];
            camera_pos[2] -= cam_mov_speed * camera_dir[2];
        }
        if controller.s_pressed {
            camera_pos[0] += cam_mov_speed * camera_dir[0];
            camera_pos[1] += cam_mov_speed * camera_dir[1];
            camera_pos[2] += cam_mov_speed * camera_dir[2];
        }
        if controller.a_pressed {
            camera_pos[0] -= cam_mov_speed * camera_right[0];
            camera_pos[1] -= cam_mov_speed * camera_right[1];
            camera_pos[2] -= cam_mov_speed * camera_right[2];
        }
        if controller.d_pressed {
            camera_pos[0] += cam_mov_speed * camera_right[0];
            camera_pos[1] += cam_mov_speed * camera_right[1];
            camera_pos[2] += cam_mov_speed * camera_right[2];
        }

        // Camera rotation control
        if controller.up_pressed {
            camera_dir = linalg::mat3_mul_v3(&linalg::rot_mat_x(cam_rot_speed), &camera_dir);
            camera_up = linalg::mat3_mul_v3(&linalg::rot_mat_x(cam_rot_speed), &camera_up);
        }
        if controller.down_pressed {
            camera_dir = linalg::mat3_mul_v3(&linalg::rot_mat_x(-cam_rot_speed), &camera_dir);
            camera_up = linalg::mat3_mul_v3(&linalg::rot_mat_x(-cam_rot_speed), &camera_up);
        }
        if controller.left_pressed {
            camera_dir = linalg::mat3_mul_v3(&linalg::rot_mat_y(-cam_rot_speed), &camera_dir);
            camera_up = linalg::mat3_mul_v3(&linalg::rot_mat_y(-cam_rot_speed), &camera_up);
        }
        if controller.right_pressed {
            camera_dir = linalg::mat3_mul_v3(&linalg::rot_mat_y(cam_rot_speed), &camera_dir);
            camera_up = linalg::mat3_mul_v3(&linalg::rot_mat_y(cam_rot_speed), &camera_up);
        }

        // Random functionality for the mouse buttons
        if controller.r_button_pressed {
            println!("Δt: {} ms  |  FPS: {}", delta * 1000.0, fps);
        }
        if controller.l_button_pressed {
            println!("LEFT");
        }

        let r = ((2.5 * time) / 2.0 + 0.5).sin();
        let g = ((2.5 * time + 2.0 * 3.1415 / 3.0) / 2.0 + 0.5).sin();
        let b = ((2.5 * time - 2.0 * 3.1415 / 3.0) / 2.0 + 0.5).sin();
        // let r = 1.0;
        // let g = 1.0;
        // let b = 1.0;

        let rot_speed = 10.0;

        // triangle_v = linalg::mat6_mul3(&triangle_v, &linalg::rot_mat_x(rot_speed * delta));
        // triangle_v = linalg::mat6_mul3(&triangle_v, &linalg::rot_mat_y(rot_speed * delta));
        // triangle_v = linalg::mat6_mul3(&triangle_v, &linalg::rot_mat_z(rot_speed * delta));
        // triangle.set_vertices(&triangle_v, &[0, 1]);

        // cube_v = linalg::mat6_mul3(&cube_v, &linalg::rot_mat_x(rot_speed * delta));
        // cube_v = linalg::mat6_mul3(&cube_v, &linalg::rot_mat_y(rot_speed * delta));
        // cube_v = linalg::mat6_mul3(&cube_v, &linalg::rot_mat_z(rot_speed * delta));
        // cube.set_vertices(&cube_v, &[0, 1]);

        // TODO: Change these magic numbers
        let camera = OrthoCamera::new(
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

        // TODO: Make materials handle these uniforms.
        material.get_shader().set_4f("timeColor", r, g, b, 1.0);
        material
            .get_shader()
            .set_matrix4fv("view", &conversions::vec4_to_v4(&camera.look_at()));
        renderer.clear();
        renderer.render();
        window.swap_buffers();
        prev_time = time;
    }
}
