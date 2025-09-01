//! Simple Graphical Pong Demo
//!
//! A graphical Pong game with simple colored rectangles.
//! Demonstrates the modular engine with basic graphics.

use modular_game_engine::*;
use std::time::Instant;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

// Game constants
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const PADDLE_WIDTH: f32 = 20.0;
const PADDLE_HEIGHT: f32 = 100.0;
const BALL_SIZE: f32 = 15.0;
const PADDLE_SPEED: f32 = 300.0;
const BALL_SPEED: f32 = 400.0;

// Game state
#[derive(Debug, Clone, PartialEq)]
pub enum GameState {
    Menu,
    Playing,
    Paused,
    GameOver { winner: String },
}

pub struct SimplePongGame {
    world: World,
    dispatcher: specs::Dispatcher<'static, 'static>,
    game_state: GameState,
    last_update: Instant,
    score: (u32, u32), // (player, ai)
}

impl SimplePongGame {
    fn new() -> Self {
        let mut world = init().unwrap();

        // Register Pong-specific components
        world.register::<Paddle>();
        world.register::<Ball>();
        world.register::<Score>();

        // Create game entities
        create_pong_entities(&mut world);

        // Set up systems
        let dispatcher = specs::DispatcherBuilder::new()
            .with(PongInputSystem, "input", &[])
            .with(PongAISystem, "ai", &["input"])
            .with(PhysicsSystem, "physics", &["ai"])
            .with(PongCollisionSystem, "collision", &["physics"])
            .with(PongGameLogicSystem, "game_logic", &["collision"])
            .build();

        Self {
            world,
            dispatcher,
            game_state: GameState::Menu,
            last_update: Instant::now(),
            score: (0, 0),
        }
    }

    fn update(&mut self) {
        let now = Instant::now();
        let delta_time = now.duration_since(self.last_update).as_secs_f32();
        self.last_update = now;

        // Update time resource
        self.world.write_resource::<Time>().delta = delta_time;
        self.world.write_resource::<Time>().elapsed += delta_time;

        match self.game_state {
            GameState::Playing => {
                // Run game systems
                self.dispatcher.dispatch(&self.world);
                self.world.maintain();

                // Update score from world
                let score_resource = self.world.read_resource::<Score>();
                self.score = (score_resource.player_score, score_resource.ai_score);

                // Check for game end
                if self.score.0 >= 5 {
                    self.game_state = GameState::GameOver {
                        winner: "Player".to_string(),
                    };
                } else if self.score.1 >= 5 {
                    self.game_state = GameState::GameOver {
                        winner: "AI".to_string(),
                    };
                }
            }
            _ => {}
        }
    }

    fn start_game(&mut self) {
        self.game_state = GameState::Playing;
        self.score = (0, 0);

        // Reset score in world and ball position
        self.world.write_resource::<Score>().player_score = 0;
        self.world.write_resource::<Score>().ai_score = 0;

        reset_ball(&mut self.world);
    }

    fn reset_game(&mut self) {
        self.game_state = GameState::Menu;
        self.score = (0, 0);
        reset_ball(&mut self.world);
    }

    fn get_game_objects(&self) -> Vec<GameObject> {
        let mut objects = Vec::new();

        let positions = self.world.read_storage::<Position>();
        let renderables = self.world.read_storage::<Renderable>();
        let paddles = self.world.read_storage::<Paddle>();
        let balls = self.world.read_storage::<Ball>();

        // Add paddles
        for (pos, _, paddle) in (&positions, &renderables, &paddles).join() {
            let color = if paddle.player_controlled {
                [0.0, 0.8, 0.0] // Green for player
            } else {
                [0.8, 0.0, 0.0] // Red for AI
            };
            objects.push(GameObject {
                x: pos.x,
                y: pos.y,
                width: PADDLE_WIDTH,
                height: PADDLE_HEIGHT,
                color,
            });
        }

        // Add ball
        for (pos, _, _) in (&positions, &renderables, &balls).join() {
            objects.push(GameObject {
                x: pos.x,
                y: pos.y,
                width: BALL_SIZE,
                height: BALL_SIZE,
                color: [1.0, 1.0, 1.0], // White ball
            });
        }

        objects
    }
}

