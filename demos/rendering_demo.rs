//! Rendering Demo
//!
//! This demo showcases the rendering system with:
//! - Sprite rendering and animation
//! - Camera following and zooming
//! - Layered rendering
//! - Visual effects

use modular_game_engine::rendering::Camera2D;
use modular_game_engine::*;
use specs::{World, WorldExt};
use std::time::{Duration, Instant};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Rendering Demo ===");
    println!("Demonstrating sprite rendering, animation, and camera systems\n");

    // Initialize the game world
    let mut world = init()?;
    let mut dispatcher = specs::DispatcherBuilder::new()
        .with(PhysicsSystem, "physics", &[])
        .with(AnimationSystem, "animation", &[])
        .with(CameraSystem, "camera", &[])
        .with(
            RenderingSystem,
            "rendering",
            &["physics", "animation", "camera"],
        )
        .build();

    // Create rendering demo entities
    create_rendering_entities(&mut world);

    // Run the simulation
    let start_time = Instant::now();
    let mut frame_count = 0;

    println!("Running rendering simulation...");
    println!("Frame | Camera Pos | Visible Entities | Render Calls");
    println!("-------|-----------|-----------------|-------------");

    while start_time.elapsed() < Duration::from_secs(8) {
        let delta_time = 1.0 / 60.0; // 60 FPS

        // Update time resource
        world.write_resource::<Time>().delta = delta_time;
        world.write_resource::<Time>().elapsed = start_time.elapsed().as_secs_f32();

        // Update camera to follow player
        update_camera_follow(&mut world);

        // Run rendering systems
        dispatcher.dispatch(&world);
        world.maintain();

        // Print status every 20 frames
        if frame_count % 20 == 0 {
            print_rendering_status(&world, frame_count);
        }

        frame_count += 1;

        // Small delay
        std::thread::sleep(Duration::from_millis(16));
    }

    println!("\n=== Rendering Demo Complete ===");
    println!("Demonstrated:");
    println!("- Sprite rendering with different layers");
    println!("- Animation system with frame updates");
    println!("- Camera following and viewport management");
    println!("- Renderable component filtering");
    println!("- Performance monitoring");

    Ok(())
}

fn create_rendering_entities(world: &mut World) {
    println!("Creating rendering entities...");

    // Create player with animation
        let _player_entity = world
        .create_entity_with_components()
        .with(Position::new(0.0, 0.0))
        .with(Velocity::new(30.0, 20.0))
        .with(Renderable {
            sprite_id: "player_idle_0".to_string(),
            layer: 2,
            visible: true,
            scale: 1.0,
        })
        .with(Animation::new(
            vec![
                "player_idle_0".to_string(),
                "player_idle_1".to_string(),
                "player_idle_2".to_string(),
                "player_idle_3".to_string(),
            ],
            0.2, // 200ms per frame
        ))
        .with(Player {
            id: 1,
            health: 100.0,
            max_health: 100.0,
        })
        .build();

    // Create camera to follow player
    world
        .create_entity_with_components()
        .with(Position::new(0.0, 0.0))
        .with(Camera2D {
            position: Vec2::new(0.0, 0.0),
            zoom: 1.0,
            rotation: 0.0,
            viewport_size: Vec2::new(800.0, 600.0),
        })
        .with(Camera {
            position: Vec2::new(0.0, 0.0),
            zoom: 1.0,
            active: true,
        })
        .build();

    // Create background elements (layer 0)
    for i in 0..10 {
        let x = (i as f32 - 5.0) * 100.0;
        let y = 0.0;

        world
            .create_entity_with_components()
            .with(Position::new(x, y))
            .with(Renderable {
                sprite_id: "background_tile".to_string(),
                layer: 0,
                visible: true,
                scale: 1.0,
            })
            .build();
    }

    // Create foreground elements (layer 1)
    for i in 0..5 {
        let x = (i as f32 - 2.0) * 150.0;
        let y = -100.0 + (i as f32) * 30.0;

        world
            .create_entity_with_components()
            .with(Position::new(x, y))
            .with(Renderable {
                sprite_id: format!("foreground_{}", i),
                layer: 1,
                visible: true,
                scale: 0.8 + (i as f32) * 0.1,
            })
            .build();
    }

    // Create animated enemies (layer 2)
    for i in 0..3 {
        let x = 200.0 + (i as f32) * 80.0;
        let y = 50.0 + (i as f32) * 40.0;

        world
            .create_entity_with_components()
            .with(Position::new(x, y))
            .with(Velocity::new(-10.0, 0.0))
            .with(Renderable {
                sprite_id: format!("enemy_{}_0", i),
                layer: 2,
                visible: true,
                scale: 1.0,
            })
            .with(Animation::new(
                vec![
                    format!("enemy_{}_0", i),
                    format!("enemy_{}_1", i),
                    format!("enemy_{}_2", i),
                ],
                0.3,
            ))
            .with(Enemy::new(EnemyType::Basic))
            .build();
    }

    // Create UI elements (layer 10)
    world
        .create_entity_with_components()
        .with(Position::new(-350.0, -250.0))
        .with(Renderable {
            sprite_id: "health_bar_bg".to_string(),
            layer: 10,
            visible: true,
            scale: 1.0,
        })
        .build();

    world
        .create_entity_with_components()
        .with(Position::new(-350.0, -250.0))
        .with(Renderable {
            sprite_id: "health_bar_fg".to_string(),
            layer: 10,
            visible: true,
            scale: 1.0,
        })
        .build();

    let entity_count = world.entities().join().count();
    println!(
        "Created {} rendering entities across {} layers",
        entity_count, 11
    );
}

