//! Window Pong Demo
//!
//! A graphical Pong game running in a real window with basic 2D graphics.
//! Uses minifb for simple window and graphics rendering.

use modular_game_engine::*;
use minifb::{Key, Window, WindowOptions};
use std::time::Instant;

// Game constants
const WINDOW_WIDTH: usize = 800;
const WINDOW_HEIGHT: usize = 600;
const PADDLE_WIDTH: f32 = 20.0;
const PADDLE_HEIGHT: f32 = 100.0;
const BALL_SIZE: f32 = 15.0;
const PADDLE_SPEED: f32 = 300.0;
const BALL_SPEED: f32 = 400.0;

// Colors (ARGB format)
const WHITE: u32 = 0xFFFFFFFF;
const BLACK: u32 = 0xFF000000;
const GREEN: u32 = 0xFF00FF00;
const RED: u32 = 0xFFFF0000;
const BLUE: u32 = 0xFF0000FF;

// Game state
#[derive(Debug, Clone, PartialEq)]
pub enum GameState {
    Menu,
    Playing,
    Paused,
    GameOver { winner: String },
}

pub struct WindowPongGame {
    world: World,
    dispatcher: specs::Dispatcher<'static, 'static>,
    game_state: GameState,
    last_update: Instant,
    score: (u32, u32), // (player, ai)
    window: Window,
    buffer: Vec<u32>,
}

impl WindowPongGame {
    fn new() -> Self {
        // Create window
        let mut window = Window::new(
            "Pong - Modular Game Engine",
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
            WindowOptions::default(),
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

        // Create pixel buffer
        let buffer: Vec<u32> = vec![BLACK; WINDOW_WIDTH * WINDOW_HEIGHT];

        // Initialize game world
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
            window,
            buffer,
        }
    }

