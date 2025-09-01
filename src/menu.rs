//! Enhanced Menu System
//!
//! A comprehensive menu system with navigation, selection highlighting,
//! and various menu types. Builds on the existing game state system.

use crate::Vec2;
use std::collections::HashMap;

/// Menu item types
#[derive(Debug, Clone)]
pub enum MenuItemType {
    /// Simple action button
    Button { text: String, action: MenuAction },
    /// Toggle option
    Toggle {
        text: String,
        value: bool,
        action: MenuAction,
    },
    /// Slider with min/max values
    Slider {
        text: String,
        value: f32,
        min: f32,
        max: f32,
        step: f32,
        action: MenuAction,
    },
    /// Selection from multiple options
    Selector {
        text: String,
        options: Vec<String>,
        selected: usize,
        action: MenuAction,
    },
    /// Display-only text
    Label { text: String },
    /// Spacer for layout
    Spacer { height: f32 },
}

/// Actions that can be triggered by menu items
#[derive(Debug, Clone)]
pub enum MenuAction {
    /// No action
    None,
    /// Switch to a different game state
    ChangeState(String),
    /// Toggle a boolean setting
    ToggleSetting(String),
    /// Set a numeric setting
    SetSetting(String, f32),
    /// Select an option
    SelectOption(String, usize),
    /// Custom action with string parameter
    Custom(String),
    /// Go back to previous menu
    Back,
    /// Quit the game
    Quit,
}

/// Individual menu item
#[derive(Debug, Clone)]
pub struct MenuItem {
    pub item_type: MenuItemType,
    pub position: Vec2,
    pub size: Vec2,
    pub enabled: bool,
    pub visible: bool,
    pub id: String,
}

impl MenuItem {
    pub fn new(id: &str, item_type: MenuItemType) -> Self {
        Self {
            item_type,
            position: Vec2::new(0.0, 0.0),
            size: Vec2::new(200.0, 40.0),
            enabled: true,
            visible: true,
            id: id.to_string(),
        }
    }

    pub fn with_position(mut self, x: f32, y: f32) -> Self {
        self.position = Vec2::new(x, y);
        self
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.size = Vec2::new(width, height);
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn get_text(&self) -> &str {
        match &self.item_type {
            MenuItemType::Button { text, .. } => text,
            MenuItemType::Toggle { text, .. } => text,
            MenuItemType::Slider { text, .. } => text,
            MenuItemType::Selector { text, .. } => text,
            MenuItemType::Label { text } => text,
            MenuItemType::Spacer { .. } => "",
        }
    }

    pub fn is_selectable(&self) -> bool {
        match &self.item_type {
            MenuItemType::Button { .. }
            | MenuItemType::Toggle { .. }
            | MenuItemType::Slider { .. }
            | MenuItemType::Selector { .. } => self.enabled,
            MenuItemType::Label { .. } | MenuItemType::Spacer { .. } => false,
        }
    }
}

/// Menu configuration
#[derive(Debug, Clone)]
pub struct MenuConfig {
    pub title: String,
    pub title_position: Vec2,
    pub background_color: [f32; 4],
    pub item_spacing: f32,
    pub item_height: f32,
    pub item_width: f32,
    pub selected_color: [f32; 4],
    pub normal_color: [f32; 4],
    pub disabled_color: [f32; 4],
    pub allow_wrapping: bool,
    pub center_items: bool,
}

impl Default for MenuConfig {
    fn default() -> Self {
        Self {
            title: String::new(),
            title_position: Vec2::new(400.0, 100.0),
            background_color: [0.1, 0.1, 0.2, 0.9],
            item_spacing: 10.0,
            item_height: 40.0,
            item_width: 250.0,
            selected_color: [1.0, 1.0, 0.0, 1.0], // Yellow
            normal_color: [1.0, 1.0, 1.0, 1.0],   // White
            disabled_color: [0.5, 0.5, 0.5, 1.0], // Gray
            allow_wrapping: true,
            center_items: true,
        }
    }
}

/// Main menu system
pub struct MenuSystem {
    pub config: MenuConfig,
    pub items: Vec<MenuItem>,
    pub selected_index: usize,
    pub navigation_enabled: bool,
    pub settings: HashMap<String, MenuSetting>,
}

#[derive(Debug, Clone)]
pub enum MenuSetting {
    Bool(bool),
    Float(f32),
    Int(usize),
    String(String),
}

impl MenuSystem {
    /// Create a new menu system
    pub fn new(config: MenuConfig) -> Self {
        Self {
            config,
            items: Vec::new(),
            selected_index: 0,
            navigation_enabled: true,
            settings: HashMap::new(),
        }
    }