#[derive(Debug, Clone)]
pub struct GameObject {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub color: [f32; 3],
}

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Simple Graphical Pong - Modular Game Engine")
        .with_inner_size(winit::dpi::PhysicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT))
        .build(&event_loop)
        .unwrap();

    let mut pong_game = SimplePongGame::new();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent { event, window_id } if window_id == window.id() => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput { input, .. } => {
                    handle_keyboard_input(&mut pong_game, input);
                }
                _ => {}
            },
            Event::MainEventsCleared => {
                pong_game.update();
                window.request_redraw();
            }
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                // Simple console-based rendering for now
                render_game(&pong_game);
            }
            _ => {}
        }
    });
}

fn handle_keyboard_input(game: &mut SimplePongGame, input: KeyboardInput) {
    if let Some(keycode) = input.virtual_keycode {
        match (keycode, input.state) {
            (VirtualKeyCode::Escape, ElementState::Pressed) => {
                match game.game_state {
                    GameState::Playing => game.game_state = GameState::Paused,
                    GameState::Paused => game.game_state = GameState::Playing,
                    GameState::Menu => {} // Could exit game
                    GameState::GameOver { .. } => game.reset_game(),
                }
            }
            (VirtualKeyCode::Space, ElementState::Pressed) => match game.game_state {
                GameState::Menu => game.start_game(),
                GameState::GameOver { .. } => game.reset_game(),
                _ => {}
            },
            _ => {}
        }
    }
}

fn render_game(game: &SimplePongGame) {
    // Clear screen (simulate)
    println!("\x1B[2J\x1B[1;1H"); // Clear terminal

    match game.game_state {
        GameState::Menu => {
            println!("╔══════════════════════════════════════════════════════════════╗");
            println!("║                    PONG - Modular Engine                    ║");
            println!("║                                                              ║");
            println!("║                   Press SPACE to Start                      ║");
            println!("║                   Press ESC to Exit                         ║");
            println!("║                                                              ║");
            println!("║                   First to 5 points wins!                   ║");
            println!("╚══════════════════════════════════════════════════════════════╝");
        }
        GameState::Playing | GameState::Paused => {
            // Draw game field
            println!("┌────────────────────────────────────────────────────────────────┐");
            for row in 0..20 {
                print!("│");
                for col in 0..60 {
                    let screen_x = (col as f32 / 60.0) * WINDOW_WIDTH as f32;
                    let screen_y = (row as f32 / 20.0) * WINDOW_HEIGHT as f32;

                    let mut found_object = false;
                    for obj in game.get_game_objects() {
                        if screen_x >= obj.x
                            && screen_x < obj.x + obj.width
                            && screen_y >= obj.y
                            && screen_y < obj.y + obj.height
                        {
                            if obj.color[1] > 0.5 {
                                // Green (player paddle)
                                print!("█");
                            } else if obj.color[0] > 0.5 {
                                // Red (AI paddle)
                                print!("█");
                            } else {
                                // White (ball)
                                print!("●");
                            }
                            found_object = true;
                            break;
                        }
                    }

                    if !found_object {
                        // Draw center line
                        if col == 30 {
                            print!("│");
                        } else {
                            print!(" ");
                        }
                    }
                }
                println!("│");
            }
            println!("└────────────────────────────────────────────────────────────────┘");

            // Draw score
            println!(
                "                    Player: {} | AI: {}",
                game.score.0, game.score.1
            );

            if let GameState::Paused = game.game_state {
                println!("                    *** PAUSED ***");
                println!("                 Press ESC to Resume");
            }
        }
        GameState::GameOver { ref winner } => {
            println!("╔══════════════════════════════════════════════════════════════╗");
            println!("║                        GAME OVER                           ║");
            println!("║                                                              ║");
            println!(
                "║                   {} Wins!                            ║",
                winner
            );
            println!("║                                                              ║");
            println!(
                "║                 Final Score: {} - {}                      ║",
                game.score.0, game.score.1
            );
            println!("║                                                              ║");
            println!("║                 Press SPACE for Menu                       ║");
            println!("║                 Press ESC to Exit                          ║");
            println!("╚══════════════════════════════════════════════════════════════╝");
        }
    }
}

// Game-specific components are defined in the main components.rs file

// Game systems
use specs::{Entities, Join, Read, ReadStorage, System, Write, WriteStorage};

