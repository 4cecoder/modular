//! Physics Demo
//!
//! This demo showcases the physics system with:
//! - Movement and acceleration
//! - Collision detection
//! - Force application
//! - Physics materials

use modular_game_engine::*;
use modular_game_engine::physics::{Mass, Force, PhysicsMaterial};
use specs::{World, WorldExt, RunNow};
use std::time::{Duration, Instant};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Physics Demo ===");
    println!("Demonstrating physics simulation and collision detection\n");

    // Initialize the game world
    let mut world = init()?;
    let mut dispatcher = specs::DispatcherBuilder::new()
        .with(PhysicsSystem, "physics", &[])
        .with(CollisionSystem, "collision", &["physics"])
        .with(PhysicsDebugSystem, "debug", &["collision"])
        .build();

    // Create physics demo entities
    create_physics_entities(&mut world);

    // Run the simulation
    let start_time = Instant::now();
    let mut frame_count = 0;

    println!("Running physics simulation...");
    println!("Frame | Ball 1 Pos | Ball 2 Pos | Collisions");
    println!("-------|-----------|-----------|-----------");

    while start_time.elapsed() < Duration::from_secs(10) {
        let delta_time = 1.0 / 60.0; // 60 FPS

        // Update time resource
        world.write_resource::<Time>().delta = delta_time;
        world.write_resource::<Time>().elapsed = start_time.elapsed().as_secs_f32();

        // Apply some forces for demonstration
        apply_demo_forces(&mut world, start_time.elapsed().as_secs_f32());

        // Run physics systems
        dispatcher.dispatch(&world);
        world.maintain();

        // Print status every 30 frames
        if frame_count % 30 == 0 {
            print_physics_status(&world, frame_count);
        }

        frame_count += 1;

        // Small delay
        std::thread::sleep(Duration::from_millis(16));
    }

    println!("\n=== Physics Demo Complete ===");
    println!("Demonstrated:");
    println!("- Entity movement with velocity and acceleration");
    println!("- Force application and physics materials");
    println!("- Collision detection and response");
    println!("- Physics debugging and monitoring");

    Ok(())
}

fn create_physics_entities(world: &mut World) {
    println!("Creating physics entities...");

    // Create bouncing balls with different properties
    let ball_configs = vec![
        ("ball1", 0.0, 0.0, 50.0, 20.0, 0.9, 1.0),  // High restitution (bouncy)
        ("ball2", 100.0, 0.0, -30.0, 15.0, 0.3, 2.0), // Low restitution, heavier
        ("ball3", -50.0, -50.0, 25.0, 10.0, 0.7, 0.5), // Medium, lighter
    ];

    for (name, x, y, vx, vy, restitution, mass) in &ball_configs {
        world.create_entity_with_components()
            .with(Position::new(*x, *y))
            .with(Velocity::new(*vx, *vy))
            .with(Acceleration::new(0.0, 98.0)) // Gravity
            .with(Renderable::new(name.to_string()))
            .with(Mass(*mass))
            .with(Force(Vec2::new(0.0, 0.0)))
            .with(PhysicsMaterial {
                restitution: *restitution,
                friction: 0.1,
                density: 1.0,
            })
            .with(Collider::new_circle(10.0))
            .build();
    }

    // Create static platforms
    let platform_configs = vec![
        ("platform1", 0.0, 100.0, 200.0, 20.0),
        ("platform2", -150.0, 50.0, 100.0, 20.0),
        ("platform3", 150.0, -20.0, 100.0, 20.0),
    ];

    for (name, x, y, width, height) in &platform_configs {
        world.create_entity_with_components()
            .with(Position::new(*x, *y))
            .with(Renderable::new(name.to_string()))
            .with(Collider::new_rectangle(*width, *height))
            .build();
    }

    println!("Created {} physics entities", ball_configs.len() + platform_configs.len());
}

fn apply_demo_forces(world: &mut World, elapsed_time: f32) {
    let mut forces = world.write_storage::<Force>();
    let positions = world.read_storage::<Position>();

    // Apply oscillating force to first ball
    for (force, _) in (&mut forces, &positions).join() {
        let magnitude = (elapsed_time * 2.0).sin() * 50.0;
        force.0.x = magnitude;
    }
}

