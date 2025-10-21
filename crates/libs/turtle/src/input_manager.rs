pub mod mouse_state;
use mouse_state::*;
pub mod keyboard_state;
use keyboard_state::*;
use winit::event::{DeviceEvent, WindowEvent};

#[derive(Default)]
pub struct InputManager {
    keyboard: KeyboardState,
    mouse: MouseState
}

impl InputManager {
    pub fn handle_window_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::KeyboardInput { event, .. } => {
                self.keyboard.process(event);
            }
            WindowEvent::MouseInput { state, button, .. } => {
                self.mouse.process_button(*button, *state);
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse.process_move((position.x as f32, position.y as f32));
            }
            WindowEvent::MouseWheel { delta, .. } => {
                self.mouse.process_scroll(delta);
            }
            _ => {}
        }
    }

    pub fn handle_device_event(&mut self, event: &DeviceEvent) {
        if let DeviceEvent::MouseMotion { delta } = event {
            self.mouse.process_motion(delta);
        }
    }

    pub fn update(&mut self) {
        self.keyboard.update();
        self.mouse.update();
    }

    pub fn keyboard(&self) -> &KeyboardState {
        &self.keyboard
    }

    pub fn mouse(&self) -> &MouseState {
        &self.mouse
    }
}