fn update_camera_follow(world: &mut World) {
    let players = world.read_storage::<Player>();
    let positions = world.read_storage::<Position>();
    let mut cameras = world.write_storage::<Camera2D>();

    // Find player position
    let player_pos = (&players, &positions)
        .join()
        .next()
        .map(|(_, pos)| pos.as_vec2())
        .unwrap_or(Vec2::new(0.0, 0.0));

    // Update camera to follow player with smoothing
    for camera in (&mut cameras).join() {
        let target_pos = player_pos;
        let current_pos = camera.position;

        // Simple linear interpolation for smooth following
        let lerp_factor = 0.05;
        camera.position = current_pos + (target_pos - current_pos) * lerp_factor;

        // Add some dynamic zoom based on player speed (simplified)
        camera.zoom = 1.0 + (player_pos.magnitude() * 0.001).min(0.5);
    }
}

fn print_rendering_status(world: &World, frame: u64) {
    let cameras = world.read_storage::<Camera2D>();
    let renderables = world.read_storage::<Renderable>();

    // Get active camera position
    let camera_pos = cameras
        .join()
        .next()
        .map(|cam| format!("({:6.1},{:6.1})", cam.position.x, cam.position.y))
        .unwrap_or("N/A".to_string());

    // Count visible entities
    let visible_count = renderables.join().filter(|r| r.visible).count();

    // Count render calls (simplified - one per visible entity)
    let render_calls = visible_count;

    println!(
        "{:6} | {} | {:15} | {:11}",
        frame, camera_pos, visible_count, render_calls
    );
}

// Additional systems for the rendering demo

use specs::{Join, Read, ReadStorage, System, WriteStorage};

/// Animation system for updating sprite animations
pub struct AnimationSystem;

impl<'a> System<'a> for AnimationSystem {
    type SystemData = (
        WriteStorage<'a, Animation>,
        WriteStorage<'a, Renderable>,
        Read<'a, Time>,
    );

    fn run(&mut self, (mut animations, mut renderables, time): Self::SystemData) {
        for (animation, renderable) in (&mut animations, &mut renderables).join() {
            animation.update(time.delta);

            // Update the sprite_id to current animation frame
            renderable.sprite_id = animation.current_sprite().to_string();
        }
    }
}

/// Camera system for updating camera positions
pub struct CameraSystem;

impl<'a> System<'a> for CameraSystem {
    type SystemData = (ReadStorage<'a, Camera2D>, WriteStorage<'a, Camera>);

    fn run(&mut self, (camera2ds, mut cameras): Self::SystemData) {
        for (camera2d, camera) in (&camera2ds, &mut cameras).join() {
            // Sync the old Camera component with new Camera2D
            camera.position = camera2d.position;
            camera.zoom = camera2d.zoom;
        }
    }
}

/// Enhanced rendering system with layer sorting
pub struct RenderingSystem;

impl<'a> System<'a> for RenderingSystem {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, Renderable>,
        ReadStorage<'a, Camera2D>,
        Read<'a, Time>,
    );

    fn run(&mut self, (positions, renderables, cameras, time): Self::SystemData) {
        // Get active camera
        let active_camera = cameras.join().next().cloned().unwrap_or(Camera2D {
            position: Vec2::new(0.0, 0.0),
            zoom: 1.0,
            rotation: 0.0,
            viewport_size: Vec2::new(800.0, 600.0),
        });

        // Collect and sort renderables by layer
        let mut render_queue: Vec<_> = (&positions, &renderables)
            .join()
            .filter(|(_, r)| r.visible)
            .collect();

        render_queue.sort_by_key(|(_, r)| r.layer);

        // Simulate rendering
        println!("--- Frame {} Rendering ---", time.elapsed as u32);

        for (position, renderable) in render_queue {
            // Apply camera transform (simplified)
            let camera_space_pos = position.as_vec2() - active_camera.position;

            // Check if entity is visible in camera viewport
            let half_width = active_camera.viewport_size.x / 2.0 / active_camera.zoom;
            let half_height = active_camera.viewport_size.y / 2.0 / active_camera.zoom;

            if camera_space_pos.x >= -half_width
                && camera_space_pos.x <= half_width
                && camera_space_pos.y >= -half_height
                && camera_space_pos.y <= half_height
            {
                println!(
                    "  Layer {}: {} at ({:.1}, {:.1}) [scale: {:.1}]",
                    renderable.layer,
                    renderable.sprite_id,
                    position.x,
                    position.y,
                    renderable.scale
                );
            }
        }

        println!("--- End Frame ---");
    }
}
