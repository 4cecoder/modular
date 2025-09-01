//! Enhanced AI System
//!
//! A flexible AI system with behavior trees, state machines, and difficulty scaling.
//! Extracted and enhanced from the Pong game AI.

use crate::Vec2;
use std::collections::HashMap;

/// AI behavior types
#[derive(Debug, Clone, PartialEq)]
pub enum AIBehavior {
    /// Simple follow behavior (like Pong paddle)
    Follow,
    /// Patrol between waypoints
    Patrol,
    /// Chase and attack target
    Chase,
    /// Flee from target
    Flee,
    /// Guard a position
    Guard,
    /// Wander randomly
    Wander,
    /// Custom behavior with string identifier
    Custom(String),
}

/// AI difficulty levels (can be mapped to the difficulty system)
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum AIDifficulty {
    VeryEasy = 0,
    Easy = 1,
    Normal = 2,
    Hard = 3,
    VeryHard = 4,
}

impl AIDifficulty {
    /// Convert from difficulty multiplier (0.0-1.0)
    pub fn from_multiplier(multiplier: f32) -> Self {
        match multiplier {
            m if m <= 0.3 => AIDifficulty::VeryEasy,
            m if m <= 0.5 => AIDifficulty::Easy,
            m if m <= 0.8 => AIDifficulty::Normal,
            m if m <= 1.2 => AIDifficulty::Hard,
            _ => AIDifficulty::VeryHard,
        }
    }

    /// Get reaction time multiplier for this difficulty
    pub fn reaction_multiplier(&self) -> f32 {
        match self {
            AIDifficulty::VeryEasy => 2.0,
            AIDifficulty::Easy => 1.5,
            AIDifficulty::Normal => 1.0,
            AIDifficulty::Hard => 0.7,
            AIDifficulty::VeryHard => 0.5,
        }
    }

    /// Get accuracy multiplier for this difficulty
    pub fn accuracy_multiplier(&self) -> f32 {
        match self {
            AIDifficulty::VeryEasy => 0.3,
            AIDifficulty::Easy => 0.5,
            AIDifficulty::Normal => 0.7,
            AIDifficulty::Hard => 0.9,
            AIDifficulty::VeryHard => 0.95,
        }
    }

    /// Get prediction ability for this difficulty
    pub fn prediction_ability(&self) -> f32 {
        match self {
            AIDifficulty::VeryEasy => 0.0,
            AIDifficulty::Easy => 0.2,
            AIDifficulty::Normal => 0.5,
            AIDifficulty::Hard => 0.8,
            AIDifficulty::VeryHard => 1.0,
        }
    }
}

/// AI state for state machine
#[derive(Debug, Clone, PartialEq)]
pub enum AIState {
    Idle,
    Moving,
    Attacking,
    Defending,
    Patrolling,
    Chasing,
    Fleeing,
    Custom(String),
}

/// Target information for AI decision making
#[derive(Debug, Clone)]
pub struct AITarget {
    pub position: Vec2,
    pub velocity: Vec2,
    pub distance: f32,
    pub is_visible: bool,
    pub threat_level: f32,
}

impl AITarget {
    pub fn new(position: Vec2, velocity: Vec2, ai_position: Vec2) -> Self {
        let distance = (position - ai_position).magnitude();
        Self {
            position,
            velocity,
            distance,
            is_visible: true, // Assume visible by default
            threat_level: 1.0,
        }
    }

    /// Predict future position based on velocity
    pub fn predict_position(&self, time_ahead: f32) -> Vec2 {
        self.position + self.velocity * time_ahead
    }
}

/// AI decision context
#[derive(Debug, Clone)]
pub struct AIContext {
    pub position: Vec2,
    pub velocity: Vec2,
    pub target: Option<AITarget>,
    pub time: f32,
    pub delta_time: f32,
    pub difficulty: AIDifficulty,
    pub state: AIState,
    pub custom_data: HashMap<String, f32>,
}

impl AIContext {
    pub fn new(position: Vec2, velocity: Vec2, difficulty: AIDifficulty) -> Self {
        Self {
            position,
            velocity,
            target: None,
            time: 0.0,
            delta_time: 0.0,
            difficulty,
            state: AIState::Idle,
            custom_data: HashMap::new(),
        }
    }

