use crate::opengl::Window;
use glfw;

pub trait CameraController {
    fn poll_window_events(&mut self, window: &mut Window) {
        for (_, event) in glfw::flush_messages(window.poll_events()) {
            self.handle_event(event);
        }
    }

    fn handle_event(&mut self, event: glfw::WindowEvent) {
        match event {
            glfw::WindowEvent::Key(key, scancode, action, modifiers) => {
                self.handle_key_event(key, scancode, action, modifiers)
            }
            glfw::WindowEvent::MouseButton(button, action, modifiers) => {
                self.handle_mouse_button_event(button, action, modifiers)
            }
            glfw::WindowEvent::CursorPos(x, y) => self.handle_cursor_pos_event(x, y),
            glfw::WindowEvent::CursorEnter(enter) => self.handle_cursor_enter_event(enter),
            _ => {}
        }
    }

    fn handle_key_event(
        &mut self,
        _key: glfw::Key,
        _scancode: glfw::Scancode,
        _action: glfw::Action,
        _modifiers: glfw::Modifiers,
    ) {
    }

    fn handle_mouse_button_event(
        &mut self,
        _mouse_button: glfw::MouseButton,
        _action: glfw::Action,
        _modifiers: glfw::Modifiers,
    ) {
    }

    fn handle_cursor_pos_event(&mut self, _x: f64, _y: f64) {}

    fn handle_cursor_enter_event(&mut self, _enter: bool) {}
}

///////////////////////////////////////////////////////////////////////////////////////////////////
//|================================| Base Camera Controller |===================================|//
///////////////////////////////////////////////////////////////////////////////////////////////////
