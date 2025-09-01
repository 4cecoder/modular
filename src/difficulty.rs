//! Difficulty System
//!
//! A configurable difficulty system that provides multipliers and settings
//! for different game difficulty levels. Extracted from the Pong game.

use std::collections::HashMap;

/// Represents different difficulty levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DifficultyLevel {
    VeryEasy,
    Easy,
    Normal,
    Hard,
    VeryHard,
    Custom,
}

impl Default for DifficultyLevel {
    fn default() -> Self {
        DifficultyLevel::Normal
    }
}

/// Generic value type for difficulty settings
#[derive(Debug, Clone)]
pub enum DifficultyValue {
    Float(f32),
    Int(i32),
    Bool(bool),
    String(String),
}

/// Configuration for a specific difficulty level
#[derive(Debug, Clone)]
pub struct DifficultyConfig {
    pub name: String,
    pub description: String,
    pub values: HashMap<String, DifficultyValue>,
}

impl DifficultyConfig {
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            values: HashMap::new(),
        }
    }

    /// Set a float value for a specific game aspect
    pub fn set_float(&mut self, aspect: &str, value: f32) {
        self.values.insert(aspect.to_string(), DifficultyValue::Float(value));
    }

    /// Set an integer value for a specific game aspect
    pub fn set_int(&mut self, aspect: &str, value: i32) {
        self.values.insert(aspect.to_string(), DifficultyValue::Int(value));
    }

    /// Set a boolean value for a specific game aspect
    pub fn set_bool(&mut self, aspect: &str, value: bool) {
        self.values.insert(aspect.to_string(), DifficultyValue::Bool(value));
    }

    /// Set a string value for a specific game aspect
    pub fn set_string(&mut self, aspect: &str, value: &str) {
        self.values.insert(aspect.to_string(), DifficultyValue::String(value.to_string()));
    }

    /// Get a float value for a specific game aspect
    pub fn get_float(&self, aspect: &str) -> f32 {
        match self.values.get(aspect) {
            Some(DifficultyValue::Float(value)) => *value,
            _ => 1.0, // Default multiplier
        }
    }

    /// Get an integer value for a specific game aspect
    pub fn get_int(&self, aspect: &str) -> i32 {
        match self.values.get(aspect) {
            Some(DifficultyValue::Int(value)) => *value,
            _ => 0,
        }
    }

    /// Get a boolean value for a specific game aspect
    pub fn get_bool(&self, aspect: &str) -> bool {
        match self.values.get(aspect) {
            Some(DifficultyValue::Bool(value)) => *value,
            _ => false,
        }
    }

    /// Get a string value for a specific game aspect
    pub fn get_string(&self, aspect: &str) -> String {
        match self.values.get(aspect) {
            Some(DifficultyValue::String(value)) => value.clone(),
            _ => String::new(),
        }
    }

    /// Get a value of any type for a specific game aspect
    pub fn get_value(&self, aspect: &str) -> Option<&DifficultyValue> {
        self.values.get(aspect)
    }

    /// Check if an aspect is configured
    pub fn has_aspect(&self, aspect: &str) -> bool {
        self.values.contains_key(aspect)
    }

    /// Get all configured aspects
    pub fn get_aspects(&self) -> Vec<String> {
        self.values.keys().cloned().collect()
    }
}

/// Main difficulty system manager
pub struct DifficultySystem {
    current_level: DifficultyLevel,
    configs: HashMap<DifficultyLevel, DifficultyConfig>,
    custom_config: Option<DifficultyConfig>,
    /// Registered aspect names for validation
    registered_aspects: HashMap<String, String>, // aspect -> description
}

impl DifficultySystem {
    /// Create a new difficulty system
    pub fn new() -> Self {
        Self {
            current_level: DifficultyLevel::Normal,
            configs: HashMap::new(),
            custom_config: None,
            registered_aspects: HashMap::new(),
        }
    }

    /// Create a new difficulty system with default Pong configurations
    pub fn with_pong_defaults() -> Self {
        let mut system = Self::new();
        system.register_pong_aspects();
        system.initialize_pong_configs();
        system
    }

