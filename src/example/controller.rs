use crate::io::cam_controller::CameraController;
use glfw;

pub struct Controller {
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
                glfw::Key::Space => self.wireframe = !self.wireframe,
                _ => {}
            },
            glfw::Action::Release => match key {
                glfw::Key::W => self.w_pressed = false,
                glfw::Key::A => self.a_pressed = false,
                glfw::Key::S => self.s_pressed = false,
                glfw::Key::D => self.d_pressed = false,
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
