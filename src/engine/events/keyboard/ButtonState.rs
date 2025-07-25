use std::collections::HashMap;
use winit::event::ElementState;
use winit::keyboard::{KeyCode, PhysicalKey};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ButtonState {
    Up,
    Pressed,
    Held,
    Released,
}

pub type Key = PhysicalKey;

#[derive(Default)]
pub struct InputManager {
    key_states: HashMap<Key, ButtonState>,
    just_released: Vec<Key>,
    just_pressed: Vec<Key>,
}

impl InputManager {
    pub fn update(&mut self) {
        for (_, state) in self.key_states.iter_mut() {
            *state = match *state {
                ButtonState::Pressed => ButtonState::Held,
                ButtonState::Released => ButtonState::Up,
                other => other,
            };
        }

        for key in self.just_released.drain(..) {
            self.key_states.insert(key, ButtonState::Released);
        }


        for key in self.just_pressed.drain(..) {
            self.key_states.insert(key, ButtonState::Pressed);
        }
    }

    pub fn handle_key(&mut self, key: Key, state: ElementState) {
        match state {
            ElementState::Pressed => {
                match self.key_states.get(&key) {
                    Some(ButtonState::Held | ButtonState::Pressed) => {},
                    _ => self.just_pressed.push(key),
                }
            }
            ElementState::Released => {
                self.just_released.push(key);
                // self.key_states.insert(key, ButtonState::Released);
            }
        }
    }

    pub fn is_pressed(&self, key: Key) -> bool {
        self.get_state(key) == ButtonState::Pressed
    }

    pub fn is_held(&self, key: Key) -> bool {
        self.get_state(key) == ButtonState::Held
    }

    pub fn is_released(&self, key: Key) -> bool {
        self.get_state(key) == ButtonState::Released
    }

    pub fn is_up(&self, key: Key) -> bool {
        self.get_state(key) == ButtonState::Up
    }

    pub fn get_state(&self, key: Key) -> ButtonState {
        self.key_states.get(&key).copied().unwrap_or(ButtonState::Up)
    }
}