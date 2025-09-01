//! Game systems
//!
//! This module contains all the core systems that operate on components.

use crate::{Acceleration, Health, MarkedForRemoval, Position, Time, Velocity};
use specs::{Entities, Join, Read, ReadStorage, System, WriteStorage};

/// Physics system for movement and physics simulation
pub struct PhysicsSystem;

impl<'a> System<'a> for PhysicsSystem {
    type SystemData = (
        WriteStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
        ReadStorage<'a, Acceleration>,
        Read<'a, Time>,
    );

    fn run(&mut self, (mut positions, mut velocities, accelerations, time): Self::SystemData) {
        // Update velocities based on acceleration
        for (velocity, acceleration) in (&mut velocities, &accelerations).join() {
            velocity.x += acceleration.x * time.delta;
            velocity.y += acceleration.y * time.delta;
        }

        // Update positions based on velocity
        for (position, velocity) in (&mut positions, &velocities).join() {
            position.x += velocity.x * time.delta;
            position.y += velocity.y * time.delta;
        }
    }
}

/// Cleanup system for removing dead entities
pub struct CleanupSystem;

impl<'a> System<'a> for CleanupSystem {
    type SystemData = (Entities<'a>, ReadStorage<'a, MarkedForRemoval>);

    fn run(&mut self, (entities, marked): Self::SystemData) {
        for (entity, _) in (&entities, &marked).join() {
            entities.delete(entity).unwrap();
        }
    }
}

/// Health system for managing entity health
pub struct HealthSystem;

impl<'a> System<'a> for HealthSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Health>,
        WriteStorage<'a, MarkedForRemoval>,
    );

    fn run(&mut self, (entities, mut healths, mut marked): Self::SystemData) {
        for (entity, health) in (&entities, &mut healths).join() {
            if !health.is_alive() {
                marked.insert(entity, MarkedForRemoval).unwrap();
            }
        }
    }
}

/// Debug system for logging game state
pub struct DebugSystem;

impl<'a> System<'a> for DebugSystem {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, Velocity>,
        ReadStorage<'a, Health>,
        Read<'a, Time>,
    );

    fn run(&mut self, (positions, _velocities, _healths, time): Self::SystemData) {
        // Only log every second
        if time.elapsed % 1.0 < time.delta {
            let entity_count = positions.join().count();
            println!(
                "Frame time: {:.2}ms, Entities: {}",
                time.delta * 1000.0,
                entity_count
            );
        }
    }
}

/// System for basic AI behavior (placeholder)
pub struct AISystem;

impl<'a> System<'a> for AISystem {
    type SystemData = (
        ReadStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
        Read<'a, Time>,
    );

    fn run(&mut self, (positions, mut velocities, _time): Self::SystemData) {
        // Simple AI: move towards origin
        for (position, velocity) in (&positions, &mut velocities).join() {
            let direction = (-position.as_vec2()).normalize();
            velocity.x = direction.x * 50.0; // Simple speed
            velocity.y = direction.y * 50.0;
        }
    }
}

/// System for handling input (placeholder)
pub struct InputSystem;

impl<'a> System<'a> for InputSystem {
    type SystemData = (
        Read<'a, crate::InputState>,
        WriteStorage<'a, Velocity>,
        ReadStorage<'a, crate::Player>,
    );

    fn run(&mut self, (input_state, mut velocities, players): Self::SystemData) {
        // Simple input handling
        for (velocity, _) in (&mut velocities, &players).join() {
            velocity.x = 0.0;
            velocity.y = 0.0;

            // Check for arrow keys (simplified)
            if input_state
                .keys_pressed
                .contains(&winit::event::VirtualKeyCode::Left)
            {
                velocity.x = -100.0;
            }
            if input_state
                .keys_pressed
                .contains(&winit::event::VirtualKeyCode::Right)
            {
                velocity.x = 100.0;
            }
            if input_state
                .keys_pressed
                .contains(&winit::event::VirtualKeyCode::Up)
            {
                velocity.y = -100.0;
            }
            if input_state
                .keys_pressed
                .contains(&winit::event::VirtualKeyCode::Down)
            {
                velocity.y = 100.0;
            }
        }
    }
}

/// System for rendering (placeholder)
pub struct RenderingSystem;

impl<'a> System<'a> for RenderingSystem {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, crate::Renderable>,
        Read<'a, Time>,
    );

    fn run(&mut self, (positions, renderables, _time): Self::SystemData) {
        // Simple rendering simulation
        for (position, renderable) in (&positions, &renderables).join() {
            if renderable.visible {
                // In a real implementation, this would render the sprite
                println!(
                    "Rendering {} at ({:.1}, {:.1})",
                    renderable.sprite_id, position.x, position.y
                );
            }
        }
    }
}
