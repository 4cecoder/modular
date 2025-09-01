//! Window Input System
//!
//! Enhanced input system that integrates with window management.
//! Provides keyboard, mouse, and window event handling.

use minifb::Key;
use std::collections::HashSet;

/// Enhanced input state that includes window-specific inputs
#[derive(Debug, Clone)]
pub struct WindowInputState {
    pub keys_pressed: HashSet<Key>,
    pub keys_just_pressed: HashSet<Key>,
    pub keys_just_released: HashSet<Key>,
    pub mouse_position: (i32, i32),
    pub mouse_delta: (i32, i32),
    pub mouse_buttons: HashSet<MouseButton>,
    pub window_focused: bool,
    pub window_resized: Option<(usize, usize)>,
}

impl Default for WindowInputState {
    fn default() -> Self {
        Self {
            keys_pressed: HashSet::new(),
            keys_just_pressed: HashSet::new(),
            keys_just_released: HashSet::new(),
            mouse_position: (0, 0),
            mouse_delta: (0, 0),
            mouse_buttons: HashSet::new(),
            window_focused: true,
            window_resized: None,
        }
    }
}

impl WindowInputState {
    /// Check if a key is currently pressed
    pub fn is_key_pressed(&self, key: Key) -> bool {
        self.keys_pressed.contains(&key)
    }

    /// Check if a key was just pressed this frame
    pub fn is_key_just_pressed(&self, key: Key) -> bool {
        self.keys_just_pressed.contains(&key)
    }

    /// Check if a key was just released this frame
    pub fn is_key_just_released(&self, key: Key) -> bool {
        self.keys_just_released.contains(&key)
    }

    /// Check if a mouse button is pressed
    pub fn is_mouse_button_pressed(&self, button: MouseButton) -> bool {
        self.mouse_buttons.contains(&button)
    }

    /// Get mouse position as tuple
    pub fn mouse_pos(&self) -> (i32, i32) {
        self.mouse_position
    }

    /// Get mouse delta as tuple
    pub fn mouse_delta(&self) -> (i32, i32) {
        self.mouse_delta
    }

    /// Clear frame-specific input states
    pub fn clear_frame_state(&mut self) {
        self.keys_just_pressed.clear();
        self.keys_just_released.clear();
        self.mouse_delta = (0, 0);
        self.window_resized = None;
    }
}

/// Mouse button enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

/// Window input manager
pub struct WindowInputManager {
    current_state: WindowInputState,
    previous_keys: HashSet<Key>,
    previous_mouse_buttons: HashSet<MouseButton>,
    previous_mouse_pos: (i32, i32),
}

impl WindowInputManager {
    /// Create a new input manager
    pub fn new() -> Self {
        Self {
            current_state: WindowInputState::default(),
            previous_keys: HashSet::new(),
            previous_mouse_buttons: HashSet::new(),
            previous_mouse_pos: (0, 0),
        }
    }

    /// Update input state from window
    pub fn update(&mut self, window: &minifb::Window) {
        // Clear frame-specific state
        self.current_state.clear_frame_state();

        // Update keyboard state
        let mut current_keys = HashSet::new();

        // Check all possible keys (this is a simplified approach)
        // In a real implementation, you'd want to check all Key variants
        for key in &[
            Key::W, Key::A, Key::S, Key::D, Key::Space, Key::Escape, Key::Enter, Key::Q,
            Key::Up, Key::Down, Key::Left, Key::Right,
            Key::Key1, Key::Key2, Key::Key3,
            Key::NumPad1, Key::NumPad2, Key::NumPad3
        ] {
            if window.is_key_down(*key) {
                current_keys.insert(*key);
            }
        }

        // Determine just pressed and just released keys
        for key in &current_keys {
            if !self.previous_keys.contains(key) {
                self.current_state.keys_just_pressed.insert(*key);
            }
        }

        for key in &self.previous_keys {
            if !current_keys.contains(key) {
                self.current_state.keys_just_released.insert(*key);
            }
        }

        self.current_state.keys_pressed = current_keys.clone();
        self.previous_keys = current_keys;

        // Update mouse state (simplified - minifb has limited mouse support)
        // In a real implementation, you'd use a more advanced input library
        self.current_state.mouse_delta = (
            self.current_state.mouse_position.0 - self.previous_mouse_pos.0,
            self.current_state.mouse_position.1 - self.previous_mouse_pos.1,
        );
        self.previous_mouse_pos = self.current_state.mouse_position;

        // Update window state
        self.current_state.window_focused = true; // Simplified
    }