    /// Register a game aspect with description
    pub fn register_aspect(&mut self, aspect: &str, description: &str) {
        self.registered_aspects.insert(aspect.to_string(), description.to_string());
    }

    /// Register common Pong game aspects
    pub fn register_pong_aspects(&mut self) {
        self.register_aspect("ai_speed", "AI paddle movement speed multiplier");
        self.register_aspect("ball_speed", "Ball movement speed multiplier");
        self.register_aspect("paddle_speed", "Player paddle movement speed multiplier");
        self.register_aspect("ai_accuracy", "AI targeting accuracy (0.0-1.0)");
        self.register_aspect("max_score", "Maximum score to win the game");
        self.register_aspect("ball_trail_length", "Length of ball trail effect");
        self.register_aspect("particle_count", "Number of particles in effects");
    }

    /// Initialize default Pong difficulty configurations
    pub fn initialize_pong_configs(&mut self) {
        // Very Easy
        let mut very_easy = DifficultyConfig::new("Very Easy", "Perfect for beginners");
        very_easy.set_float("ai_speed", 0.4);
        very_easy.set_float("ball_speed", 0.7);
        very_easy.set_float("paddle_speed", 1.2);
        very_easy.set_float("ai_accuracy", 0.3);
        very_easy.set_int("max_score", 3);
        very_easy.set_int("ball_trail_length", 10);
        very_easy.set_int("particle_count", 15);
        self.configs.insert(DifficultyLevel::VeryEasy, very_easy);

        // Easy
        let mut easy = DifficultyConfig::new("Easy", "Relaxed gameplay");
        easy.set_float("ai_speed", 0.6);
        easy.set_float("ball_speed", 0.8);
        easy.set_float("paddle_speed", 1.1);
        easy.set_float("ai_accuracy", 0.5);
        easy.set_int("max_score", 5);
        easy.set_int("ball_trail_length", 15);
        easy.set_int("particle_count", 20);
        self.configs.insert(DifficultyLevel::Easy, easy);

        // Normal
        let mut normal = DifficultyConfig::new("Normal", "Balanced challenge");
        normal.set_float("ai_speed", 0.8);
        normal.set_float("ball_speed", 1.0);
        normal.set_float("paddle_speed", 1.0);
        normal.set_float("ai_accuracy", 0.7);
        normal.set_int("max_score", 5);
        normal.set_int("ball_trail_length", 20);
        normal.set_int("particle_count", 25);
        self.configs.insert(DifficultyLevel::Normal, normal);

        // Hard
        let mut hard = DifficultyConfig::new("Hard", "Challenging gameplay");
        hard.set_float("ai_speed", 1.0);
        hard.set_float("ball_speed", 1.2);
        hard.set_float("paddle_speed", 0.9);
        hard.set_float("ai_accuracy", 0.9);
        hard.set_int("max_score", 7);
        hard.set_int("ball_trail_length", 25);
        hard.set_int("particle_count", 30);
        self.configs.insert(DifficultyLevel::Hard, hard);

        // Very Hard
        let mut very_hard = DifficultyConfig::new("Very Hard", "Extreme challenge");
        very_hard.set_float("ai_speed", 1.2);
        very_hard.set_float("ball_speed", 1.4);
        very_hard.set_float("paddle_speed", 0.8);
        very_hard.set_float("ai_accuracy", 0.95);
        very_hard.set_int("max_score", 10);
        very_hard.set_int("ball_trail_length", 30);
        very_hard.set_int("particle_count", 40);
        self.configs.insert(DifficultyLevel::VeryHard, very_hard);
    }

    /// Set the current difficulty level
    pub fn set_difficulty(&mut self, level: DifficultyLevel) {
        self.current_level = level;
    }

    /// Get the current difficulty level
    pub fn get_current_level(&self) -> DifficultyLevel {
        self.current_level
    }

