//! Game State Management System
//!
//! Provides a framework for managing different game states (menu, gameplay, pause, etc.)
//! with clean transitions and state-specific logic.

use std::collections::HashMap;

/// Unique identifier for game states
pub type StateId = String;

/// Game state trait that all game states must implement
pub trait GameState {
    /// Called when entering this state
    fn on_enter(&mut self, _context: &mut StateContext) {}

    /// Called when exiting this state
    fn on_exit(&mut self, _context: &mut StateContext) {}

    /// Update logic for this state
    fn update(&mut self, context: &mut StateContext, delta_time: f32) -> StateTransition;

    /// Render logic for this state
    fn render(&mut self, _context: &mut StateContext) {}

    /// Handle input for this state
    fn handle_input(
        &mut self,
        _context: &mut StateContext,
        _input: &crate::input_window::WindowInputState,
    ) -> Option<StateTransition> {
        None
    }

    /// Get the state's unique identifier
    fn id(&self) -> StateId;
}

/// State transition commands
#[derive(Debug, Clone)]
pub enum StateTransition {
    /// Stay in current state
    None,
    /// Switch to a different state
    Switch(StateId),
    /// Push a new state on top of the current one
    Push(StateId),
    /// Pop the current state and return to the previous one
    Pop,
    /// Quit the game
    Quit,
}

/// Context passed to states during updates
pub struct StateContext {
    pub delta_time: f32,
    pub total_time: f32,
    pub window_width: usize,
    pub window_height: usize,
    // Add other shared resources here as needed
}

impl StateContext {
    pub fn new(window_width: usize, window_height: usize) -> Self {
        Self {
            delta_time: 0.0,
            total_time: 0.0,
            window_width,
            window_height,
        }
    }

    pub fn update_time(&mut self, delta_time: f32) {
        self.delta_time = delta_time;
        self.total_time += delta_time;
    }
}

/// Game state manager
pub struct StateManager {
    states: HashMap<StateId, Box<dyn GameState>>,
    state_stack: Vec<StateId>,
    context: StateContext,
}

impl StateManager {
    /// Create a new state manager
    pub fn new(window_width: usize, window_height: usize) -> Self {
        Self {
            states: HashMap::new(),
            state_stack: Vec::new(),
            context: StateContext::new(window_width, window_height),
        }
    }

    /// Register a state with the manager
    pub fn register_state(&mut self, state: Box<dyn GameState>) {
        let id = state.id();
        self.states.insert(id, state);
    }

    /// Switch to a specific state
    pub fn switch_to(&mut self, state_id: StateId) -> Result<(), String> {
        if !self.states.contains_key(&state_id) {
            return Err(format!("State '{}' not found", state_id));
        }

        // Exit current state
        if let Some(current_id) = self.state_stack.last() {
            if let Some(current_state) = self.states.get_mut(current_id) {
                current_state.on_exit(&mut self.context);
            }
        }

        // Switch to new state
        self.state_stack.clear();
        self.state_stack.push(state_id.clone());

        // Enter new state
        if let Some(new_state) = self.states.get_mut(&state_id) {
            new_state.on_enter(&mut self.context);
        }

        Ok(())
    }

    /// Push a state onto the stack
    pub fn push_state(&mut self, state_id: StateId) -> Result<(), String> {
        if !self.states.contains_key(&state_id) {
            return Err(format!("State '{}' not found", state_id));
        }

        // Pause current state (don't exit)
        if let Some(current_id) = self.state_stack.last() {
            if let Some(_current_state) = self.states.get_mut(current_id) {
                // Could add on_pause method to GameState trait
            }
        }

        // Push new state
        self.state_stack.push(state_id.clone());

        // Enter new state
        if let Some(new_state) = self.states.get_mut(&state_id) {
            new_state.on_enter(&mut self.context);
        }

        Ok(())
    }

