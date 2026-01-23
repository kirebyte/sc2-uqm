//! Input handling system
//!
//! Manages keyboard, mouse, and gamepad input.

use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Input state tracker
#[derive(Debug, Clone, Default)]
pub struct InputState {
    /// Currently pressed keys
    pub pressed_keys: HashSet<Key>,
    /// Mouse position (x, y)
    pub mouse_position: (f32, f32),
    /// Mouse buttons currently pressed
    pub mouse_buttons: HashSet<MouseButton>,
}

impl InputState {
    /// Create a new input state
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if a key is currently pressed
    #[must_use]
    pub fn is_key_pressed(&self, key: Key) -> bool {
        self.pressed_keys.contains(&key)
    }

    /// Check if a mouse button is pressed
    #[must_use]
    pub fn is_mouse_button_pressed(&self, button: MouseButton) -> bool {
        self.mouse_buttons.contains(&button)
    }

    /// Press a key
    pub fn press_key(&mut self, key: Key) {
        self.pressed_keys.insert(key);
    }

    /// Release a key
    pub fn release_key(&mut self, key: Key) {
        self.pressed_keys.remove(&key);
    }

    /// Update mouse position
    pub fn update_mouse_position(&mut self, x: f32, y: f32) {
        self.mouse_position = (x, y);
    }
}

/// Keyboard key enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Key {
    // Movement keys
    W,
    A,
    S,
    D,
    Up,
    Down,
    Left,
    Right,
    
    // Action keys
    Space,
    Enter,
    Escape,
    Tab,
    Shift,
    Control,
    
    // Number keys
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    
    // Function keys
    F1,
    F2,
    F3,
    F12,
}

/// Mouse button enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MouseButton {
    /// Left mouse button
    Left,
    /// Right mouse button
    Right,
    /// Middle mouse button
    Middle,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_state_creation() {
        let state = InputState::new();
        assert!(!state.is_key_pressed(Key::W));
    }

    #[test]
    fn test_key_press() {
        let mut state = InputState::new();
        state.press_key(Key::W);
        assert!(state.is_key_pressed(Key::W));
    }

    #[test]
    fn test_key_release() {
        let mut state = InputState::new();
        state.press_key(Key::W);
        state.release_key(Key::W);
        assert!(!state.is_key_pressed(Key::W));
    }
}
