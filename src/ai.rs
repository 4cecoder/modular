//! AI system module
//!
//! Artificial intelligence with pathfinding and behavior trees.

use crate::Vec2;
use specs::{Component, VecStorage};

/// AI state component
#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct AIState {
    pub current_state: String,
    pub target_position: Option<Vec2>,
}

/// Pathfinding component
#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Path {
    pub waypoints: Vec<Vec2>,
    pub current_waypoint: usize,
}

/// AI controller placeholder
pub struct AIController {
    pub decision_timer: f32,
}

impl AIController {
    pub fn new() -> Self {
        Self {
            decision_timer: 0.0,
        }
    }

    pub fn update(&mut self, _delta_time: f32) {
        // AI decision making
    }
}