    /// Get the current difficulty configuration
    pub fn get_current_config(&self) -> &DifficultyConfig {
        match self.current_level {
            DifficultyLevel::Custom => self.custom_config.as_ref().unwrap_or_else(|| {
                panic!("Custom difficulty not configured")
            }),
            level => self.configs.get(&level).unwrap_or_else(|| {
                panic!("Difficulty level not found: {:?}", level)
            }),
        }
    }

    /// Get a float value for the current difficulty
    pub fn get_float(&self, aspect: &str) -> f32 {
        self.get_current_config().get_float(aspect)
    }

    /// Get an integer value for the current difficulty
    pub fn get_int(&self, aspect: &str) -> i32 {
        self.get_current_config().get_int(aspect)
    }

    /// Get a boolean value for the current difficulty
    pub fn get_bool(&self, aspect: &str) -> bool {
        self.get_current_config().get_bool(aspect)
    }

    /// Get a string value for the current difficulty
    pub fn get_string(&self, aspect: &str) -> String {
        self.get_current_config().get_string(aspect)
    }

    /// Get a value of any type for the current difficulty
    pub fn get_value(&self, aspect: &str) -> Option<&DifficultyValue> {
        self.get_current_config().get_value(aspect)
    }

    /// Helper methods for common game aspects (Pong-specific)
    pub fn ai_speed_multiplier(&self) -> f32 {
        self.get_float("ai_speed")
    }

    pub fn ball_speed_multiplier(&self) -> f32 {
        self.get_float("ball_speed")
    }

    pub fn paddle_speed_multiplier(&self) -> f32 {
        self.get_float("paddle_speed")
    }

    pub fn ai_accuracy(&self) -> f32 {
        self.get_float("ai_accuracy")
    }

    pub fn max_score(&self) -> i32 {
        self.get_int("max_score")
    }

    pub fn ball_trail_length(&self) -> i32 {
        self.get_int("ball_trail_length")
    }

    pub fn particle_count(&self) -> i32 {
        self.get_int("particle_count")
    }

    /// Get a specific float value for the current difficulty
    pub fn get_multiplier(&self, aspect: &str) -> f32 {
        self.get_current_config().get_float(aspect)
    }

    /// Get a specific setting for the current difficulty
    pub fn get_setting(&self, setting: &str) -> f32 {
        self.get_current_config().get_float(setting)
    }

    /// Set a custom difficulty configuration
    pub fn set_custom_config(&mut self, config: DifficultyConfig) {
        self.custom_config = Some(config);
        self.current_level = DifficultyLevel::Custom;
    }

    /// Get all available difficulty levels
    pub fn get_available_levels(&self) -> Vec<DifficultyLevel> {
        let mut levels: Vec<_> = self.configs.keys().cloned().collect();
        if self.custom_config.is_some() {
            levels.push(DifficultyLevel::Custom);
        }
        levels.sort_by_key(|level| match level {
            DifficultyLevel::VeryEasy => 0,
            DifficultyLevel::Easy => 1,
            DifficultyLevel::Normal => 2,
            DifficultyLevel::Hard => 3,
            DifficultyLevel::VeryHard => 4,
            DifficultyLevel::Custom => 5,
        });
        levels
    }

    /// Add a custom difficulty configuration
    pub fn add_config(&mut self, level: DifficultyLevel, config: DifficultyConfig) {
        self.configs.insert(level, config);
    }

    /// Remove a difficulty configuration
    pub fn remove_config(&mut self, level: DifficultyLevel) {
        self.configs.remove(&level);
        if self.current_level == level {
            self.current_level = DifficultyLevel::Normal;
        }
    }

    /// Get all registered aspects
    pub fn get_registered_aspects(&self) -> &HashMap<String, String> {
        &self.registered_aspects
    }

    /// Validate that a configuration has all registered aspects
    pub fn validate_config(&self, config: &DifficultyConfig) -> Vec<String> {
        let mut missing = Vec::new();
        for aspect in self.registered_aspects.keys() {
            if !config.has_aspect(aspect) {
                missing.push(aspect.clone());
            }
        }
        missing
    }