    /// Add a menu item
    pub fn add_item(&mut self, item: MenuItem) {
        self.items.push(item);
        self.update_layout();
    }

    /// Remove a menu item by ID
    pub fn remove_item(&mut self, id: &str) {
        self.items.retain(|item| item.id != id);
        self.update_layout();
        self.clamp_selection();
    }

    /// Clear all items
    pub fn clear_items(&mut self) {
        self.items.clear();
        self.selected_index = 0;
    }

    /// Get the currently selected item
    pub fn get_selected_item(&self) -> Option<&MenuItem> {
        if self.selected_index < self.items.len() {
            Some(&self.items[self.selected_index])
        } else {
            None
        }
    }

    /// Get the currently selected item mutably
    pub fn get_selected_item_mut(&mut self) -> Option<&mut MenuItem> {
        if self.selected_index < self.items.len() {
            Some(&mut self.items[self.selected_index])
        } else {
            None
        }
    }

    /// Select the next item
    pub fn select_next(&mut self) {
        if !self.navigation_enabled || self.items.is_empty() {
            return;
        }

        let mut next_index = self.selected_index;
        loop {
            next_index = (next_index + 1) % self.items.len();
            if next_index == self.selected_index {
                break; // Wrapped around, no selectable items
            }
            if self.items[next_index].is_selectable() {
                self.selected_index = next_index;
                break;
            }
        }
    }

    /// Select the previous item
    pub fn select_previous(&mut self) {
        if !self.navigation_enabled || self.items.is_empty() {
            return;
        }

        let mut prev_index = self.selected_index;
        loop {
            prev_index = if prev_index == 0 {
                self.items.len() - 1
            } else {
                prev_index - 1
            };
            if prev_index == self.selected_index {
                break; // Wrapped around, no selectable items
            }
            if self.items[prev_index].is_selectable() {
                self.selected_index = prev_index;
                break;
            }
        }
    }

    /// Select item by index
    pub fn select_index(&mut self, index: usize) {
        if index < self.items.len() && self.items[index].is_selectable() {
            self.selected_index = index;
        }
    }

    /// Select item by ID
    pub fn select_by_id(&mut self, id: &str) {
        for (index, item) in self.items.iter().enumerate() {
            if item.id == id && item.is_selectable() {
                self.selected_index = index;
                break;
            }
        }
    }

    /// Activate the currently selected item
    pub fn activate_selected(&mut self) -> Option<MenuAction> {
        if let Some(item) = self.get_selected_item() {
            if item.enabled {
                return self.get_item_action(item);
            }
        }
        None
    }

    /// Get the action for a menu item
    fn get_item_action(&self, item: &MenuItem) -> Option<MenuAction> {
        match &item.item_type {
            MenuItemType::Button { action, .. } => Some(action.clone()),
            MenuItemType::Toggle { action, .. } => {
                // For toggles, we might want to return a modified action
                Some(action.clone())
            }
            MenuItemType::Slider { action, .. } => Some(action.clone()),
            MenuItemType::Selector { action, .. } => Some(action.clone()),
            _ => None,
        }
    }

    /// Update the layout of menu items
    pub fn update_layout(&mut self) {
        let start_y = self.config.title_position.y + 80.0;
        let mut current_y = start_y;

        for item in &mut self.items {
            if !item.visible {
                continue;
            }

            if self.config.center_items {
                item.position.x = self.config.title_position.x - self.config.item_width / 2.0;
            } else {
                item.position.x = self.config.title_position.x;
            }

            item.position.y = current_y;
            item.size.x = self.config.item_width;
            item.size.y = self.config.item_height;

            current_y += self.config.item_height + self.config.item_spacing;
        }

        self.clamp_selection();
    }