fn print_physics_status(world: &World, frame: u64) {
    let positions = world.read_storage::<Position>();
    let renderables = world.read_storage::<Renderable>();

    // Find ball positions
    let mut ball_positions = Vec::new();
    for (position, renderable) in (&positions, &renderables).join() {
        if renderable.sprite_id.starts_with("ball") {
            ball_positions.push((renderable.sprite_id.clone(), position.x, position.y));
        }
    }

    ball_positions.sort_by(|a, b| a.0.cmp(&b.0));

    let ball1_pos = ball_positions.get(0)
        .map(|(_, x, y)| format!("({:6.1},{:6.1})", x, y))
        .unwrap_or("N/A".to_string());

    let ball2_pos = ball_positions.get(1)
        .map(|(_, x, y)| format!("({:6.1},{:6.1})", x, y))
        .unwrap_or("N/A".to_string());

    // Count entities (simplified collision detection)
    let entity_count = positions.join().count();

    println!("{:6} | {} | {} | {:9}",
             frame, ball1_pos, ball2_pos, entity_count);
}

// Additional systems for the physics demo

use specs::{System, ReadStorage, WriteStorage, Entities, Join};

/// Simple collision detection system
pub struct CollisionSystem;

impl<'a> System<'a> for CollisionSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Collider>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, (entities, positions, colliders, mut velocities): Self::SystemData) {
        // Simple collision detection between dynamic objects
        let mut dynamic_entities = Vec::new();

        // Collect dynamic entities (those with velocity)
        for (entity, position, collider, velocity) in (&entities, &positions, &colliders, &velocities).join() {
            dynamic_entities.push((entity, position.clone(), collider.clone(), velocity.clone()));
        }

        // Check collisions between dynamic entities
        for i in 0..dynamic_entities.len() {
            for j in (i + 1)..dynamic_entities.len() {
                let (entity_a, pos_a, collider_a, vel_a) = &dynamic_entities[i];
                let (entity_b, pos_b, collider_b, vel_b) = &dynamic_entities[j];

                if check_collision(pos_a, collider_a, pos_b, collider_b) {
                    // Simple collision response - reverse velocities
                    if let Some(vel_a) = velocities.get_mut(*entity_a) {
                        vel_a.x = -vel_a.x * 0.8; // Some energy loss
                        vel_a.y = -vel_a.y * 0.8;
                    }
                    if let Some(vel_b) = velocities.get_mut(*entity_b) {
                        vel_b.x = -vel_b.x * 0.8;
                        vel_b.y = -vel_b.y * 0.8;
                    }
                }
            }
        }
    }
}

fn check_collision(pos_a: &Position, collider_a: &Collider, pos_b: &Position, collider_b: &Collider) -> bool {
    match (&collider_a.shape, &collider_b.shape) {
        (CollisionShape::Circle { radius: r1 }, CollisionShape::Circle { radius: r2 }) => {
            let distance = ((pos_a.x - pos_b.x).powi(2) + (pos_a.y - pos_b.y).powi(2)).sqrt();
            distance < (r1 + r2)
        }
        _ => false, // Simplified - only circle-circle collision for demo
    }
}

/// Debug system for physics
pub struct PhysicsDebugSystem;

impl<'a> System<'a> for PhysicsDebugSystem {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, Velocity>,
        ReadStorage<'a, Force>,
        ReadStorage<'a, Renderable>,
    );

    fn run(&mut self, (positions, velocities, forces, renderables): Self::SystemData) {
        // Debug output for physics entities
        for (position, velocity, force, renderable) in (&positions, &velocities, &forces, &renderables).join() {
            if renderable.sprite_id.starts_with("ball") {
                println!("{}: pos=({:.1},{:.1}) vel=({:.1},{:.1}) force=({:.1},{:.1})",
                    renderable.sprite_id,
                    position.x, position.y,
                    velocity.x, velocity.y,
                    force.0.x, force.0.y);
            }
        }
    }
}