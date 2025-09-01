//! ECS Demo
//!
//! This demo showcases the Entity Component System functionality.
//! It creates entities with different components and runs systems on them.

use modular_game_engine::*;
use specs::{World, WorldExt, RunNow};
use std::time::{Duration, Instant};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== ECS Demo ===");
    println!("Demonstrating Entity Component System functionality\n");

    // Initialize the game world
    let mut world = init()?;
    let mut dispatcher = specs::DispatcherBuilder::new()
        .with(PhysicsSystem, "physics", &[])
        .with(HealthSystem, "health", &[])
        .with(CleanupSystem, "cleanup", &[])
        .with(DebugSystem, "debug", &[])
        .build();

    // Create some demo entities
    create_demo_entities(&mut world);

    // Run the simulation for a few seconds
    let start_time = Instant::now();
    let mut frame_count = 0;

    println!("Running simulation...");
    println!("Frame | Entities | Player Health | Enemy Count");
    println!("-------|----------|---------------|------------");

    while start_time.elapsed() < Duration::from_secs(5) {
        let delta_time = 1.0 / 60.0; // 60 FPS

        // Update time resource
        world.write_resource::<Time>().delta = delta_time;
        world.write_resource::<Time>().elapsed = start_time.elapsed().as_secs_f32();

        // Run systems
        dispatcher.dispatch(&world);
        world.maintain();

        // Print status every 10 frames
        if frame_count % 10 == 0 {
            print_status(&world, frame_count);
        }

        frame_count += 1;

        // Small delay to not spam output
        std::thread::sleep(Duration::from_millis(16));
    }

    println!("\n=== Demo Complete ===");
    println!("Created and managed entities using ECS architecture");
    println!("Systems processed: Physics, Health, Cleanup, Debug");

    Ok(())
}

fn create_demo_entities(world: &mut World) {
    println!("Creating demo entities...");

    // Create player entity
    world.create_entity_with_components()
        .with(Position::new(0.0, 0.0))
        .with(Velocity::new(10.0, 5.0))
        .with(Renderable::new("player_sprite".to_string()))
        .with(Player { id: 1, health: 100.0, max_health: 100.0 })
        .with(Health::new(100.0))
        .build();

    // Create enemy entities
    for i in 0..5 {
        let x = (i as f32 - 2.0) * 50.0;
        let y = 100.0 + (i as f32) * 20.0;

        world.create_entity_with_components()
            .with(Position::new(x, y))
            .with(Velocity::new(-5.0 + (i as f32), 0.0))
            .with(Renderable::new(format!("enemy_sprite_{}", i)))
            .with(Enemy::new(EnemyType::Basic))
            .with(Health::new(50.0))
            .build();
    }

    // Create static objects
    for i in 0..3 {
        let x = (i as f32 - 1.0) * 100.0;
        let y = -50.0;

        world.create_entity_with_components()
            .with(Position::new(x, y))
            .with(Renderable::new(format!("static_object_{}", i)))
            .build();
    }

    let entity_count = world.entities().join().count();
    println!("Created {} entities total", entity_count);
}

fn print_status(world: &World, frame: u64) {
    let positions = world.read_storage::<Position>();
    let players = world.read_storage::<Player>();
    let enemies = world.read_storage::<Enemy>();
    let healths = world.read_storage::<Health>();

    let entity_count = positions.join().count();
    let enemy_count = enemies.join().count();

        // Get player health
        let player_health = (&players, &healths).join()
            .next()
            .map(|(_, health)| format!("{:.0}/{}", health.current, health.maximum))
            .unwrap_or("No player".to_string());

    println!("{:6} | {:8} | {:13} | {:10}",
             frame, entity_count, player_health, enemy_count);
}