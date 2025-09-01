//! Pong Demo
//!
//! A complete Pong game implementation using the modular game engine.
//! Demonstrates ECS, Physics, Input, and Game Logic integration.

use modular_game_engine::*;
use specs::{World, WorldExt, RunNow, Component, VecStorage, DenseVecStorage};
use std::time::{Duration, Instant};

// Game constants
const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;
const PADDLE_WIDTH: f32 = 20.0;
const PADDLE_HEIGHT: f32 = 100.0;
const BALL_SIZE: f32 = 15.0;
const PADDLE_SPEED: f32 = 300.0;
const BALL_SPEED: f32 = 400.0;

// Game state
#[derive(Debug, Clone)]
pub struct GameState {
    player_score: u32,
    ai_score: u32,
    game_phase: GamePhase,
}

#[derive(Debug, Clone, PartialEq)]
enum GamePhase {
    Playing,
    Scored,
    Reset,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            player_score: 0,
            ai_score: 0,
            game_phase: GamePhase::Playing,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎮 Pong Demo - Modular Game Engine");
    println!("==================================");
    println!("Controls: W/S keys to move paddle");
    println!("Goal: First to 5 points wins!");
    println!();

    // Initialize the game world
    let mut world = init()?;

    // Add game-specific components
    world.register::<Paddle>();
    world.register::<Ball>();
    world.register::<Score>();

    // Register game state as resource
    world.insert(GameState::default());



    // Create game entities
    create_pong_entities(&mut world);

    // Set up systems
    let mut dispatcher = specs::DispatcherBuilder::new()
        .with(PongInputSystem, "input", &[])
        .with(PongAISystem, "ai", &["input"])
        .with(PhysicsSystem, "physics", &["ai"])
        .with(PongCollisionSystem, "collision", &["physics"])
        .with(PongGameLogicSystem, "game_logic", &["collision"])
        .with(PongRenderingSystem, "rendering", &["game_logic"])
        .build();

    // Game loop
    let mut frame_count = 0;
    let start_time = Instant::now();

    println!("Player: 0 | AI: 0");
    println!("---------");

    loop {
        let delta_time = 1.0 / 60.0; // 60 FPS

        // Update time resource
        world.write_resource::<Time>().delta = delta_time;
        world.write_resource::<Time>().elapsed = start_time.elapsed().as_secs_f32();

        // Run game systems
        dispatcher.dispatch(&world);
        world.maintain();

        // Display game state every 60 frames (1 second)
        if frame_count % 60 == 0 {
            let game_state = world.read_resource::<GameState>();
            print!("\rPlayer: {} | AI: {} | ", game_state.player_score, game_state.ai_score);

            match game_state.game_phase {
                GamePhase::Playing => print!("Playing"),
                GamePhase::Scored => print!("Point Scored!"),
                GamePhase::Reset => print!("Resetting..."),
            }

            // Check for game end
            if game_state.player_score >= 5 || game_state.ai_score >= 5 {
                println!("\n🎉 Game Over!");
                if game_state.player_score >= 5 {
                    println!("🏆 You Win!");
                } else {
                    println!("🤖 AI Wins!");
                }
                break;
            }
        }

        frame_count += 1;

        // Small delay to maintain 60 FPS
        std::thread::sleep(Duration::from_millis(16));
    }

    println!("\n=== Pong Demo Complete ===");
    println!("Demonstrated:");
    println!("- Complete game implementation using modular systems");
    println!("- ECS entity management for game objects");
    println!("- Physics-based ball movement and collisions");
    println!("- Input handling for player control");
    println!("- Simple AI for computer opponent");
    println!("- Game state management and scoring");
    println!("- Real-time game loop with proper timing");

    Ok(())
}

fn create_pong_entities(world: &mut World) {
    println!("Creating Pong entities...");

    // Create player paddle (left side)
    world.create_entity_with_components()
        .with(Position::new(50.0, SCREEN_HEIGHT / 2.0 - PADDLE_HEIGHT / 2.0))
        .with(Velocity::new(0.0, 0.0))
        .with(Renderable::new("player_paddle".to_string()))
        .with(Paddle { player_controlled: true })
        .with(Collider::new_rectangle(PADDLE_WIDTH, PADDLE_HEIGHT))
        .build();

    // Create AI paddle (right side)
    world.create_entity_with_components()
        .with(Position::new(SCREEN_WIDTH - 50.0 - PADDLE_WIDTH, SCREEN_HEIGHT / 2.0 - PADDLE_HEIGHT / 2.0))
        .with(Velocity::new(0.0, 0.0))
        .with(Renderable::new("ai_paddle".to_string()))
        .with(Paddle { player_controlled: false })
        .with(Collider::new_rectangle(PADDLE_WIDTH, PADDLE_HEIGHT))
        .build();

    // Create ball (center)
    world.create_entity_with_components()
        .with(Position::new(SCREEN_WIDTH / 2.0 - BALL_SIZE / 2.0, SCREEN_HEIGHT / 2.0 - BALL_SIZE / 2.0))
        .with(Velocity::new(BALL_SPEED, BALL_SPEED * 0.5))
        .with(Renderable::new("ball".to_string()))
        .with(Ball)
        .with(Collider::new_circle(BALL_SIZE / 2.0))
        .build();

    // Create score entities (for display)
    world.create_entity_with_components()
        .with(Score { player_score: 0, ai_score: 0 })
        .build();

    println!("Created: Player paddle, AI paddle, Ball, Score system");
}

