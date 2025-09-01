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

    /// Check if a key is currently pressed
    pub fn is_key_pressed(&self, key: Key) -> bool {
        self.window.is_key_down(key)
    }

    /// Get all currently pressed keys
    pub fn get_pressed_keys(&self) -> HashSet<Key> {
        // Note: minifb doesn't provide a way to enumerate all pressed keys
        // This would need to be implemented differently or use a different library
        HashSet::new()
    }

    /// Update the window (call this each frame)
    pub fn update(&mut self) {
        // Pump the minifb event queue so input states and window events are updated.
        // `update` returns whether the window is still open. We ignore the return
        // value here and use `is_open`/`should_close` for state.
        let _ = self.window.update();

        // Handle simple key-based quit checks after events are processed.
        if self.window.is_key_down(Key::Escape) || self.window.is_key_down(Key::Q) {
            self.should_close = true;
        }
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
pub enum WindowEvent {
    KeyPressed(Key),
    KeyReleased(Key),
    WindowClosed,
    WindowResized { width: usize, height: usize },
}

/// Window event iterator (placeholder for more advanced event handling)
pub struct WindowEvents<'a> {
    #[allow(dead_code)]
    window: &'a Window,
}

impl<'a> WindowEvents<'a> {
    pub fn new(window: &'a Window) -> Self {
        Self { window }
    }

    // In a real implementation, this would collect and return events
    // For now, it's a placeholder
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<WindowEvent> {
        None
    }
}
