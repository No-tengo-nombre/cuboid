use cuboid::opengl::io::CameraController;
use glfw;

pub struct Controller {
    pub esc_pressed: bool,
    pub wireframe: bool,
}

impl CameraController for Controller {
    fn handle_key_event(
        &mut self,
        key: glfw::Key,
        _: glfw::Scancode,
        action: glfw::Action,
        _: glfw::Modifiers,
    ) {
        match action {
            glfw::Action::Press => match key {
                glfw::Key::Escape => self.esc_pressed = true,
                glfw::Key::Space => self.wireframe = !self.wireframe,
                _ => {}
            },
            glfw::Action::Release => match key {
                glfw::Key::Escape => self.esc_pressed = false,
                _ => {}
            },
            _ => {}
        }
    }
}

impl Controller {
    pub fn new() -> Controller {
        return Controller {
            esc_pressed: false,
            wireframe: false,
        };
    }
}