    fn update(&mut self) {
        let now = Instant::now();
        let delta_time = now.duration_since(self.last_update).as_secs_f32();
        self.last_update = now;

        // Update time resource
        self.world.write_resource::<Time>().delta = delta_time;
        self.world.write_resource::<Time>().elapsed += delta_time;

        // Handle input
        self.handle_input();

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
                    self.game_state = GameState::GameOver { winner: "Player".to_string() };
                } else if self.score.1 >= 5 {
                    self.game_state = GameState::GameOver { winner: "AI".to_string() };
                }
            }
            _ => {}
        }

        // Render frame
        self.render();
    }

    fn handle_input(&mut self) {
        if self.window.is_key_down(Key::Escape) {
            match self.game_state {
                GameState::Playing => self.game_state = GameState::Paused,
                GameState::Paused => self.game_state = GameState::Playing,
                GameState::Menu => {} // Could exit game
                GameState::GameOver { .. } => self.reset_game(),
            }
        }

        if self.window.is_key_down(Key::Space) {
            match self.game_state {
                GameState::Menu => self.start_game(),
                GameState::GameOver { .. } => self.reset_game(),
                _ => {}
            }
        }
    }

    fn start_game(&mut self) {
        self.game_state = GameState::Playing;
        self.world.write_resource::<Score>().player_score = 0;
        self.world.write_resource::<Score>().ai_score = 0;
        reset_ball(&mut self.world);
    }

    fn reset_game(&mut self) {
        self.game_state = GameState::Menu;
        self.score = (0, 0);
        reset_ball(&mut self.world);
    }

    fn render(&mut self) {
        // Clear buffer
        self.buffer.fill(BLACK);

        match self.game_state {
            GameState::Menu => {
                self.draw_menu();
            }
            GameState::Playing | GameState::Paused => {
                self.draw_game();
                if let GameState::Paused = self.game_state {
                    self.draw_text("PAUSED", WINDOW_WIDTH / 2 - 100, WINDOW_HEIGHT / 2, WHITE, 2);
                    self.draw_text("Press ESC to Resume", WINDOW_WIDTH / 2 - 150, WINDOW_HEIGHT / 2 + 50, WHITE, 1);
                }
            }
            GameState::GameOver { ref winner } => {
                let winner_text = format!("{} Wins!", winner);
                let score_text = format!("Final Score: {} - {}", self.score.0, self.score.1);
                self.draw_game();
                self.draw_text("GAME OVER", WINDOW_WIDTH / 2 - 120, WINDOW_HEIGHT / 2 - 50, RED, 2);
                self.draw_text(&winner_text, WINDOW_WIDTH / 2 - 100, WINDOW_HEIGHT / 2, WHITE, 2);
                self.draw_text("Press SPACE for Menu", WINDOW_WIDTH / 2 - 140, WINDOW_HEIGHT / 2 + 50, WHITE, 1);
                self.draw_text(&score_text, WINDOW_WIDTH / 2 - 120, WINDOW_HEIGHT / 2 + 100, BLUE, 1);
            }
        }

        // Update window with buffer
        self.window.update_with_buffer(&self.buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap();
    }

    fn draw_menu(&mut self) {
        // Draw title
        self.draw_text("PONG", WINDOW_WIDTH / 2 - 80, WINDOW_HEIGHT / 2 - 100, WHITE, 3);
        self.draw_text("Modular Game Engine", WINDOW_WIDTH / 2 - 150, WINDOW_HEIGHT / 2 - 60, BLUE, 2);

        // Draw instructions
        self.draw_text("Press SPACE to Start", WINDOW_WIDTH / 2 - 130, WINDOW_HEIGHT / 2, GREEN, 1);
        self.draw_text("W/S: Move Paddle", WINDOW_WIDTH / 2 - 100, WINDOW_HEIGHT / 2 + 40, WHITE, 1);
        self.draw_text("ESC: Pause/Menu", WINDOW_WIDTH / 2 - 110, WINDOW_HEIGHT / 2 + 70, WHITE, 1);
        self.draw_text("First to 5 points wins!", WINDOW_WIDTH / 2 - 140, WINDOW_HEIGHT / 2 + 120, RED, 1);
    }

    fn draw_game(&mut self) {
        // Get game object data first (avoid borrowing conflicts)
        let paddle_data: Vec<(f32, f32, bool)> = {
            let positions = self.world.read_storage::<Position>();
            let paddles = self.world.read_storage::<Paddle>();
            (&positions, &paddles).join()
                .map(|(pos, paddle)| (pos.x, pos.y, paddle.player_controlled))
                .collect()
        };

        let ball_data: Vec<(f32, f32)> = {
            let positions = self.world.read_storage::<Position>();
            let balls = self.world.read_storage::<Ball>();
            (&positions, &balls).join()
                .map(|(pos, _)| (pos.x, pos.y))
                .collect()
        };

        // Draw paddles
        for (x, y, is_player) in paddle_data {
            let color = if is_player { GREEN } else { RED };
            self.draw_rect(x as i32, y as i32, PADDLE_WIDTH as i32, PADDLE_HEIGHT as i32, color);
        }

        // Draw ball
        for (x, y) in ball_data {
            self.draw_rect(x as i32, y as i32, BALL_SIZE as i32, BALL_SIZE as i32, WHITE);
        }

        // Draw center line
        for i in 0..20 {
            let y = i * 30;
            self.draw_rect(WINDOW_WIDTH as i32 / 2 - 2, y, 4, 15, WHITE);
        }

        // Draw score
        self.draw_score();
    }

    fn draw_score(&mut self) {
        // Draw player score (left side)
        self.draw_text(&self.score.0.to_string(), 50, 50, GREEN, 2);

        // Draw AI score (right side)
        let ai_score_text = self.score.1.to_string();
        self.draw_text(&ai_score_text, WINDOW_WIDTH - 80, 50, RED, 2);
    }

    fn draw_rect(&mut self, x: i32, y: i32, width: i32, height: i32, color: u32) {
        for dy in 0..height {
            for dx in 0..width {
                let px = x + dx;
                let py = y + dy;
                if px >= 0 && px < WINDOW_WIDTH as i32 && py >= 0 && py < WINDOW_HEIGHT as i32 {
                    let index = (py as usize) * WINDOW_WIDTH + (px as usize);
                    if index < self.buffer.len() {
                        self.buffer[index] = color;
                    }
                }
            }
        }
    }

    fn draw_text(&mut self, text: &str, x: usize, y: usize, color: u32, scale: usize) {
        let mut current_x = x;
        for ch in text.chars() {
            if ch != ' ' {
                self.draw_char(ch, current_x, y, color, scale);
            }
            current_x += 8 * scale;
        }
    }

    fn draw_char(&mut self, ch: char, x: usize, y: usize, color: u32, scale: usize) {
        // Simple 5x7 font for basic characters
        let font_data = match ch {
            '0' => [[true, true, true, true, true],
                    [true, false, false, false, true],
                    [true, false, false, false, true],
                    [true, false, false, false, true],
                    [true, false, false, false, true],
                    [true, false, false, false, true],
                    [true, true, true, true, true]],
            '1' => [[false, false, true, false, false],
                    [false, true, true, false, false],
                    [false, false, true, false, false],
                    [false, false, true, false, false],
                    [false, false, true, false, false],
                    [false, false, true, false, false],
                    [true, true, true, true, true]],
            '2' => [[true, true, true, true, true],
                    [false, false, false, false, true],
                    [false, false, false, false, true],
                    [true, true, true, true, true],
                    [true, false, false, false, false],
                    [true, false, false, false, false],
                    [true, true, true, true, true]],
            '3' => [[true, true, true, true, true],
                    [false, false, false, false, true],
                    [false, false, false, false, true],
                    [true, true, true, true, true],
                    [false, false, false, false, true],
                    [false, false, false, false, true],
                    [true, true, true, true, true]],
            '4' => [[true, false, false, false, true],
                    [true, false, false, false, true],
                    [true, false, false, false, true],
                    [true, true, true, true, true],
                    [false, false, false, false, true],
                    [false, false, false, false, true],
                    [false, false, false, false, true]],
            '5' => [[true, true, true, true, true],
                    [true, false, false, false, false],
                    [true, false, false, false, false],
                    [true, true, true, true, true],
                    [false, false, false, false, true],
                    [false, false, false, false, true],
                    [true, true, true, true, true]],
            '6' => [[true, true, true, true, true],
                    [true, false, false, false, false],
                    [true, false, false, false, false],
                    [true, true, true, true, true],
                    [true, false, false, false, true],
                    [true, false, false, false, true],
                    [true, true, true, true, true]],
            '7' => [[true, true, true, true, true],
                    [false, false, false, false, true],
                    [false, false, false, false, true],
                    [false, false, false, true, false],
                    [false, false, true, false, false],
                    [false, true, false, false, false],
                    [true, false, false, false, false]],
            '8' => [[true, true, true, true, true],
                    [true, false, false, false, true],
                    [true, false, false, false, true],
                    [true, true, true, true, true],
                    [true, false, false, false, true],
                    [true, false, false, false, true],
                    [true, true, true, true, true]],
            '9' => [[true, true, true, true, true],
                    [true, false, false, false, true],
                    [true, false, false, false, true],
                    [true, true, true, true, true],
                    [false, false, false, false, true],
                    [false, false, false, false, true],
                    [true, true, true, true, true]],
            'A'..='Z' => match ch {
                'A' => [[false, true, true, true, false],
                        [true, false, false, false, true],
                        [true, false, false, false, true],
                        [true, true, true, true, true],
                        [true, false, false, false, true],
                        [true, false, false, false, true],
                        [true, false, false, false, true]],
                'G' => [[true, true, true, true, true],
                        [true, false, false, false, false],
                        [true, false, false, false, false],
                        [true, false, true, true, true],
                        [true, false, false, false, true],
                        [true, false, false, false, true],
                        [true, true, true, true, true]],
                'M' => [[true, false, false, false, true],
                        [true, true, false, true, true],
                        [true, false, true, false, true],
                        [true, false, true, false, true],
                        [true, false, false, false, true],
                        [true, false, false, false, true],
                        [true, false, false, false, true]],
                'O' => [[true, true, true, true, true],
                        [true, false, false, false, true],
                        [true, false, false, false, true],
                        [true, false, false, false, true],
                        [true, false, false, false, true],
                        [true, false, false, false, true],
                        [true, true, true, true, true]],
                'P' => [[true, true, true, true, true],
                        [true, false, false, false, true],
                        [true, false, false, false, true],
                        [true, true, true, true, true],
                        [true, false, false, false, false],
                        [true, false, false, false, false],
                        [true, false, false, false, false]],
                'W' => [[true, false, false, false, true],
                        [true, false, false, false, true],
                        [true, false, false, false, true],
                        [true, false, true, false, true],
                        [true, false, true, false, true],
                        [true, true, false, true, true],
                        [true, false, false, false, true]],
                _ => [[true, true, true, true, true],
                      [true, false, false, false, true],
                      [true, false, false, false, true],
                      [true, false, false, false, true],
                      [true, false, false, false, true],
                      [true, false, false, false, true],
                      [true, true, true, true, true]], // Default box
            },
            _ => [[false, false, false, false, false],
                  [false, false, false, false, false],
                  [false, false, false, false, false],
                  [false, false, false, false, false],
                  [false, false, false, false, false],
                  [false, false, false, false, false],
                  [false, false, false, false, false]], // Empty
        };

        for (row, pixels) in font_data.iter().enumerate() {
            for (col, pixel) in pixels.iter().enumerate() {
                if *pixel {
                    let px = x + col * scale;
                    let py = y + row * scale;
                    self.draw_rect(px as i32, py as i32, scale as i32, scale as i32, color);
                }
            }
        }
    }

    fn is_running(&self) -> bool {
        self.window.is_open() && !self.window.is_key_down(Key::Q)
    }
}

