use winit::{event::{ElementState, KeyEvent}, keyboard::{KeyCode, PhysicalKey}};
use std::collections::HashSet;

#[derive(Default)]
pub struct KeyboardState {
    pressed: HashSet<KeyCode>,
    just_pressed: HashSet<KeyCode>,
    just_released: HashSet<KeyCode>,
}

impl KeyboardState {
    pub fn process(&mut self, event: &KeyEvent) {
        if let PhysicalKey::Code(code) = event.physical_key {
            match event.state {
                ElementState::Pressed => {
                    if !self.pressed.contains(&code) {
                        self.just_pressed.insert(code);
                    }
                    self.pressed.insert(code);
                }
                ElementState::Released => {
                    self.just_released.insert(code);
                    self.pressed.remove(&code);
                }
            }
        }
    }

    pub fn update(&mut self) {
        self.just_pressed.clear();
        self.just_released.clear();
    }

    pub fn is_pressed(&self, key: KeyCode) -> bool {
        self.pressed.contains(&key)
    }

    pub fn just_pressed(&self, key: KeyCode) -> bool {
        self.just_pressed.contains(&key)
    }

    pub fn just_released(&self, key: KeyCode) -> bool {
        self.just_released.contains(&key)
    }
}