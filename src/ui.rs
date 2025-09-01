//! UI system module
//!
//! User interface with buttons, panels, and text.

use crate::Vec2;
use specs::{Component, VecStorage};

/// UI element component
#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct UIElement {
    pub element_type: UIElementType,
    pub position: Vec2,
    pub size: Vec2,
}

/// UI element types
#[derive(Debug, Clone)]
pub enum UIElementType {
    Button { text: String },
    Text { text: String },
    Panel,
}

/// UI manager placeholder
pub struct UIManager {
    pub elements: Vec<UIElement>,
}

impl UIManager {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    pub fn add_element(&mut self, element: UIElement) {
        self.elements.push(element);
    }
}