// Game systems (reuse from simple graphical pong)
use specs::{System, ReadStorage, WriteStorage, Read, Write, Entities, Join};

pub struct PongInputSystem;
impl<'a> System<'a> for PongInputSystem {
    type SystemData = (
        WriteStorage<'a, Velocity>,
        ReadStorage<'a, Paddle>,
    );

    fn run(&mut self, (mut velocities, paddles): Self::SystemData) {
        for (velocity, paddle) in (&mut velocities, &paddles).join() {
            if paddle.player_controlled {
                velocity.y = 0.0;
                // Input is handled in the main game loop
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
        let ball_pos = balls.join()
            .next()
            .and_then(|_| positions.join().next())
            .map(|pos| pos.as_vec2())
            .unwrap_or(Vec2::new(WINDOW_WIDTH as f32 / 2.0, WINDOW_HEIGHT as f32 / 2.0));

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

    fn run(&mut self, (entities, mut positions, mut velocities, balls, paddles, mut score): Self::SystemData) {
        // Get collision data first to avoid borrowing conflicts
        let ball_positions: Vec<(specs::Entity, Position)> = (&entities, &positions, &balls).join()
            .map(|(entity, pos, _)| (entity, pos.clone()))
            .collect();

        let paddle_positions: Vec<(specs::Entity, Position)> = (&entities, &positions, &paddles).join()
            .map(|(entity, pos, _)| (entity, pos.clone()))
            .collect();

        // Process collisions
        for (ball_entity, ball_pos) in &ball_positions {
            // Check wall collisions
            if ball_pos.y <= 0.0 || ball_pos.y >= WINDOW_HEIGHT as f32 - BALL_SIZE {
                if let Some(vel) = velocities.get_mut(*ball_entity) {
                    vel.y = -vel.y;
                }
            }

            // Check paddle collisions
            for (paddle_entity, paddle_pos) in &paddle_positions {
                if check_paddle_ball_collision(ball_pos, paddle_pos) {
                    if let Some(vel) = velocities.get_mut(*ball_entity) {
                        vel.x = -vel.x;

                        // Add spin based on hit position
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
                    break; // Only handle first collision
                }
            }

            // Check for scoring
            if ball_pos.x < -BALL_SIZE {
                score.ai_score += 1;
                reset_ball_positions(&mut positions, &mut velocities, &balls);
            } else if ball_pos.x > WINDOW_WIDTH as f32 {
                score.player_score += 1;
                reset_ball_positions(&mut positions, &mut velocities, &balls);
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

fn reset_ball_positions(positions: &mut WriteStorage<Position>, velocities: &mut WriteStorage<Velocity>, balls: &ReadStorage<Ball>) {
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
    world.create_entity_with_components()
        .with(Position::new(50.0, WINDOW_HEIGHT as f32 / 2.0 - PADDLE_HEIGHT / 2.0))
        .with(Velocity::new(0.0, 0.0))
        .with(Renderable::new("player_paddle".to_string()))
        .with(Paddle { player_controlled: true })
        .with(Collider::new_rectangle(PADDLE_WIDTH, PADDLE_HEIGHT))
        .build();

    // Create AI paddle (right side)
    world.create_entity_with_components()
        .with(Position::new(WINDOW_WIDTH as f32 - 50.0 - PADDLE_WIDTH, WINDOW_HEIGHT as f32 / 2.0 - PADDLE_HEIGHT / 2.0))
        .with(Velocity::new(0.0, 0.0))
        .with(Renderable::new("ai_paddle".to_string()))
        .with(Paddle { player_controlled: false })
        .with(Collider::new_rectangle(PADDLE_WIDTH, PADDLE_HEIGHT))
        .build();

    // Create ball (center)
    world.create_entity_with_components()
        .with(Position::new(WINDOW_WIDTH as f32 / 2.0 - BALL_SIZE / 2.0, WINDOW_HEIGHT as f32 / 2.0 - BALL_SIZE / 2.0))
        .with(Velocity::new(BALL_SPEED, BALL_SPEED * 0.5))
        .with(Renderable::new("ball".to_string()))
        .with(Ball)
        .with(Collider::new_circle(BALL_SIZE / 2.0))
        .build();

    // Create score entity
    world.create_entity_with_components()
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

fn main() {
    let mut pong_game = WindowPongGame::new();

    println!("🎮 Window Pong Demo");
    println!("===================");
    println!("Controls:");
    println!("  W/S: Move paddle");
    println!("  SPACE: Start game / Return to menu");
    println!("  ESC: Pause / Resume");
    println!("  Q: Quit");
    println!();
    println!("Window should open now...");

    // Main game loop
    while pong_game.is_running() {
        pong_game.update();

        // Small delay to prevent excessive CPU usage
        std::thread::sleep(std::time::Duration::from_millis(16));
    }

    println!("Game closed. Thanks for playing!");
}