    pub fn with_target(mut self, target: AITarget) -> Self {
        self.target = Some(target);
        self
    }

    pub fn set_custom_data(&mut self, key: &str, value: f32) {
        self.custom_data.insert(key.to_string(), value);
    }

    pub fn get_custom_data(&self, key: &str) -> f32 {
        self.custom_data.get(key).copied().unwrap_or(0.0)
    }
}

/// AI decision result
#[derive(Debug, Clone)]
pub struct AIDecision {
    pub desired_velocity: Vec2,
    pub desired_state: AIState,
    pub should_attack: bool,
    pub priority: f32,   // 0.0-1.0, higher = more important
    pub confidence: f32, // 0.0-1.0, how confident in this decision
}

impl AIDecision {
    pub fn new() -> Self {
        Self {
            desired_velocity: Vec2::new(0.0, 0.0),
            desired_state: AIState::Idle,
            should_attack: false,
            priority: 0.5,
            confidence: 1.0,
        }
    }

    pub fn with_velocity(mut self, velocity: Vec2) -> Self {
        self.desired_velocity = velocity;
        self
    }

    pub fn with_state(mut self, state: AIState) -> Self {
        self.desired_state = state;
        self
    }

    pub fn with_attack(mut self, attack: bool) -> Self {
        self.should_attack = attack;
        self
    }
}

/// AI behavior implementation trait
pub trait AIBehaviorImpl {
    fn decide(&self, context: &AIContext) -> AIDecision;
    fn get_behavior_type(&self) -> AIBehavior;
}

/// Pong paddle AI implementation
pub struct PongPaddleAI {
    pub paddle_speed: f32,
    pub reaction_delay: f32,
    pub last_decision_time: f32,
}

impl PongPaddleAI {
    pub fn new(paddle_speed: f32) -> Self {
        Self {
            paddle_speed,
            reaction_delay: 0.0,
            last_decision_time: 0.0,
        }
    }

    pub fn with_difficulty(mut self, difficulty: AIDifficulty) -> Self {
        self.reaction_delay = difficulty.reaction_multiplier() * 0.1;
        self
    }
}

impl AIBehaviorImpl for PongPaddleAI {
    fn decide(&self, context: &AIContext) -> AIDecision {
        let mut decision = AIDecision::new().with_state(AIState::Moving);

        if let Some(target) = &context.target {
            // Add reaction delay for more realistic AI
            if context.time - self.last_decision_time < self.reaction_delay {
                return decision.with_velocity(context.velocity); // Continue current movement
            }

            let paddle_center = context.position.y;
            let ball_center = target.position.y;
            let distance = (ball_center - paddle_center).abs();

            // Only move if ball is reasonably close
            if distance > 5.0 {
                let _direction = if ball_center > paddle_center {
                    1.0
                } else {
                    -1.0
                };
                let speed = self.paddle_speed * context.difficulty.reaction_multiplier();

                // Add some imperfection based on difficulty
                let accuracy = context.difficulty.accuracy_multiplier();
                let error = (context.time * 2.0).sin() * (1.0 - accuracy) * 20.0;

                let target_y = ball_center + error;
                let direction_with_error = if target_y > paddle_center { 1.0 } else { -1.0 };

                decision.desired_velocity = Vec2::new(0.0, direction_with_error * speed);
            } else {
                decision.desired_velocity = Vec2::new(0.0, 0.0);
            }
        }

        decision
    }

    fn get_behavior_type(&self) -> AIBehavior {
        AIBehavior::Follow
    }
}

/// Simple chase AI implementation
pub struct ChaseAI {
    pub move_speed: f32,
    pub attack_range: f32,
    pub prediction_time: f32,
}

impl ChaseAI {
    pub fn new(move_speed: f32, attack_range: f32) -> Self {
        Self {
            move_speed,
            attack_range,
            prediction_time: 0.5,
        }
    }

    pub fn with_difficulty(mut self, difficulty: AIDifficulty) -> Self {
        self.prediction_time = difficulty.prediction_ability() * 0.5;
        self
    }
}

