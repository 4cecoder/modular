//! Window management system
//!
//! Provides cross-platform window creation and management.
//! Abstracts away platform-specific window handling.

use minifb::{Key, Window, WindowOptions};
use std::collections::HashSet;

/// Window configuration
#[derive(Debug, Clone)]
pub struct WindowConfig {
    pub title: String,
    pub width: usize,
    pub height: usize,
    pub resizable: bool,
    pub vsync: bool,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: "Game Window".to_string(),
            width: 800,
            height: 600,
            resizable: true,
            vsync: true,
        }
    }
}

/// Window manager for handling window lifecycle
pub struct WindowManager {
    window: Window,
    config: WindowConfig,
    should_close: bool,
    // Store previous key states to detect presses and releases
    previous_keys: HashSet<Key>,
}

impl WindowManager {
    /// Create a new window with the given configuration
    pub fn new(config: WindowConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let window = Window::new(
            &config.title,
            config.width,
            config.height,
            WindowOptions {
                resize: config.resizable,
                ..WindowOptions::default()
            },
        )?;

        Ok(Self {
            window,
            config,
            should_close: false,
            previous_keys: HashSet::new(),
        })
    }

    /// Check if the window should close
    pub fn should_close(&self) -> bool {
        !self.window.is_open() || self.should_close
    }

    /// Get the window dimensions
    pub fn dimensions(&self) -> (usize, usize) {
        (self.config.width, self.config.height)
    }

    /// Set the window title
    pub fn set_title(&mut self, title: &str) {
        self.window.set_title(title);
    }

    /// Update the window and collect events (call this each frame)
    pub fn update(&mut self) -> Vec<WindowEvent> {
        let mut events = Vec::new();

        // Pump the minifb event queue so input states and window events are updated.
        // Pump the minifb event queue so input states and window events are updated.
        self.window.update();

        if !self.window.is_open() {
            self.should_close = true;
            events.push(WindowEvent::WindowClosed);
        }

        // Handle key presses and releases
        let current_keys: HashSet<Key> = self.window.get_keys().into_iter().collect();

        for key in current_keys.difference(&self.previous_keys) {
            events.push(WindowEvent::KeyPressed(*key));
        }

        for key in self.previous_keys.difference(&current_keys) {
            events.push(WindowEvent::KeyReleased(*key));
        }

        self.previous_keys = current_keys;

        // Check for window resize
        let (current_width, current_height) = self.window.get_size();
        if current_width != self.config.width || current_height != self.config.height {
            self.config.width = current_width;
            self.config.height = current_height;
            events.push(WindowEvent::WindowResized {
                width: current_width,
                height: current_height,
            });
        }

        events
    }

    /// Get mutable reference to the underlying window
    pub fn window(&mut self) -> &mut Window {
        &mut self.window
    }

    /// Get immutable reference to the underlying window
    pub fn window_ref(&self) -> &Window {
        &self.window
    }
}

/// Window event handling
#[derive(Debug)]
pub enum WindowEvent {
    KeyPressed(Key),
    KeyReleased(Key),
    WindowClosed,
    WindowResized { width: usize, height: usize },
}

// The WindowEvents struct and its impl are no longer needed as update() now returns Vec<WindowEvent>
// and the responsibility of iterating events is shifted to the caller.
