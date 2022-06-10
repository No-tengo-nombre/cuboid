mod controller;

use gl;
use glfw;
use glfw::Context;

use controller::Controller;
use cuboid::opengl::components::{
    Camera,
    PerspectiveCamera,
    Material,
    Renderer3D,
    Shape,
    Texture2D,
};
use cuboid::opengl::{Shader, Window};
use cuboid::opengl::io::CameraController;
use cuboid::utils::{math::linalg, types};

const WINDOW_TITLE: &str = "Texture example";

fn main() {
    let mut delta;
    let mut fps;
    let mut prev_time = 0.0;
    let square_v: Vec<types::V8> = vec![
        [-10.0, -10.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0],
        [10.0, -10.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0],
        [10.0, 10.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
        [-10.0, 10.0, 0.0, 1.0, 1.0, 1.0, 0.0, 1.0],
    ];

    let square_i: Vec<u32> = vec![0, 1, 2, 3];

    let (mut window, events, mut glfw_instance) = Window::new()
        .width(1000)
        .height(1000)
        .title(WINDOW_TITLE)
        .windowed()
        .build();
    let mut renderer = Renderer3D::new();
    renderer.set_clear_color(0.0, 0.0, 0.0, 1.0);
    let shader = Shader::new()
        .vertex("examples/ogl_texture_example/resources/shaders/test.vert")
        .fragment("examples/ogl_texture_example/resources/shaders/test.frag");
    let material = Material::new().shader(&shader);

    let mut square = Shape::new()
        .vertices(&square_v)
        .indices(&square_i)
        .material(&material)
        .layouts(&[0, 1, 2])
        .usage(gl::DYNAMIC_DRAW)
        .texture(&Texture2D::from_path("examples/ogl_texture_example/resources/images/perroxd.png"))
        .build();

    renderer.add_item_with_mode(&square, gl::QUADS);
    let mut camera_pos = [0.0, 0.0, 20.0];
    let mut camera_dir = [0.0, 0.0, 1.0];
    let mut camera_up = [0.0, 1.0, 0.0];
    let mut camera_right;

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

    let cam_mov_speed = 0.01;
    let cam_rot_speed = 0.5;

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
        // println!("FPS : {}", fps);

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
            camera_dir = linalg::mat3_mul_v3(
                &linalg::rot_mat3(&camera_right, -cam_rot_speed),
                &camera_dir,
            );
            camera_up =
                linalg::mat3_mul_v3(&linalg::rot_mat3(&camera_right, -cam_rot_speed), &camera_up);
        }
        if controller.down_pressed {
            camera_dir =
                linalg::mat3_mul_v3(&linalg::rot_mat3(&camera_right, cam_rot_speed), &camera_dir);
            camera_up =
                linalg::mat3_mul_v3(&linalg::rot_mat3(&camera_right, cam_rot_speed), &camera_up);
        }
        if controller.left_pressed {
            camera_dir = linalg::mat3_mul_v3(&linalg::rot_mat3_y(-cam_rot_speed), &camera_dir);
            camera_up = linalg::mat3_mul_v3(&linalg::rot_mat3_y(-cam_rot_speed), &camera_up);
        }
        if controller.right_pressed {
            camera_dir = linalg::mat3_mul_v3(&linalg::rot_mat3_y(cam_rot_speed), &camera_dir);
            camera_up = linalg::mat3_mul_v3(&linalg::rot_mat3_y(cam_rot_speed), &camera_up);
        }
        camera.update(&camera_pos, &camera_dir, &camera_up);

        renderer.clear();
        renderer.render();
        window.swap_buffers();
        prev_time = time;
    }
}