    /// Ensure selected index is valid
    fn clamp_selection(&mut self) {
        if self.items.is_empty() {
            self.selected_index = 0;
            return;
        }

        // Find the first selectable item if current selection is invalid
        if self.selected_index >= self.items.len()
            || !self.items[self.selected_index].is_selectable()
        {
            for (index, item) in self.items.iter().enumerate() {
                if item.is_selectable() {
                    self.selected_index = index;
                    return;
                }
            }
            // No selectable items, keep current index
        }
    }

    /// Handle input for menu navigation
    pub fn handle_input(&mut self, input_state: &crate::input_window::WindowInputState) {
        use minifb::Key;

        if !self.navigation_enabled {
            return;
        }

        // Navigation
        if input_state.is_key_just_pressed(Key::Down) {
            self.select_next();
        } else if input_state.is_key_just_pressed(Key::Up) {
            self.select_previous();
        }

        // Activation
        if input_state.is_key_just_pressed(Key::Enter)
            || input_state.is_key_just_pressed(Key::Space)
        {
            self.activate_selected();
        }

        // Direct selection with number keys
        for i in 0..9 {
            let key = match i {
                0 => Key::Key1,
                1 => Key::Key2,
                2 => Key::Key3,
                3 => Key::Key4,
                4 => Key::Key5,
                5 => Key::Key6,
                6 => Key::Key7,
                7 => Key::Key8,
                8 => Key::Key9,
                _ => continue,
            };

            if input_state.is_key_just_pressed(key) {
                self.select_index(i);
                break;
            }
        }
    }

    /// Get a setting value
    pub fn get_setting(&self, key: &str) -> Option<&MenuSetting> {
        self.settings.get(key)
    }

    /// Set a setting value
    pub fn set_setting(&mut self, key: String, value: MenuSetting) {
        self.settings.insert(key, value);
    }

    /// Get all selectable items
    pub fn get_selectable_items(&self) -> Vec<&MenuItem> {
        self.items
            .iter()
            .filter(|item| item.is_selectable())
            .collect()
    }

    /// Create a preset main menu
    pub fn create_main_menu() -> Self {
        let config = MenuConfig {
            title: "MAIN MENU".to_string(),
            ..Default::default()
        };

        let mut menu = Self::new(config);

        menu.add_item(MenuItem::new(
            "play",
            MenuItemType::Button {
                text: "Play Game".to_string(),
                action: MenuAction::ChangeState("gameplay".to_string()),
            },
        ));

        menu.add_item(MenuItem::new(
            "settings",
            MenuItemType::Button {
                text: "Settings".to_string(),
                action: MenuAction::ChangeState("settings".to_string()),
            },
        ));

        menu.add_item(MenuItem::new(
            "quit",
            MenuItemType::Button {
                text: "Quit".to_string(),
                action: MenuAction::Quit,
            },
        ));

        menu
    }

    /// Create a preset difficulty selection menu
    pub fn create_difficulty_menu() -> Self {
        let config = MenuConfig {
            title: "SELECT DIFFICULTY".to_string(),
            ..Default::default()
        };

        let mut menu = Self::new(config);

        menu.add_item(MenuItem::new(
            "easy",
            MenuItemType::Button {
                text: "Easy".to_string(),
                action: MenuAction::Custom("difficulty_easy".to_string()),
            },
        ));

        menu.add_item(MenuItem::new(
            "normal",
            MenuItemType::Button {
                text: "Normal".to_string(),
                action: MenuAction::Custom("difficulty_normal".to_string()),
            },
        ));

        menu.add_item(MenuItem::new(
            "hard",
            MenuItemType::Button {
                text: "Hard".to_string(),
                action: MenuAction::Custom("difficulty_hard".to_string()),
            },
        ));

        menu.add_item(MenuItem::new(
            "back",
            MenuItemType::Button {
                text: "Back".to_string(),
                action: MenuAction::Back,
            },
        ));

        menu
    }

