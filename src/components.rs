//! Game components
//!
//! This module defines all the core components used in the game.

use crate::Vec2;
use specs::{Component, DenseVecStorage, VecStorage};

/// Position component for 2D positioning
#[derive(Component, Debug, Clone, Copy)]
#[storage(VecStorage)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn as_vec2(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
}

/// Velocity component for movement
#[derive(Component, Debug, Clone, Copy)]
#[storage(VecStorage)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn as_vec2(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
}

/// Acceleration component for physics
#[derive(Component, Debug, Clone, Copy)]
#[storage(VecStorage)]
pub struct Acceleration {
    pub x: f32,
    pub y: f32,
}

impl Acceleration {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

/// Renderable component for visual entities
#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Renderable {
    pub sprite_id: String,
    pub layer: i32,
    pub visible: bool,
    pub scale: f32,
}

impl Renderable {
    pub fn new(sprite_id: String) -> Self {
        Self {
            sprite_id,
            layer: 0,
            visible: true,
            scale: 1.0,
        }
    }
}

/// Player component to mark player entities
#[derive(Component, Debug, Clone, Default)]
#[storage(DenseVecStorage)]
pub struct Player {
    pub id: u32,
    pub health: f32,
    pub max_health: f32,
}

/// Enemy component for AI-controlled entities
#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Enemy {
    pub enemy_type: EnemyType,
    pub health: f32,
    pub max_health: f32,
    pub damage: f32,
}

impl Enemy {
    pub fn new(enemy_type: EnemyType) -> Self {
        let (health, damage) = match enemy_type {
            EnemyType::Basic => (50.0, 10.0),
            EnemyType::Advanced => (100.0, 20.0),
            EnemyType::Boss => (500.0, 50.0),
        };

        Self {
            enemy_type,
            health,
            max_health: health,
            damage,
        }
    }
}

/// Types of enemies
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnemyType {
    Basic,
    Advanced,
    Boss,
}

/// Health component for damageable entities
#[derive(Component, Debug, Clone, Copy)]
#[storage(VecStorage)]
pub struct Health {
    pub current: f32,
    pub maximum: f32,
}

impl Health {
    pub fn new(maximum: f32) -> Self {
        Self {
            current: maximum,
            maximum,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.current > 0.0
    }

    pub fn take_damage(&mut self, damage: f32) {
        self.current = (self.current - damage).max(0.0);
    }

    pub fn heal(&mut self, amount: f32) {
        self.current = (self.current + amount).min(self.maximum);
    }
}

/// Collider component for collision detection
#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Collider {
    pub shape: CollisionShape,
    pub is_trigger: bool,
}

impl Collider {
    pub fn new_circle(radius: f32) -> Self {
        Self {
            shape: CollisionShape::Circle { radius },
            is_trigger: false,
        }
    }

    pub fn new_rectangle(width: f32, height: f32) -> Self {
        Self {
            shape: CollisionShape::Rectangle { width, height },
            is_trigger: false,
        }
    }
}

/// Collision shapes
#[derive(Debug, Clone)]
pub enum CollisionShape {
    Circle { radius: f32 },
    Rectangle { width: f32, height: f32 },
}

/// Camera component for rendering
#[derive(Component, Debug, Clone)]
#[storage(DenseVecStorage)]
pub struct Camera {
    pub position: Vec2,
    pub zoom: f32,
    pub active: bool,
}

impl Camera {
    pub fn new(position: Vec2) -> Self {
        Self {
            position,
            zoom: 1.0,
            active: true,
        }
    }
}

/// Marker component for entities that should be removed
#[derive(Component, Debug, Clone, Default)]
#[storage(DenseVecStorage)]
pub struct MarkedForRemoval;

/// Animation component for animated sprites
#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Animation {
    pub current_frame: usize,
    pub frame_time: f32,
    pub frame_duration: f32,
    pub loop_animation: bool,
    pub frames: Vec<String>, // Sprite IDs for each frame
}

impl Animation {
    pub fn new(frames: Vec<String>, frame_duration: f32) -> Self {
        Self {
            current_frame: 0,
            frame_time: 0.0,
            frame_duration,
            loop_animation: true,
            frames,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.frame_time += delta_time;

        if self.frame_time >= self.frame_duration {
            self.frame_time = 0.0;
            self.current_frame += 1;

            if self.current_frame >= self.frames.len() {
                if self.loop_animation {
                    self.current_frame = 0;
                } else {
                    self.current_frame = self.frames.len() - 1;
                }
            }
        }
    }

    pub fn current_sprite(&self) -> &str {
        &self.frames[self.current_frame]
    }
}

/// Score component for tracking game scores
#[derive(Component, Debug, Clone)]
#[storage(DenseVecStorage)]
pub struct Score {
    pub player_score: u32,
    pub ai_score: u32,
}

impl Default for Score {
    fn default() -> Self {
        Self {
            player_score: 0,
            ai_score: 0,
        }
    }
}

/// Paddle component for Pong paddles
#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Paddle {
    pub player_controlled: bool,
}

/// Ball component for Pong ball
#[derive(Component, Debug, Clone, Default)]
#[storage(DenseVecStorage)]
pub struct Ball;