    /// Pop the current state from the stack
    pub fn pop_state(&mut self) -> Result<(), String> {
        if self.state_stack.len() <= 1 {
            return Err("Cannot pop the last state".to_string());
        }

        // Exit current state
        if let Some(current_id) = self.state_stack.pop() {
            if let Some(current_state) = self.states.get_mut(&current_id) {
                current_state.on_exit(&mut self.context);
            }
        }

        // Resume previous state
        if let Some(previous_id) = self.state_stack.last() {
            if let Some(_previous_state) = self.states.get_mut(previous_id) {
                // Could add on_resume method to GameState trait
            }
        }

        Ok(())
    }

    /// Update the current state
    pub fn update(&mut self, delta_time: f32) -> StateTransition {
        self.context.update_time(delta_time);

        if let Some(current_id) = self.state_stack.last() {
            if let Some(current_state) = self.states.get_mut(current_id) {
                return current_state.update(&mut self.context, delta_time);
            }
        }

        StateTransition::None
    }

    /// Render the current state
    pub fn render(&mut self) {
        if let Some(current_id) = self.state_stack.last() {
            if let Some(current_state) = self.states.get_mut(current_id) {
                current_state.render(&mut self.context);
            }
        }
    }

    /// Handle input for the current state
    pub fn handle_input(
        &mut self,
        input: &crate::input_window::WindowInputState,
    ) -> Option<StateTransition> {
        if let Some(current_id) = self.state_stack.last() {
            if let Some(current_state) = self.states.get_mut(current_id) {
                return current_state.handle_input(&mut self.context, input);
            }
        }
        None
    }

    /// Get the current state ID
    pub fn current_state(&self) -> Option<&StateId> {
        self.state_stack.last()
    }

    /// Check if a specific state is active
    pub fn is_state_active(&self, state_id: &StateId) -> bool {
        self.state_stack.last() == Some(state_id)
    }

    /// Get the state stack depth
    pub fn stack_depth(&self) -> usize {
        self.state_stack.len()
    }

    /// Get context for external access
    pub fn context(&self) -> &StateContext {
        &self.context
    }

    /// Get mutable context for external access
    pub fn context_mut(&mut self) -> &mut StateContext {
        &mut self.context
    }
}

/// Example menu state implementation
pub struct MenuState {
    selected_option: usize,
    options: Vec<String>,
}

impl Default for MenuState {
    fn default() -> Self {
        Self::new()
    }
}

impl MenuState {
    pub fn new() -> Self {
        Self {
            selected_option: 0,
            options: vec![
                "Start Game".to_string(),
                "Options".to_string(),
                "Exit".to_string(),
            ],
        }
    }
}

impl GameState for MenuState {
    fn on_enter(&mut self, _context: &mut StateContext) {
        println!("Entering main menu");
    }

    fn on_exit(&mut self, _context: &mut StateContext) {
        println!("Exiting main menu");
    }

    fn update(&mut self, _context: &mut StateContext, _delta_time: f32) -> StateTransition {
        StateTransition::None
    }

    fn handle_input(
        &mut self,
        _context: &mut StateContext,
        input: &crate::input_window::WindowInputState,
    ) -> Option<StateTransition> {
        use minifb::Key;

        if (input.is_key_just_pressed(Key::W) || input.is_key_just_pressed(Key::Up))
            && self.selected_option > 0
        {
            self.selected_option -= 1;
        }

        if (input.is_key_just_pressed(Key::S) || input.is_key_just_pressed(Key::Down))
            && self.selected_option < self.options.len() - 1
        {
            self.selected_option += 1;
        }

        if input.is_key_just_pressed(Key::Space) || input.is_key_just_pressed(Key::Enter) {
            match self.selected_option {
                0 => return Some(StateTransition::Switch("gameplay".to_string())),
                1 => return Some(StateTransition::Push("options".to_string())),
                2 => return Some(StateTransition::Quit),
                _ => {}
            }
        }

        None
    }