    /// Get the name of a difficulty level
    pub fn get_level_name(&self, level: DifficultyLevel) -> &str {
        match level {
            DifficultyLevel::Custom => {
                if let Some(ref config) = self.custom_config {
                    &config.name
                } else {
                    "Custom"
                }
            },
            level => {
                if let Some(config) = self.configs.get(&level) {
                    &config.name
                } else {
                    "Unknown"
                }
            }
        }
    }

    /// Get the description of a difficulty level
    pub fn get_level_description(&self, level: DifficultyLevel) -> &str {
        match level {
            DifficultyLevel::Custom => {
                if let Some(ref config) = self.custom_config {
                    &config.description
                } else {
                    "Custom difficulty configuration"
                }
            },
            level => {
                if let Some(config) = self.configs.get(&level) {
                    &config.description
                } else {
                    "Unknown difficulty level"
                }
            }
        }
    }

    /// Cycle to the next difficulty level
    pub fn next_level(&mut self) {
        let levels = self.get_available_levels();
        let current_index = levels.iter().position(|&level| level == self.current_level).unwrap_or(0);
        let next_index = (current_index + 1) % levels.len();
        self.current_level = levels[next_index];
    }

    /// Cycle to the previous difficulty level
    pub fn previous_level(&mut self) {
        let levels = self.get_available_levels();
        let current_index = levels.iter().position(|&level| level == self.current_level).unwrap_or(0);
        let prev_index = if current_index == 0 { levels.len() - 1 } else { current_index - 1 };
        self.current_level = levels[prev_index];
    }
}

impl Default for DifficultySystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_difficulty_system_creation() {
        let system = DifficultySystem::new();
        assert_eq!(system.get_current_level(), DifficultyLevel::Normal);
    }

    #[test]
    fn test_pong_difficulty_system() {
        let mut system = DifficultySystem::with_pong_defaults();
        assert_eq!(system.get_current_level(), DifficultyLevel::Normal);

        // Test helper methods
        assert_eq!(system.ai_speed_multiplier(), 0.8);
        assert_eq!(system.ball_speed_multiplier(), 1.0);
        assert_eq!(system.max_score(), 5);
    }

    #[test]
    fn test_difficulty_values() {
        let mut system = DifficultySystem::new();
        system.register_aspect("test_float", "Test float value");
        system.register_aspect("test_int", "Test int value");
        system.register_aspect("test_bool", "Test bool value");

        let mut config = DifficultyConfig::new("Test", "Test config");
        config.set_float("test_float", 2.5);
        config.set_int("test_int", 42);
        config.set_bool("test_bool", true);

        system.add_config(DifficultyLevel::Easy, config);
        system.set_difficulty(DifficultyLevel::Easy);

        assert_eq!(system.get_float("test_float"), 2.5);
        assert_eq!(system.get_int("test_int"), 42);
        assert_eq!(system.get_bool("test_bool"), true);
    }

    #[test]
    fn test_difficulty_navigation() {
        let mut system = DifficultySystem::with_pong_defaults();

        system.set_difficulty(DifficultyLevel::Easy);
        system.next_level();
        assert_eq!(system.get_current_level(), DifficultyLevel::Normal);

        system.previous_level();
        assert_eq!(system.get_current_level(), DifficultyLevel::Easy);
    }

    #[test]
    fn test_custom_difficulty() {
        let mut system = DifficultySystem::new();
        system.register_aspect("custom_value", "Custom test value");

        let mut custom_config = DifficultyConfig::new("Custom", "Custom difficulty");
        custom_config.set_float("custom_value", 99.9);

        system.set_custom_config(custom_config);
        assert_eq!(system.get_current_level(), DifficultyLevel::Custom);
        assert_eq!(system.get_float("custom_value"), 99.9);
    }

    #[test]
    fn test_config_validation() {
        let mut system = DifficultySystem::new();
        system.register_aspect("required_aspect", "Required aspect");

        let config = DifficultyConfig::new("Test", "Test config");
        // Don't set the required aspect

        let missing = system.validate_config(&config);
        assert_eq!(missing, vec!["required_aspect"]);
    }
}