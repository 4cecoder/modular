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

/// Type alias for event subscriber functions
type EventSubscriber = Box<dyn Fn(&GameEvent)>;

/// Event bus placeholder
pub struct EventBus {
    pub subscribers: HashMap<String, Vec<EventSubscriber>>,
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
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