    /// Create a preset settings menu
    pub fn create_settings_menu() -> Self {
        let config = MenuConfig {
            title: "SETTINGS".to_string(),
            ..Default::default()
        };

        let mut menu = Self::new(config);

        menu.add_item(MenuItem::new(
            "music",
            MenuItemType::Toggle {
                text: "Music".to_string(),
                value: true,
                action: MenuAction::ToggleSetting("music_enabled".to_string()),
            },
        ));

        menu.add_item(MenuItem::new(
            "sound",
            MenuItemType::Toggle {
                text: "Sound Effects".to_string(),
                value: true,
                action: MenuAction::ToggleSetting("sound_enabled".to_string()),
            },
        ));

        menu.add_item(MenuItem::new(
            "volume",
            MenuItemType::Slider {
                text: "Master Volume".to_string(),
                value: 0.8,
                min: 0.0,
                max: 1.0,
                step: 0.1,
                action: MenuAction::SetSetting("master_volume".to_string(), 0.8),
            },
        ));

        menu.add_item(MenuItem::new(
            "back",
            MenuItemType::Button {
                text: "Back".to_string(),
                action: MenuAction::Back,
            },
        ));

        menu
    }
}

impl Default for MenuSystem {
    fn default() -> Self {
        Self::new(MenuConfig::default())
    }
}

/// Helper functions for creating common menu items
pub mod menu_items {
    use super::*;

    pub fn button(id: &str, text: &str, action: MenuAction) -> MenuItem {
        MenuItem::new(
            id,
            MenuItemType::Button {
                text: text.to_string(),
                action,
            },
        )
    }

    pub fn toggle(id: &str, text: &str, value: bool, action: MenuAction) -> MenuItem {
        MenuItem::new(
            id,
            MenuItemType::Toggle {
                text: text.to_string(),
                value,
                action,
            },
        )
    }

    pub fn slider(
        id: &str,
        text: &str,
        value: f32,
        min: f32,
        max: f32,
        step: f32,
        action: MenuAction,
    ) -> MenuItem {
        MenuItem::new(
            id,
            MenuItemType::Slider {
                text: text.to_string(),
                value,
                min,
                max,
                step,
                action,
            },
        )
    }

    pub fn selector(
        id: &str,
        text: &str,
        options: Vec<String>,
        selected: usize,
        action: MenuAction,
    ) -> MenuItem {
        MenuItem::new(
            id,
            MenuItemType::Selector {
                text: text.to_string(),
                options,
                selected,
                action,
            },
        )
    }

    pub fn label(id: &str, text: &str) -> MenuItem {
        MenuItem::new(
            id,
            MenuItemType::Label {
                text: text.to_string(),
            },
        )
    }

    pub fn spacer(id: &str, height: f32) -> MenuItem {
        MenuItem::new(id, MenuItemType::Spacer { height })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_menu_creation() {
        let menu = MenuSystem::create_main_menu();
        assert_eq!(menu.items.len(), 3);
        assert_eq!(menu.config.title, "MAIN MENU");
    }

    #[test]
    fn test_menu_navigation() {
        let mut menu = MenuSystem::create_main_menu();

        let initial_selection = menu.selected_index;
        menu.select_next();
        assert_ne!(menu.selected_index, initial_selection);

        menu.select_previous();
        assert_eq!(menu.selected_index, initial_selection);
    }

    #[test]
    fn test_menu_item_creation() {
        let button = menu_items::button("test", "Test Button", MenuAction::None);
        assert_eq!(button.get_text(), "Test Button");
        assert!(button.is_selectable());
    }

    #[test]
    fn test_menu_item_types() {
        let label = menu_items::label("test", "Test Label");
        assert_eq!(label.get_text(), "Test Label");
        assert!(!label.is_selectable());

        let spacer = menu_items::spacer("test", 20.0);
        assert_eq!(spacer.get_text(), "");
        assert!(!spacer.is_selectable());
    }
}