// Collision helper types
#[derive(Clone)]
enum CollisionType {
    Wall,
    Paddle(Position),
}

enum ScoreEvent {
    PlayerScore,
    AIScore,
}

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
                velocity.y = 0.0;
                if input_state
                    .keys_pressed
                    .contains(&winit::event::VirtualKeyCode::W)
                {
                    velocity.y = -PADDLE_SPEED;
                }
                if input_state
                    .keys_pressed
                    .contains(&winit::event::VirtualKeyCode::S)
                {
                    velocity.y = PADDLE_SPEED;
                }
            }
        }
    }
}

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
        let ball_pos = balls
            .join()
            .next()
            .and_then(|_| positions.join().next())
            .map(|pos| pos.as_vec2())
            .unwrap_or(Vec2::new(
                WINDOW_WIDTH as f32 / 2.0,
                WINDOW_HEIGHT as f32 / 2.0,
            ));

        for (position, velocity, paddle) in (&positions, &mut velocities, &paddles).join() {
            if !paddle.player_controlled {
                let paddle_center = position.y + PADDLE_HEIGHT / 2.0;
                let ball_center = ball_pos.y;
                let diff = ball_center - paddle_center;
                let ai_error = (time.elapsed * 2.0).sin() * 20.0;
                let target_diff = diff + ai_error;

                if target_diff.abs() > 10.0 {
                    velocity.y = target_diff.signum() * PADDLE_SPEED * 0.8;
                } else {
                    velocity.y = 0.0;
                }
            }
        }
    }
}

pub struct PongCollisionSystem;
impl<'a> System<'a> for PongCollisionSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
        ReadStorage<'a, Ball>,
        ReadStorage<'a, Paddle>,
        Write<'a, Score>,
    );

    fn run(
        &mut self,
        (entities, mut positions, mut velocities, balls, paddles, mut score): Self::SystemData,
    ) {
        // First pass: detect collisions and prepare responses
        let mut collision_responses = Vec::new();
        let mut scoring_events = Vec::new();

        for (ball_entity, ball_pos, _) in (&entities, &positions, &balls).join() {
            // Check wall collisions
            if ball_pos.y <= 0.0 || ball_pos.y >= WINDOW_HEIGHT as f32 - BALL_SIZE {
                collision_responses.push((ball_entity, CollisionType::Wall));
            }

            // Check paddle collisions
            for (paddle_entity, paddle_pos, _) in (&entities, &positions, &paddles).join() {
                if check_paddle_ball_collision(ball_pos, paddle_pos) {
                    collision_responses
                        .push((ball_entity, CollisionType::Paddle(paddle_pos.clone())));
                    break; // Only handle first collision
                }
            }

            // Check for scoring
            if ball_pos.x < -BALL_SIZE {
                scoring_events.push(ScoreEvent::AIScore);
            } else if ball_pos.x > WINDOW_WIDTH as f32 {
                scoring_events.push(ScoreEvent::PlayerScore);
            }
        }

        // Second pass: apply collision responses
        for (entity, collision_type) in collision_responses {
            match collision_type {
                CollisionType::Wall => {
                    if let Some(vel) = velocities.get_mut(entity) {
                        vel.y = -vel.y;
                    }
                }
                CollisionType::Paddle(paddle_pos) => {
                    if let Some(vel) = velocities.get_mut(entity) {
                        vel.x = -vel.x;

                        // Add spin based on hit position
                        let ball_pos = positions.get(entity).unwrap();
                        let paddle_center = paddle_pos.y + PADDLE_HEIGHT / 2.0;
                        let hit_pos = ball_pos.y + BALL_SIZE / 2.0;
                        let spin_factor = (hit_pos - paddle_center) / (PADDLE_HEIGHT / 2.0);
                        vel.y += spin_factor * 100.0;

                        // Ensure ball doesn't get too fast
                        let speed = (vel.x * vel.x + vel.y * vel.y).sqrt();
                        if speed > BALL_SPEED * 1.5 {
                            vel.x = vel.x / speed * BALL_SPEED * 1.2;
                            vel.y = vel.y / speed * BALL_SPEED * 1.2;
                        }
                    }
                }
            }
        }

        // Handle scoring
        for event in scoring_events {
            match event {
                ScoreEvent::PlayerScore => {
                    score.player_score += 1;
                }
                ScoreEvent::AIScore => {
                    score.ai_score += 1;
                }
            }
            reset_ball_positions(&mut positions, &mut velocities, &balls);
        }
    }
}

