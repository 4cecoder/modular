//! Event system module
//!
//! Decoupled communication between systems.

use std::collections::HashMap;

/// Event types
#[derive(Debug, Clone)]
pub enum GameEvent {
    EntityCreated,
    EntityDestroyed,
    Collision,
}

/// Event bus placeholder
pub struct EventBus {
    pub subscribers: HashMap<String, Vec<Box<dyn Fn(&GameEvent)>>>,
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            subscribers: HashMap::new(),
        }
    }

    pub fn publish(&self, _event: GameEvent) {
        // Publish event
    }
}
