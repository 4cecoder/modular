//! Rendering system module
//!
//! Graphics rendering with sprites, cameras, and visual effects.

use crate::Vec2;
use specs::{Component, DenseVecStorage, VecStorage};

/// Sprite component for 2D rendering
#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Sprite {
    pub texture_id: String,
    pub size: Vec2,
    pub color: [f32; 4],
}

/// Camera component for view management
#[derive(Component, Debug, Clone)]
#[storage(DenseVecStorage)]
pub struct Camera2D {
    pub position: Vec2,
    pub zoom: f32,
    pub rotation: f32,
    pub viewport_size: Vec2,
}

/// Renderer placeholder
pub struct Renderer {
    pub clear_color: [f32; 4],
}

impl Default for Renderer {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            clear_color: [0.0, 0.0, 0.0, 1.0],
        }
    }

    pub fn render(&mut self, _delta_time: f32) {
        // Rendering logic
    }
}
