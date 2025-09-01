//! # Modular Game Engine
//!
//! A modular game engine built with Rust, featuring:
//! - Entity Component System (ECS) architecture
//! - Physics simulation
//! - Rendering pipeline
//! - Input handling
//! - AI systems
//! - Audio management
//! - UI framework
//! - Plugin system
//! - Event system

pub mod ai;
pub mod audio;
pub mod components;
pub mod difficulty;
pub mod ecs;
pub mod enhanced_ai;
pub mod events;
pub mod font;
pub mod game_loop;
pub mod game_state;
pub mod input;
pub mod input_window;
pub mod menu;
pub mod particles;
pub mod physics;
pub mod plugins;
pub mod renderer_2d;
pub mod rendering;
pub mod resources;
pub mod scoring;
pub mod systems;
pub mod trail_system;
pub mod ui;
pub mod visual_effects;
pub mod window;

pub use components::*;
pub use ecs::*;
pub use systems::*;

// Re-export commonly used types
pub use specs::{Entity, Join, World, WorldExt};

// Type aliases for convenience
pub type Vec2 = nalgebra::Vector2<f32>;
pub type Vec3 = nalgebra::Vector3<f32>;
pub type Mat4 = nalgebra::Matrix4<f32>;
pub type Point2 = nalgebra::Point2<f32>;
pub type Point3 = nalgebra::Point3<f32>;

/// Initialize the game engine with default systems
pub fn init() -> Result<World, Box<dyn std::error::Error>> {
    let mut world = World::new();

    // Register core components
    world.register::<Position>();
    world.register::<Velocity>();
    world.register::<Acceleration>();
    world.register::<Renderable>();
    world.register::<Player>();
    world.register::<Enemy>();
    world.register::<Health>();
    world.register::<Collider>();
    world.register::<Camera>();
    world.register::<MarkedForRemoval>();
    world.register::<Score>();
    world.register::<Paddle>();
    world.register::<Ball>();

    // Register physics components
    world.register::<physics::Mass>();
    world.register::<physics::Force>();
    world.register::<physics::PhysicsMaterial>();

    // Register rendering components
    world.register::<rendering::Camera2D>();
    world.register::<rendering::Sprite>();

    // Register animation components
    world.register::<Animation>();

    // Add core resources
    world.insert(Time::default());
    world.insert(InputState::default());
    world.insert(Score::default());

    Ok(world)
}

/// Main game structure
pub struct Game {
    pub world: World,
    pub dispatcher: specs::Dispatcher<'static, 'static>,
}

impl Game {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let world = init()?;

        // Create dispatcher with core systems
        let dispatcher = specs::DispatcherBuilder::new()
            .with(PhysicsSystem, "physics", &[])
            .with(RenderingSystem, "rendering", &["physics"])
            .with(InputSystem, "input", &[])
            .build();

        Ok(Self { world, dispatcher })
    }

    pub fn update(&mut self, delta_time: f32) {
        // Update time
        self.world.write_resource::<Time>().delta = delta_time;
        self.world.write_resource::<Time>().elapsed += delta_time;

        // Run systems
        self.dispatcher.dispatch(&self.world);
        self.world.maintain();
    }
}