impl AIBehaviorImpl for ChaseAI {
    fn decide(&self, context: &AIContext) -> AIDecision {
        let mut decision = AIDecision::new().with_state(AIState::Chasing);

        if let Some(target) = &context.target {
            let to_target = target.position - context.position;
            let distance = to_target.magnitude();

            if distance <= self.attack_range {
                // In attack range
                decision = decision
                    .with_state(AIState::Attacking)
                    .with_attack(true)
                    .with_velocity(Vec2::new(0.0, 0.0));
            } else {
                // Move towards target with prediction
                let predicted_position = target.predict_position(self.prediction_time);
                let to_predicted = predicted_position - context.position;
                let direction = to_predicted.normalize();

                decision.desired_velocity = direction * self.move_speed;
            }
        } else {
            decision.desired_state = AIState::Idle;
        }

        decision
    }

    fn get_behavior_type(&self) -> AIBehavior {
        AIBehavior::Chase
    }
}

/// Patrol AI implementation
pub struct PatrolAI {
    pub waypoints: Vec<Vec2>,
    pub current_waypoint: usize,
    pub move_speed: f32,
    pub waypoint_threshold: f32,
}

impl PatrolAI {
    pub fn new(waypoints: Vec<Vec2>, move_speed: f32) -> Self {
        Self {
            waypoints,
            current_waypoint: 0,
            move_speed,
            waypoint_threshold: 10.0,
        }
    }

    pub fn with_difficulty(mut self, difficulty: AIDifficulty) -> Self {
        // Harder difficulties might have more complex patrol patterns
        self.waypoint_threshold *= difficulty.reaction_multiplier();
        self
    }
}

impl AIBehaviorImpl for PatrolAI {
    fn decide(&self, context: &AIContext) -> AIDecision {
        let mut decision = AIDecision::new().with_state(AIState::Patrolling);

        if self.waypoints.is_empty() {
            return decision.with_state(AIState::Idle);
        }

        let target_waypoint = self.waypoints[self.current_waypoint];
        let to_waypoint = target_waypoint - context.position;
        let distance = to_waypoint.magnitude();

        if distance <= self.waypoint_threshold {
            // Reached waypoint, move to next
            let next_waypoint = (self.current_waypoint + 1) % self.waypoints.len();
            let next_target = self.waypoints[next_waypoint];
            let to_next = next_target - context.position;
            let direction = to_next.normalize();

            decision.desired_velocity = direction * self.move_speed;
            // Note: In a real implementation, you'd need to update current_waypoint
            // This would require mutable access to the AI instance
        } else {
            let direction = to_waypoint.normalize();
            decision.desired_velocity = direction * self.move_speed;
        }

        decision
    }

    fn get_behavior_type(&self) -> AIBehavior {
        AIBehavior::Patrol
    }
}

/// Main AI system manager
pub struct AISystem {
    agents: HashMap<String, Box<dyn AIBehaviorImpl>>,
    contexts: HashMap<String, AIContext>,
}

impl AISystem {
    pub fn new() -> Self {
        Self {
            agents: HashMap::new(),
            contexts: HashMap::new(),
        }
    }

    /// Register an AI agent
    pub fn register_agent(&mut self, id: &str, agent: Box<dyn AIBehaviorImpl>) {
        self.agents.insert(id.to_string(), agent);
    }

    /// Register a Pong paddle AI
    pub fn register_pong_ai(&mut self, id: &str, paddle_speed: f32, difficulty: AIDifficulty) {
        let ai = PongPaddleAI::new(paddle_speed).with_difficulty(difficulty);
        self.register_agent(id, Box::new(ai));
    }

    /// Register a chase AI
    pub fn register_chase_ai(
        &mut self,
        id: &str,
        move_speed: f32,
        attack_range: f32,
        difficulty: AIDifficulty,
    ) {
        let ai = ChaseAI::new(move_speed, attack_range).with_difficulty(difficulty);
        self.register_agent(id, Box::new(ai));
    }

    /// Register a patrol AI
    pub fn register_patrol_ai(
        &mut self,
        id: &str,
        waypoints: Vec<Vec2>,
        move_speed: f32,
        difficulty: AIDifficulty,
    ) {
        let ai = PatrolAI::new(waypoints, move_speed).with_difficulty(difficulty);
        self.register_agent(id, Box::new(ai));
    }