// Game-specific components
#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Paddle {
    pub player_controlled: bool,
}

#[derive(Component, Debug, Clone, Default)]
#[storage(DenseVecStorage)]
pub struct Ball;

#[derive(Component, Debug, Clone)]
#[storage(DenseVecStorage)]
pub struct Score {
    pub player_score: u32,
    pub ai_score: u32,
}

// Game systems
use specs::{System, ReadStorage, WriteStorage, Read, Write, Entities, Join};

/// Input system for Pong
pub struct PongInputSystem;

impl<'a> System<'a> for PongInputSystem {
    type SystemData = (
        WriteStorage<'a, Velocity>,
        ReadStorage<'a, Paddle>,
        Read<'a, crate::InputState>,
    );

    fn run(&mut self, (mut velocities, paddles, input_state): Self::SystemData) {
        for (velocity, paddle) in (&mut velocities, &paddles).join() {
            if paddle.player_controlled {
                // Player paddle control
                velocity.y = 0.0;

                if input_state.keys_pressed.contains(&winit::event::VirtualKeyCode::W) {
                    velocity.y = -PADDLE_SPEED;
                }
                if input_state.keys_pressed.contains(&winit::event::VirtualKeyCode::S) {
                    velocity.y = PADDLE_SPEED;
                }
            }
        }
    }
}

/// AI system for computer paddle
pub struct PongAISystem;

impl<'a> System<'a> for PongAISystem {
    type SystemData = (
        ReadStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
        ReadStorage<'a, Paddle>,
        ReadStorage<'a, Ball>,
        Read<'a, Time>,
    );

    fn run(&mut self, (positions, mut velocities, paddles, balls, time): Self::SystemData) {
        // Find ball position
        let ball_pos = balls.join()
            .next()
            .and_then(|_| positions.join().next())
            .map(|pos| pos.as_vec2())
            .unwrap_or(Vec2::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0));

        // Control AI paddle
        for (position, velocity, paddle) in (&positions, &mut velocities, &paddles).join() {
            if !paddle.player_controlled {
                // Simple AI: follow ball with some delay
                let paddle_center = position.y + PADDLE_HEIGHT / 2.0;
                let ball_center = ball_pos.y;
                let diff = ball_center - paddle_center;

                // Add some imperfection to make it beatable
                let ai_error = (time.elapsed * 2.0).sin() * 20.0;
                let target_diff = diff + ai_error;

                if target_diff.abs() > 10.0 {
                    velocity.y = target_diff.signum() * PADDLE_SPEED * 0.8; // Slightly slower than player
                } else {
                    velocity.y = 0.0;
                }
            }
        }
    }
}

/// Collision system for Pong
pub struct PongCollisionSystem;