    /// Handle window resize event
    pub fn handle_resize(&mut self, width: usize, height: usize) {
        self.current_state.window_resized = Some((width, height));
    }

    /// Get current input state
    pub fn state(&self) -> &WindowInputState {
        &self.current_state
    }

    /// Get mutable access to input state
    pub fn state_mut(&mut self) -> &mut WindowInputState {
        &mut self.current_state
    }

    /// Check for quit condition
    pub fn should_quit(&self) -> bool {
        self.current_state.is_key_pressed(Key::Escape) ||
        self.current_state.is_key_pressed(Key::Q)
    }
}

/// Input mapping system for window inputs
pub struct WindowInputMapper {
    key_mappings: std::collections::HashMap<Key, String>,
    action_states: std::collections::HashMap<String, bool>,
}

impl WindowInputMapper {
    /// Create a new input mapper
    pub fn new() -> Self {
        let mut key_mappings = std::collections::HashMap::new();

        // Default key mappings
        key_mappings.insert(Key::W, "move_up".to_string());
        key_mappings.insert(Key::S, "move_down".to_string());
        key_mappings.insert(Key::A, "move_left".to_string());
        key_mappings.insert(Key::D, "move_right".to_string());
        key_mappings.insert(Key::Space, "action".to_string());
        key_mappings.insert(Key::Escape, "pause".to_string());
        key_mappings.insert(Key::Enter, "confirm".to_string());

        Self {
            key_mappings,
            action_states: std::collections::HashMap::new(),
        }
    }

    /// Update action states based on input state
    pub fn update(&mut self, input_state: &WindowInputState) {
        for (key, action) in &self.key_mappings {
            let is_pressed = input_state.is_key_pressed(*key);
            self.action_states.insert(action.clone(), is_pressed);
        }
    }

    /// Check if an action is active
    pub fn is_action_active(&self, action: &str) -> bool {
        self.action_states.get(action).copied().unwrap_or(false)
    }

    /// Map a key to an action
    pub fn map_key(&mut self, key: Key, action: String) {
        self.key_mappings.insert(key, action);
    }

    /// Remove a key mapping
    pub fn unmap_key(&mut self, key: Key) {
        self.key_mappings.remove(&key);
    }

    /// Get all active actions
    pub fn get_active_actions(&self) -> Vec<String> {
        self.action_states.iter()
            .filter(|(_, &active)| active)
            .map(|(action, _)| action.clone())
            .collect()
    }
}

/// Game controller abstraction for window inputs
pub struct WindowGameController {
    input_mapper: WindowInputMapper,
    deadzone: f32,
}

impl WindowGameController {
    /// Create a new game controller
    pub fn new() -> Self {
        Self {
            input_mapper: WindowInputMapper::new(),
            deadzone: 0.1,
        }
    }

    /// Update controller state
    pub fn update(&mut self, input_state: &WindowInputState) {
        self.input_mapper.update(input_state);
    }

    /// Get movement vector from input
    pub fn get_movement_vector(&self) -> (f32, f32) {
        let mut x: f32 = 0.0;
        let mut y: f32 = 0.0;

        if self.input_mapper.is_action_active("move_left") { x -= 1.0; }
        if self.input_mapper.is_action_active("move_right") { x += 1.0; }
        if self.input_mapper.is_action_active("move_up") { y -= 1.0; }
        if self.input_mapper.is_action_active("move_down") { y += 1.0; }

        // Apply deadzone
        if x.abs() < self.deadzone { x = 0.0; }
        if y.abs() < self.deadzone { y = 0.0; }

        (x, y)
    }

    /// Check if action button is pressed
    pub fn is_action_pressed(&self) -> bool {
        self.input_mapper.is_action_active("action")
    }

    /// Check if pause button is pressed
    pub fn is_pause_pressed(&self) -> bool {
        self.input_mapper.is_action_active("pause")
    }

    /// Check if confirm button is pressed
    pub fn is_confirm_pressed(&self) -> bool {
        self.input_mapper.is_action_active("confirm")
    }

    /// Get input mapper for customization
    pub fn input_mapper(&mut self) -> &mut WindowInputMapper {
        &mut self.input_mapper
    }
}