    /// Update AI context for an agent
    pub fn update_context(
        &mut self,
        id: &str,
        position: Vec2,
        velocity: Vec2,
        target: Option<AITarget>,
    ) {
        let context = self
            .contexts
            .entry(id.to_string())
            .or_insert_with(|| AIContext::new(position, velocity, AIDifficulty::Normal));

        context.position = position;
        context.velocity = velocity;
        context.target = target;
        context.delta_time = 0.016; // Assume 60 FPS
        context.time += context.delta_time;
    }

    /// Set difficulty for an agent
    pub fn set_agent_difficulty(&mut self, id: &str, difficulty: AIDifficulty) {
        if let Some(context) = self.contexts.get_mut(id) {
            context.difficulty = difficulty;
        }
    }

    /// Get AI decision for an agent
    pub fn get_decision(&self, id: &str) -> Option<AIDecision> {
        let agent = self.agents.get(id)?;
        let context = self.contexts.get(id)?;

        Some(agent.decide(context))
    }

    /// Update all AI agents
    pub fn update_all(&mut self, delta_time: f32) {
        for context in self.contexts.values_mut() {
            context.delta_time = delta_time;
            context.time += delta_time;
        }
    }

    /// Remove an AI agent
    pub fn remove_agent(&mut self, id: &str) {
        self.agents.remove(id);
        self.contexts.remove(id);
    }

    /// Get all registered agent IDs
    pub fn get_agent_ids(&self) -> Vec<String> {
        self.agents.keys().cloned().collect()
    }

    /// Check if an agent exists
    pub fn has_agent(&self, id: &str) -> bool {
        self.agents.contains_key(id)
    }
}

impl Default for AISystem {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper functions for common AI setups
pub mod ai_helpers {
    use super::*;

    /// Create a standard Pong AI setup
    pub fn create_pong_ai(difficulty: AIDifficulty) -> PongPaddleAI {
        PongPaddleAI::new(350.0).with_difficulty(difficulty)
    }

    /// Create a simple enemy AI that chases the player
    pub fn create_chaser_ai(difficulty: AIDifficulty) -> ChaseAI {
        ChaseAI::new(100.0, 50.0).with_difficulty(difficulty)
    }

    /// Create a patrolling guard AI
    pub fn create_guard_ai(waypoints: Vec<Vec2>, difficulty: AIDifficulty) -> PatrolAI {
        PatrolAI::new(waypoints, 75.0).with_difficulty(difficulty)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_difficulty() {
        let easy = AIDifficulty::Easy;
        let hard = AIDifficulty::Hard;

        assert!(easy.reaction_multiplier() > hard.reaction_multiplier());
        assert!(easy.accuracy_multiplier() < hard.accuracy_multiplier());
    }

    #[test]
    fn test_pong_ai_decision() {
        let ai = PongPaddleAI::new(100.0);
        let mut context = AIContext::new(
            Vec2::new(400.0, 300.0), // AI position
            Vec2::new(0.0, 0.0),     // AI velocity
            AIDifficulty::Normal,
        );

        // Ball above AI
        let target = AITarget::new(
            Vec2::new(350.0, 250.0), // Ball position
            Vec2::new(0.0, 0.0),     // Ball velocity
            context.position,
        );
        context.target = Some(target);

        let decision = ai.decide(&context);
        assert!(decision.desired_velocity.y < 0.0); // Should move up
    }

    #[test]
    fn test_chase_ai_decision() {
        let ai = ChaseAI::new(100.0, 50.0);
        let mut context = AIContext::new(
            Vec2::new(0.0, 0.0),
            Vec2::new(0.0, 0.0),
            AIDifficulty::Normal,
        );

        let target = AITarget::new(Vec2::new(100.0, 0.0), Vec2::new(0.0, 0.0), context.position);
        context.target = Some(target);

        let decision = ai.decide(&context);
        assert!(decision.desired_velocity.x > 0.0); // Should move right towards target
    }

    #[test]
    fn test_ai_system() {
        let mut system = AISystem::new();

        system.register_pong_ai("paddle1", 100.0, AIDifficulty::Normal);
        assert!(system.has_agent("paddle1"));

        system.update_context(
            "paddle1",
            Vec2::new(400.0, 300.0),
            Vec2::new(0.0, 0.0),
            None,
        );

        let decision = system.get_decision("paddle1");
        assert!(decision.is_some());
    }
}
