use winit::event::{ElementState, MouseButton, MouseScrollDelta};


#[derive(Default)]
pub struct MouseState {
    pub position: (f32, f32),
    pub delta: (f32, f32),
    pub scroll: f32,
    buttons: [bool; 3]
}

impl MouseState {
    pub fn process_button(&mut self, button: MouseButton, state: ElementState) {
        let pressed = matches!(state, ElementState::Pressed);
        match button {
            MouseButton::Left => self.buttons[0] = pressed,
            MouseButton::Right => self.buttons[1] = pressed,
            MouseButton::Middle => self.buttons[2] = pressed,
            _ => {}
        }
    }

    pub fn process_move(&mut self, position: (f32, f32)) {
        self.position = position;
    }

    pub fn process_motion(&mut self, delta: &(f64, f64)) {
        self.delta = (delta.0 as f32, delta.1 as f32);
    }

    pub fn process_scroll(&mut self, delta: &MouseScrollDelta) {
        self.scroll = match delta {
            MouseScrollDelta::LineDelta(_, y) => *y,
            MouseScrollDelta::PixelDelta(p) => p.y as f32,
        }
    }

    pub fn update(&mut self) {
        self.delta = (0.0, 0.0);
        self.scroll = 0.0;
    }

    pub fn is_pressed(&self, button: MouseButton) -> bool {
        match button {
            MouseButton::Left => self.buttons[0],
            MouseButton::Right => self.buttons[1],
            MouseButton::Middle => self.buttons[2],
            _ => false
        }
    }
}