    fn render(&mut self, _context: &mut StateContext) {
        // This would render the menu using the rendering system
        // For now, just print to console
        println!("=== MAIN MENU ===");
        for (i, option) in self.options.iter().enumerate() {
            let prefix = if i == self.selected_option { ">" } else { " " };
            println!("{} {}", prefix, option);
        }
        println!("=================");
    }

    fn id(&self) -> StateId {
        "menu".to_string()
    }
}

/// Example gameplay state implementation
pub struct GameplayState {
    score: u32,
    game_time: f32,
}

impl Default for GameplayState {
    fn default() -> Self {
        Self::new()
    }
}

impl GameplayState {
    pub fn new() -> Self {
        Self {
            score: 0,
            game_time: 0.0,
        }
    }
}

impl GameState for GameplayState {
    fn on_enter(&mut self, _context: &mut StateContext) {
        println!("Starting gameplay!");
        self.score = 0;
        self.game_time = 0.0;
    }

    fn update(&mut self, _context: &mut StateContext, delta_time: f32) -> StateTransition {
        self.game_time += delta_time;
        self.score = (self.game_time * 10.0) as u32; // Simple score based on time

        // Example: auto-win after 10 seconds
        if self.game_time > 10.0 {
            return StateTransition::Switch("game_over".to_string());
        }

        StateTransition::None
    }

    fn handle_input(
        &mut self,
        _context: &mut StateContext,
        input: &crate::input_window::WindowInputState,
    ) -> Option<StateTransition> {
        use minifb::Key;

        if input.is_key_just_pressed(Key::Escape) {
            return Some(StateTransition::Push("pause".to_string()));
        }

        None
    }

    fn render(&mut self, _context: &mut StateContext) {
        println!(
            "🎮 Gameplay - Score: {} | Time: {:.1}s",
            self.score, self.game_time
        );
    }

    fn id(&self) -> StateId {
        "gameplay".to_string()
    }
}

/// Example pause state implementation
pub struct PauseState;

impl Default for PauseState {
    fn default() -> Self {
        Self::new()
    }
}

impl PauseState {
    pub fn new() -> Self {
        Self
    }
}

impl GameState for PauseState {
    fn on_enter(&mut self, _context: &mut StateContext) {
        println!("Game paused");
    }

    fn update(&mut self, _context: &mut StateContext, _delta_time: f32) -> StateTransition {
        StateTransition::None
    }

    fn handle_input(
        &mut self,
        _context: &mut StateContext,
        input: &crate::input_window::WindowInputState,
    ) -> Option<StateTransition> {
        use minifb::Key;

        if input.is_key_just_pressed(Key::Escape) {
            return Some(StateTransition::Pop); // Resume game
        }

        None
    }

    fn render(&mut self, _context: &mut StateContext) {
        println!("⏸️  PAUSED - Press ESC to resume");
    }

    fn id(&self) -> StateId {
        "pause".to_string()
    }
}

/// Example game over state implementation
pub struct GameOverState {
    final_score: u32,
}

impl GameOverState {
    pub fn new(final_score: u32) -> Self {
        Self { final_score }
    }
}

impl GameState for GameOverState {
    fn on_enter(&mut self, _context: &mut StateContext) {
        println!("Game Over! Final Score: {}", self.final_score);
    }

    fn update(&mut self, _context: &mut StateContext, _delta_time: f32) -> StateTransition {
        StateTransition::None
    }

    fn handle_input(
        &mut self,
        _context: &mut StateContext,
        input: &crate::input_window::WindowInputState,
    ) -> Option<StateTransition> {
        use minifb::Key;

        if input.is_key_just_pressed(Key::Space) || input.is_key_just_pressed(Key::Enter) {
            return Some(StateTransition::Switch("menu".to_string()));
        }

        None
    }

    fn render(&mut self, _context: &mut StateContext) {
        println!("💀 GAME OVER - Final Score: {}", self.final_score);
        println!("Press SPACE to return to menu");
    }

    fn id(&self) -> StateId {
        "game_over".to_string()
    }
}