impl<'a> System<'a> for PongCollisionSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
        ReadStorage<'a, Ball>,
        ReadStorage<'a, Paddle>,
        Write<'a, GameState>,
    );

    fn run(&mut self, (entities, positions, mut velocities, balls, paddles, mut game_state): Self::SystemData) {
        for (ball_entity, ball_pos, _) in (&entities, &positions, &balls).join() {
            // Ball collision with top/bottom walls
            if ball_pos.y <= 0.0 || ball_pos.y >= SCREEN_HEIGHT - BALL_SIZE {
                if let Some(vel) = velocities.get_mut(ball_entity) {
                    vel.y = -vel.y;
                }
            }

            // Ball collision with paddles
            let mut ball_hit = false;
            let mut new_ball_vel = None;

            for (paddle_pos, _, _) in (&positions, &velocities, &paddles).join() {
                if check_paddle_ball_collision(ball_pos, paddle_pos) {
                    ball_hit = true;
                    // Calculate new velocity
                    let current_vel = velocities.get(ball_entity).unwrap();
                    let mut new_vel = Velocity { x: -current_vel.x, y: current_vel.y };

                    // Add spin based on where ball hits paddle
                    let paddle_center = paddle_pos.y + PADDLE_HEIGHT / 2.0;
                    let hit_pos = ball_pos.y + BALL_SIZE / 2.0;
                    let spin_factor = (hit_pos - paddle_center) / (PADDLE_HEIGHT / 2.0);
                    new_vel.y += spin_factor * 100.0;

                    // Ensure ball doesn't get too fast
                    let speed = (new_vel.x * new_vel.x + new_vel.y * new_vel.y).sqrt();
                    if speed > BALL_SPEED * 1.5 {
                        new_vel.x = new_vel.x / speed * BALL_SPEED * 1.2;
                        new_vel.y = new_vel.y / speed * BALL_SPEED * 1.2;
                    }

                    new_ball_vel = Some(new_vel);
                    break; // Only handle first collision
                }
            }

            // Apply velocity change after the loop
            if let Some(new_vel) = new_ball_vel {
                if let Some(vel) = velocities.get_mut(ball_entity) {
                    *vel = new_vel;
                }
            }

            // Check for scoring (ball goes off screen)
            if ball_pos.x < -BALL_SIZE {
                // AI scores
                game_state.ai_score += 1;
                game_state.game_phase = GamePhase::Scored;
                reset_ball(&mut velocities, &balls);
            } else if ball_pos.x > SCREEN_WIDTH {
                // Player scores
                game_state.player_score += 1;
                game_state.game_phase = GamePhase::Scored;
                reset_ball(&mut velocities, &balls);
            }
        }
    }
}

fn check_paddle_ball_collision(ball_pos: &Position, paddle_pos: &Position) -> bool {
    ball_pos.x < paddle_pos.x + PADDLE_WIDTH &&
    ball_pos.x + BALL_SIZE > paddle_pos.x &&
    ball_pos.y < paddle_pos.y + PADDLE_HEIGHT &&
    ball_pos.y + BALL_SIZE > paddle_pos.y
}

fn reset_ball(velocities: &mut WriteStorage<Velocity>, balls: &ReadStorage<Ball>) {
    for (vel, _) in (velocities, balls).join() {
        // Reset ball position and give it a random direction
        vel.x = BALL_SPEED * (if rand::random::<bool>() { 1.0 } else { -1.0 });
        vel.y = (rand::random::<f32>() - 0.5) * BALL_SPEED * 0.5;
    }
}

/// Game logic system
pub struct PongGameLogicSystem;

impl<'a> System<'a> for PongGameLogicSystem {
    type SystemData = (
        WriteStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
        ReadStorage<'a, Ball>,
        Write<'a, GameState>,
        Read<'a, Time>,
    );

    fn run(&mut self, (mut positions, mut velocities, balls, mut game_state, time): Self::SystemData) {
        match game_state.game_phase {
            GamePhase::Scored => {
                // Wait a moment before resetting
                if time.elapsed % 2.0 < time.delta {
                    game_state.game_phase = GamePhase::Reset;
                }
            }
            GamePhase::Reset => {
                // Reset ball to center
                for (pos, _) in (&mut positions, &balls).join() {
                    pos.x = SCREEN_WIDTH / 2.0 - BALL_SIZE / 2.0;
                    pos.y = SCREEN_HEIGHT / 2.0 - BALL_SIZE / 2.0;
                }

                // Small delay before starting
                if time.elapsed % 1.0 < time.delta {
                    game_state.game_phase = GamePhase::Playing;
                }
            }
            GamePhase::Playing => {
                // Game is active
            }
        }
    }
}

/// Rendering system for Pong
pub struct PongRenderingSystem;

impl<'a> System<'a> for PongRenderingSystem {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, Renderable>,
        ReadStorage<'a, Paddle>,
        ReadStorage<'a, Ball>,
        Read<'a, GameState>,
        Read<'a, Time>,
    );

    fn run(&mut self, (positions, renderables, paddles, balls, game_state, time): Self::SystemData) {
        // Simple console-based rendering
        if time.elapsed % 1.0 < time.delta {
            println!("\n=== Pong Game State ===");

            // Display paddle positions
            for (pos, renderable, paddle) in (&positions, &renderables, &paddles).join() {
                let paddle_type = if paddle.player_controlled { "PLAYER" } else { "AI" };
                println!("{} Paddle: ({:.0}, {:.0})", paddle_type, pos.x, pos.y);
            }

            // Display ball position
            for (pos, renderable, _) in (&positions, &renderables, &balls).join() {
                println!("Ball: ({:.0}, {:.0})", pos.x, pos.y);
            }

            println!("Score - Player: {} | AI: {}", game_state.player_score, game_state.ai_score);
            println!("====================");
        }
    }
}