fn check_paddle_ball_collision(ball_pos: &Position, paddle_pos: &Position) -> bool {
    ball_pos.x < paddle_pos.x + PADDLE_WIDTH
        && ball_pos.x + BALL_SIZE > paddle_pos.x
        && ball_pos.y < paddle_pos.y + PADDLE_HEIGHT
        && ball_pos.y + BALL_SIZE > paddle_pos.y
}

fn reset_ball_positions(
    positions: &mut WriteStorage<Position>,
    velocities: &mut WriteStorage<Velocity>,
    balls: &ReadStorage<Ball>,
) {
    for (pos, vel, _) in (positions, velocities, balls).join() {
        pos.x = WINDOW_WIDTH as f32 / 2.0 - BALL_SIZE / 2.0;
        pos.y = WINDOW_HEIGHT as f32 / 2.0 - BALL_SIZE / 2.0;
        vel.x = BALL_SPEED * (if rand::random::<bool>() { 1.0 } else { -1.0 });
        vel.y = (rand::random::<f32>() - 0.5) * BALL_SPEED * 0.5;
    }
}

pub struct PongGameLogicSystem;
impl<'a> System<'a> for PongGameLogicSystem {
    type SystemData = (
        WriteStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
        ReadStorage<'a, Ball>,
        Read<'a, Time>,
    );

    fn run(&mut self, (mut positions, mut velocities, balls, time): Self::SystemData) {
        // Simple game logic - ball reset is handled in collision system
    }
}

// Helper functions
fn create_pong_entities(world: &mut World) {
    // Create player paddle (left side)
    world
        .create_entity_with_components()
        .with(Position::new(
            50.0,
            WINDOW_HEIGHT as f32 / 2.0 - PADDLE_HEIGHT / 2.0,
        ))
        .with(Velocity::new(0.0, 0.0))
        .with(Renderable::new("player_paddle".to_string()))
        .with(Paddle {
            player_controlled: true,
        })
        .with(Collider::new_rectangle(PADDLE_WIDTH, PADDLE_HEIGHT))
        .build();

    // Create AI paddle (right side)
    world
        .create_entity_with_components()
        .with(Position::new(
            WINDOW_WIDTH as f32 - 50.0 - PADDLE_WIDTH,
            WINDOW_HEIGHT as f32 / 2.0 - PADDLE_HEIGHT / 2.0,
        ))
        .with(Velocity::new(0.0, 0.0))
        .with(Renderable::new("ai_paddle".to_string()))
        .with(Paddle {
            player_controlled: false,
        })
        .with(Collider::new_rectangle(PADDLE_WIDTH, PADDLE_HEIGHT))
        .build();

    // Create ball (center)
    world
        .create_entity_with_components()
        .with(Position::new(
            WINDOW_WIDTH as f32 / 2.0 - BALL_SIZE / 2.0,
            WINDOW_HEIGHT as f32 / 2.0 - BALL_SIZE / 2.0,
        ))
        .with(Velocity::new(BALL_SPEED, BALL_SPEED * 0.5))
        .with(Renderable::new("ball".to_string()))
        .with(Ball)
        .with(Collider::new_circle(BALL_SIZE / 2.0))
        .build();

    // Create score entity
    world
        .create_entity_with_components()
        .with(Score::default())
        .build();
}

fn reset_ball(world: &mut World) {
    let mut positions = world.write_storage::<Position>();
    let mut velocities = world.write_storage::<Velocity>();
    let balls = world.read_storage::<Ball>();

    for (pos, vel, _) in (&mut positions, &mut velocities, &balls).join() {
        pos.x = WINDOW_WIDTH as f32 / 2.0 - BALL_SIZE / 2.0;
        pos.y = WINDOW_HEIGHT as f32 / 2.0 - BALL_SIZE / 2.0;
        vel.x = BALL_SPEED * (if rand::random::<bool>() { 1.0 } else { -1.0 });
        vel.y = (rand::random::<f32>() - 0.5) * BALL_SPEED * 0.5;
    }
}
