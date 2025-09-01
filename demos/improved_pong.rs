//! Improved Pong Demo
//!
//! An enhanced version of Pong showcasing all the extracted systems:
//! - Professional 2D graphics with the renderer_2d system
//! - FreeType font integration for high-quality text rendering
//! - Enhanced input handling with action mapping
//! - Game state management with menus and transitions
//! - Particle effects and visual enhancements
//! - Improved AI and gameplay mechanics
//!
//! Font Loading: To use custom TTF fonts, call renderer.load_font() in your game setup.
//! Example:
//! ```rust
//! // Load a custom font (add TTF files to assets/fonts/)
//! renderer.load_font("game_font", "assets/fonts/DejaVuSans.ttf");
//! renderer.set_default_font("game_font");
//! ```
//!
//! The game will automatically use high-quality fonts for all text rendering.
//! If no fonts are loaded, it falls back to the built-in bitmap font.

use modular_game_engine::*;

// Game constants
const WINDOW_WIDTH: usize = 800;
const WINDOW_HEIGHT: usize = 600;
const PADDLE_WIDTH: f32 = 20.0;
const PADDLE_HEIGHT: f32 = 100.0;
const BALL_SIZE: f32 = 15.0;
const PADDLE_SPEED: f32 = 350.0;
const BALL_SPEED: f32 = 450.0;
const MAX_SCORE: u32 = 5;

// Particle system for visual effects
struct Particle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    life: f32,
    max_life: f32,
    color: renderer_2d::Color,
    size: f32,
}

struct ParticleSystem {
    particles: Vec<Particle>,
}

impl ParticleSystem {
    fn new() -> Self {
        Self {
            particles: Vec::new(),
        }
    }

    fn emit(&mut self, x: f32, y: f32, count: usize, color: renderer_2d::Color) {
        for _ in 0..count {
            let angle = (rand::random::<f32>() - 0.5) * std::f32::consts::PI * 2.0;
            let speed = rand::random::<f32>() * 200.0 + 50.0;
            let life = rand::random::<f32>() * 0.5 + 0.5;

            self.particles.push(Particle {
                x,
                y,
                vx: angle.cos() * speed,
                vy: angle.sin() * speed,
                life,
                max_life: life,
                color,
                size: rand::random::<f32>() * 3.0 + 1.0,
            });
        }
    }

    fn update(&mut self, delta_time: f32) {
        self.particles.retain_mut(|particle| {
            particle.x += particle.vx * delta_time;
            particle.y += particle.vy * delta_time;
            particle.vy += 300.0 * delta_time; // Gravity
            particle.life -= delta_time;
            particle.life > 0.0
        });
    }

    fn render(&self, renderer: &mut renderer_2d::Renderer2D) {
        for particle in &self.particles {
            let alpha = particle.life / particle.max_life;
            let size = particle.size * alpha;

            // Create color with alpha
            let r = ((particle.color.0 >> 16) & 0xFF) as f32 * alpha;
            let g = ((particle.color.0 >> 8) & 0xFF) as f32 * alpha;
            let b = (particle.color.0 & 0xFF) as f32 * alpha;

            let faded_color = renderer_2d::Color::rgb(r as u8, g as u8, b as u8);

            renderer.draw_circle_filled(
                particle.x as i32,
                particle.y as i32,
                size as i32,
                faded_color,
            );
        }
    }
}

// Enhanced Pong game state
struct ImprovedPongGame {
    world: World,
    dispatcher: specs::Dispatcher<'static, 'static>,
    game_state: GameState,
    last_update: std::time::Instant,
    score: (u32, u32),
    particle_system: ParticleSystem,
    ball_trail: Vec<(f32, f32, f32)>, // (x, y, alpha)
    game_time: f32,
    difficulty: Difficulty,
}

#[derive(Clone, Copy, PartialEq)]
enum Difficulty {
    Easy,
    Normal,
    Hard,
}

impl Difficulty {
    fn ai_speed_multiplier(&self) -> f32 {
        match self {
            Difficulty::Easy => 0.6,
            Difficulty::Normal => 0.8,
            Difficulty::Hard => 1.0,
        }
    }

    fn ball_speed_multiplier(&self) -> f32 {
        match self {
            Difficulty::Easy => 0.8,
            Difficulty::Normal => 1.0,
            Difficulty::Hard => 1.2,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum GameState {
    Menu,
    DifficultySelect,
    Playing,
    Paused,
    Scored { points: u32, is_player: bool },
    GameOver { winner: String },
}

impl ImprovedPongGame {
    fn new() -> Self {
        let mut world = init().unwrap();

        // Register game-specific components
        world.register::<Paddle>();
        world.register::<Ball>();
        world.register::<Score>();

        // Create game entities
        create_pong_entities(&mut world);

        // Insert input state resource
        world.insert(crate::input_window::WindowInputState::default());

        // Set up systems
        let dispatcher = specs::DispatcherBuilder::new()
            .with(ImprovedPongInputSystem, "input", &[])
            .with(ImprovedPongAISystem, "ai", &["input"])
            .with(PhysicsSystem, "physics", &["ai"])
            .with(ImprovedPongCollisionSystem, "collision", &["physics"])
            .with(ImprovedPongGameLogicSystem, "game_logic", &["collision"])
            .build();

        Self {
            world,
            dispatcher,
            game_state: GameState::Menu,
            last_update: std::time::Instant::now(),
            score: (0, 0),
            particle_system: ParticleSystem::new(),
            ball_trail: Vec::new(),
            game_time: 0.0,
            difficulty: Difficulty::Normal,
        }
    }

    fn update(&mut self, delta_time: f32, input: &input_window::WindowInputState) {
        self.game_time += delta_time;

        // Update time resource
        self.world.write_resource::<Time>().delta = delta_time;
        self.world.write_resource::<Time>().elapsed = self.game_time;

        // Update input state resource
        *self
            .world
            .write_resource::<crate::input_window::WindowInputState>() = input.clone();

        // Update particle system
        self.particle_system.update(delta_time);

        // Update ball trail
        self.update_ball_trail(delta_time);

        match &self.game_state {
            GameState::Menu => {
                if input.is_key_just_pressed(minifb::Key::Space) {
                    self.game_state = GameState::DifficultySelect;
                }
            }
            GameState::DifficultySelect => {
                // Navigation keys (change selection without starting)
                if input.is_key_just_pressed(minifb::Key::Down) {
                    self.difficulty = match self.difficulty {
                        Difficulty::Easy => Difficulty::Normal,
                        Difficulty::Normal => Difficulty::Hard,
                        Difficulty::Hard => Difficulty::Easy,
                    };
                } else if input.is_key_just_pressed(minifb::Key::Up) {
                    self.difficulty = match self.difficulty {
                        Difficulty::Easy => Difficulty::Hard,
                        Difficulty::Normal => Difficulty::Easy,
                        Difficulty::Hard => Difficulty::Normal,
                    };
                }

                // Direct selection keys (immediate start)
                if input.is_key_just_pressed(minifb::Key::Key1)
                    || input.is_key_just_pressed(minifb::Key::NumPad1)
                {
                    self.difficulty = Difficulty::Easy;
                    self.start_game();
                } else if input.is_key_just_pressed(minifb::Key::Key2)
                    || input.is_key_just_pressed(minifb::Key::NumPad2)
                {
                    self.difficulty = Difficulty::Normal;
                    self.start_game();
                } else if input.is_key_just_pressed(minifb::Key::Key3)
                    || input.is_key_just_pressed(minifb::Key::NumPad3)
                {
                    self.difficulty = Difficulty::Hard;
                    self.start_game();
                }
                // Confirmation keys (start with current selection)
                else if input.is_key_just_pressed(minifb::Key::Enter)
                    || input.is_key_just_pressed(minifb::Key::Space)
                {
                    self.start_game();
                } else if input.is_key_just_pressed(minifb::Key::Escape) {
                    self.game_state = GameState::Menu;
                }
            }
            GameState::Playing => {
                // Run game systems
                self.dispatcher.dispatch(&mut self.world);
                self.world.maintain();

                // Update score from world
                let score_resource = self.world.read_resource::<Score>();
                self.score = (score_resource.player_score, score_resource.ai_score);

                // Check for game end
                if self.score.0 >= MAX_SCORE {
                    self.game_state = GameState::GameOver {
                        winner: "Player".to_string(),
                    };
                } else if self.score.1 >= MAX_SCORE {
                    self.game_state = GameState::GameOver {
                        winner: "AI".to_string(),
                    };
                }

                // Handle pause
                if input.is_key_just_pressed(minifb::Key::Escape) {
                    self.game_state = GameState::Paused;
                }
            }
            GameState::Paused => {
                if input.is_key_just_pressed(minifb::Key::Escape) {
                    self.game_state = GameState::Playing;
                } else if input.is_key_just_pressed(minifb::Key::Q) {
                    self.game_state = GameState::Menu;
                    self.reset_game();
                }
            }
            GameState::Scored { .. } => {
                // Wait a moment before continuing
                if self.game_time % 1.5 < delta_time {
                    self.game_state = GameState::Playing;
                }
            }
            GameState::GameOver { .. } => {
                if input.is_key_just_pressed(minifb::Key::Space) {
                    self.reset_game();
                    self.game_state = GameState::Menu;
                }
            }
        }
    }

    fn start_game(&mut self) {
        self.game_state = GameState::Playing;
        self.score = (0, 0);
        self.game_time = 0.0;

        // Reset score in world
        {
            let mut score_resource = self.world.write_resource::<Score>();
            score_resource.player_score = 0;
            score_resource.ai_score = 0;
        }

        // Reset ball
        reset_ball(&mut self.world, self.difficulty.ball_speed_multiplier());

        // Clear particles and trail
        self.particle_system.particles.clear();
        self.ball_trail.clear();
    }

    fn reset_game(&mut self) {
        self.score = (0, 0);
        self.game_time = 0.0;
        reset_ball(&mut self.world, 1.0);
        self.particle_system.particles.clear();
        self.ball_trail.clear();
    }

    fn update_ball_trail(&mut self, delta_time: f32) {
        // Add current ball position to trail
        let ball_pos = self
            .world
            .read_storage::<Position>()
            .join()
            .find(|_| true) // Get first ball
            .map(|pos| (pos.x, pos.y));

        if let Some((x, y)) = ball_pos {
            self.ball_trail.push((x, y, 1.0));

            // Limit trail length
            if self.ball_trail.len() > 20 {
                self.ball_trail.remove(0);
            }
        }

        // Update trail alpha
        for (_, _, alpha) in &mut self.ball_trail {
            *alpha -= delta_time * 2.0;
            if *alpha < 0.0 {
                *alpha = 0.0;
            }
        }

        // Remove faded trail points
        self.ball_trail.retain(|(_, _, alpha)| *alpha > 0.0);
    }

    fn render(&self, renderer: &mut renderer_2d::Renderer2D) {
        // Clear screen with dark background
        renderer.clear(renderer_2d::Color::rgb(20, 20, 30));

        match self.game_state {
            GameState::Menu => {
                self.render_menu(renderer);
            }
            GameState::DifficultySelect => {
                self.render_difficulty_select(renderer);
            }
            GameState::Playing | GameState::Paused => {
                self.render_gameplay(renderer);

                if let GameState::Paused = self.game_state {
                    self.render_pause_overlay(renderer);
                }
            }
            GameState::Scored { points, is_player } => {
                self.render_gameplay(renderer);
                self.render_score_effect(renderer, points, is_player);
            }
            GameState::GameOver { ref winner } => {
                self.render_gameplay(renderer);
                self.render_game_over(renderer, winner);
            }
        }

        // Render particles on top of everything
        self.particle_system.render(renderer);
    }

    fn render_menu(&self, renderer: &mut renderer_2d::Renderer2D) {
        // Title
        renderer.draw_text_centered(
            "IMPROVED PONG",
            WINDOW_WIDTH / 2,
            150,
            renderer_2d::Color::WHITE,
            3,
        );

        // Subtitle
        renderer.draw_text_centered(
            "Enhanced with Modular Game Engine",
            WINDOW_WIDTH / 2,
            200,
            renderer_2d::Color::rgb(150, 150, 200),
            1,
        );

        // Instructions
        renderer.draw_text_centered(
            "Press SPACE to Start",
            WINDOW_WIDTH / 2,
            300,
            renderer_2d::Color::GREEN,
            2,
        );
        renderer.draw_text_centered(
            "W/S: Move Paddle",
            WINDOW_WIDTH / 2,
            350,
            renderer_2d::Color::WHITE,
            1,
        );
        renderer.draw_text_centered(
            "ESC: Pause",
            WINDOW_WIDTH / 2,
            380,
            renderer_2d::Color::WHITE,
            1,
        );
        renderer.draw_text_centered(
            "Q: Quit to Menu",
            WINDOW_WIDTH / 2,
            410,
            renderer_2d::Color::WHITE,
            1,
        );

        // Version info
        renderer.draw_text_centered(
            "v2.0 - Enhanced Edition",
            WINDOW_WIDTH / 2,
            500,
            renderer_2d::Color::rgb(100, 100, 100),
            1,
        );
    }

    fn render_difficulty_select(&self, renderer: &mut renderer_2d::Renderer2D) {
        renderer.draw_text_centered(
            "SELECT DIFFICULTY",
            WINDOW_WIDTH / 2,
            150,
            renderer_2d::Color::WHITE,
            2,
        );

        let easy_color = if self.difficulty == Difficulty::Easy {
            renderer_2d::Color::GREEN
        } else {
            renderer_2d::Color::WHITE
        };
        let normal_color = if self.difficulty == Difficulty::Normal {
            renderer_2d::Color::GREEN
        } else {
            renderer_2d::Color::WHITE
        };
        let hard_color = if self.difficulty == Difficulty::Hard {
            renderer_2d::Color::GREEN
        } else {
            renderer_2d::Color::WHITE
        };

        renderer.draw_text_centered("1. EASY", WINDOW_WIDTH / 2, 250, easy_color, 2);
        renderer.draw_text_centered("2. NORMAL", WINDOW_WIDTH / 2, 300, normal_color, 2);
        renderer.draw_text_centered("3. HARD", WINDOW_WIDTH / 2, 350, hard_color, 2);

        renderer.draw_text_centered(
            "Use UP/DOWN arrows to navigate",
            WINDOW_WIDTH / 2,
            420,
            renderer_2d::Color::rgb(200, 200, 200),
            1,
        );
        renderer.draw_text_centered(
            "Press ENTER or SPACE to start",
            WINDOW_WIDTH / 2,
            450,
            renderer_2d::Color::rgb(150, 150, 150),
            1,
        );
        renderer.draw_text_centered(
            "Or press 1, 2, 3 for quick select",
            WINDOW_WIDTH / 2,
            480,
            renderer_2d::Color::rgb(150, 150, 150),
            1,
        );
        renderer.draw_text_centered(
            "ESC to go back",
            WINDOW_WIDTH / 2,
            510,
            renderer_2d::Color::rgb(150, 150, 150),
            1,
        );
    }

    fn render_gameplay(&self, renderer: &mut renderer_2d::Renderer2D) {
        // Draw ball trail
        for (i, (x, y, alpha)) in self.ball_trail.iter().enumerate() {
            let trail_color = renderer_2d::Color::rgba(
                (255.0 * alpha) as u8,
                (255.0 * alpha) as u8,
                (100.0 * alpha) as u8,
                (alpha * 255.0) as u8,
            );
            renderer.draw_circle_filled(
                *x as i32,
                *y as i32,
                (BALL_SIZE * alpha * 0.5) as i32,
                trail_color,
            );
        }

        // Draw game objects
        let positions = self.world.read_storage::<Position>();
        let renderables = self.world.read_storage::<Renderable>();
        let paddles = self.world.read_storage::<Paddle>();
        let balls = self.world.read_storage::<Ball>();

        // Draw paddles with glow effect
        for (pos, _, paddle) in (&positions, &renderables, &paddles).join() {
            let base_color = if paddle.player_controlled {
                renderer_2d::Color::rgb(0, 150, 0) // Green for player
            } else {
                renderer_2d::Color::rgb(150, 0, 0) // Red for AI
            };

            // Glow effect
            renderer.draw_rect(
                pos.x as i32 - 3,
                pos.y as i32 - 3,
                (PADDLE_WIDTH + 6.0) as i32,
                (PADDLE_HEIGHT + 6.0) as i32,
                renderer_2d::Color::rgba(255, 255, 255, 50),
            );

            // Main paddle
            renderer.draw_rect(
                pos.x as i32,
                pos.y as i32,
                PADDLE_WIDTH as i32,
                PADDLE_HEIGHT as i32,
                base_color,
            );
        }

        // Draw ball with glow
        for (pos, _, _) in (&positions, &renderables, &balls).join() {
            // Glow effect
            renderer.draw_circle_filled(
                pos.x as i32,
                pos.y as i32,
                (BALL_SIZE * 1.5) as i32,
                renderer_2d::Color::rgba(255, 255, 100, 100),
            );

            // Main ball
            renderer.draw_circle_filled(
                pos.x as i32,
                pos.y as i32,
                BALL_SIZE as i32,
                renderer_2d::Color::WHITE,
            );
        }

        // Draw center line with animated effect
        let line_offset = (self.game_time * 2.0).sin() * 5.0;
        for i in 0..15 {
            let y = i * 40 + line_offset as i32;
            renderer.draw_rect(
                WINDOW_WIDTH as i32 / 2 - 2,
                y,
                4,
                20,
                renderer_2d::Color::rgba(150, 150, 150, 200),
            );
        }

        // Draw score with better styling
        self.render_score(renderer);
    }

    fn render_score(&self, renderer: &mut renderer_2d::Renderer2D) {
        // Player score (left side)
        renderer.draw_text(
            &self.score.0.to_string(),
            100,
            50,
            renderer_2d::Color::rgb(0, 200, 0),
            3,
        );

        // AI score (right side)
        let ai_score_text = self.score.1.to_string();
        renderer.draw_text(
            &ai_score_text,
            WINDOW_WIDTH - 150,
            50,
            renderer_2d::Color::rgb(200, 0, 0),
            3,
        );

        // Difficulty indicator
        let diff_text = match self.difficulty {
            Difficulty::Easy => "EASY",
            Difficulty::Normal => "NORMAL",
            Difficulty::Hard => "HARD",
        };
        renderer.draw_text(
            diff_text,
            WINDOW_WIDTH / 2 - 50,
            30,
            renderer_2d::Color::rgb(150, 150, 200),
            1,
        );
    }

    fn render_pause_overlay(&self, renderer: &mut renderer_2d::Renderer2D) {
        // Semi-transparent overlay
        renderer.draw_rect(
            0,
            0,
            WINDOW_WIDTH as i32,
            WINDOW_HEIGHT as i32,
            renderer_2d::Color::rgba(0, 0, 0, 150),
        );

        renderer.draw_text_centered(
            "PAUSED",
            WINDOW_WIDTH / 2,
            WINDOW_HEIGHT / 2 - 50,
            renderer_2d::Color::WHITE,
            3,
        );
        renderer.draw_text_centered(
            "Press ESC to Resume",
            WINDOW_WIDTH / 2,
            WINDOW_HEIGHT / 2,
            renderer_2d::Color::rgb(200, 200, 200),
            1,
        );
        renderer.draw_text_centered(
            "Press Q to Quit to Menu",
            WINDOW_WIDTH / 2,
            WINDOW_HEIGHT / 2 + 40,
            renderer_2d::Color::rgb(200, 200, 200),
            1,
        );
    }

    fn render_score_effect(
        &self,
        renderer: &mut renderer_2d::Renderer2D,
        points: u32,
        is_player: bool,
    ) {
        let color = if is_player {
            renderer_2d::Color::GREEN
        } else {
            renderer_2d::Color::RED
        };
        let text = if is_player {
            "PLAYER SCORES!"
        } else {
            "AI SCORES!"
        };

        renderer.draw_text_centered(text, WINDOW_WIDTH / 2, WINDOW_HEIGHT / 2, color, 2);
    }

    fn render_game_over(&self, renderer: &mut renderer_2d::Renderer2D, winner: &str) {
        // Semi-transparent overlay
        renderer.draw_rect(
            0,
            0,
            WINDOW_WIDTH as i32,
            WINDOW_HEIGHT as i32,
            renderer_2d::Color::rgba(0, 0, 0, 200),
        );

        renderer.draw_text_centered(
            "GAME OVER",
            WINDOW_WIDTH / 2,
            WINDOW_HEIGHT / 2 - 100,
            renderer_2d::Color::WHITE,
            3,
        );

        let winner_color = if winner == "Player" {
            renderer_2d::Color::GREEN
        } else {
            renderer_2d::Color::RED
        };

        let winner_text = format!("{} Wins!", winner);
        renderer.draw_text_centered(
            &winner_text,
            WINDOW_WIDTH / 2,
            WINDOW_HEIGHT / 2 - 20,
            winner_color,
            2,
        );

        let score_text = format!("Final Score: {} - {}", self.score.0, self.score.1);
        renderer.draw_text_centered(
            &score_text,
            WINDOW_WIDTH / 2,
            WINDOW_HEIGHT / 2 + 40,
            renderer_2d::Color::rgb(200, 200, 200),
            1,
        );

        renderer.draw_text_centered(
            "Press SPACE for Main Menu",
            WINDOW_WIDTH / 2,
            WINDOW_HEIGHT / 2 + 100,
            renderer_2d::Color::rgb(150, 150, 200),
            1,
        );
    }
}

fn main() {
    println!("🎮 Improved Pong Demo");
    println!("=====================");
    println!("Enhanced features:");
    println!("- Professional 2D graphics with glow effects");
    println!("- Particle system for visual feedback");
    println!("- Ball trail effects");
    println!("- Multiple difficulty levels");
    println!("- Improved AI with difficulty scaling");
    println!("- Enhanced UI and menus");
    println!("- Smooth animations and transitions");
    println!();
    println!("Controls:");
    println!("  W/S: Move paddle");
    println!("  SPACE: Start game / Menu navigation");
    println!("  ESC: Pause / Resume");
    println!("  Q: Quit to menu");
    println!("  1/2/3: Select difficulty (in menu)");
    println!();

    // Initialize window and rendering
    let window_config = window::WindowConfig {
        title: "Improved Pong - Modular Game Engine".to_string(),
        width: WINDOW_WIDTH,
        height: WINDOW_HEIGHT,
        resizable: false,
        vsync: true,
    };

    let mut render_context = renderer_2d::RenderContext::new(window_config).unwrap();
    let mut input_manager = input_window::WindowInputManager::new();
    let mut pong_game = ImprovedPongGame::new();

    // Main game loop
    while !render_context.should_close() {
        let current_time = std::time::Instant::now();
        let delta_time = current_time
            .duration_since(pong_game.last_update)
            .as_secs_f32();
        pong_game.last_update = current_time;

        // Update input
        input_manager.update(render_context.window.window_ref());

        // Update game
        pong_game.update(delta_time, input_manager.state());

        // Render
        pong_game.render(&mut render_context.renderer);
        render_context.present().unwrap();

        // Update window
        render_context.update();

        // Small delay to prevent excessive CPU usage
        std::thread::sleep(std::time::Duration::from_millis(16));
    }

    println!("Game closed. Thanks for playing Improved Pong!");
}

// Game systems (enhanced versions)
use specs::{Entities, Join, Read, ReadStorage, System, Write, WriteStorage};

pub struct ImprovedPongInputSystem;
impl<'a> System<'a> for ImprovedPongInputSystem {
    type SystemData = (
        WriteStorage<'a, Velocity>,
        ReadStorage<'a, Paddle>,
        Read<'a, crate::input_window::WindowInputState>,
    );

    fn run(&mut self, (mut velocities, paddles, input_state): Self::SystemData) {
        for (velocity, paddle) in (&mut velocities, &paddles).join() {
            if paddle.player_controlled {
                velocity.y = 0.0;
                if input_state.keys_pressed.contains(&minifb::Key::W) {
                    velocity.y = -PADDLE_SPEED;
                }
                if input_state.keys_pressed.contains(&minifb::Key::S) {
                    velocity.y = PADDLE_SPEED;
                }
            }
        }
    }
}

pub struct ImprovedPongAISystem;
impl<'a> System<'a> for ImprovedPongAISystem {
    type SystemData = (
        ReadStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
        ReadStorage<'a, Paddle>,
        ReadStorage<'a, Ball>,
        Read<'a, Time>,
        Read<'a, Score>,
    );

    fn run(&mut self, (positions, mut velocities, paddles, balls, time, score): Self::SystemData) {
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

                // Adjust AI speed based on score difference
                let score_diff = score.player_score as i32 - score.ai_score as i32;
                let ai_multiplier = match score_diff {
                    -2..=2 => 0.8, // Normal speed
                    3..=5 => 1.0,  // Faster when losing
                    _ => 0.6,      // Slower when winning
                };

                let ai_error = (time.elapsed * 3.0).sin() * 15.0;
                let target_diff = diff + ai_error;

                if target_diff.abs() > 15.0 {
                    velocity.y = target_diff.signum() * PADDLE_SPEED * ai_multiplier;
                } else {
                    velocity.y = 0.0;
                }
            }
        }
    }
}

pub struct ImprovedPongCollisionSystem;
impl<'a> System<'a> for ImprovedPongCollisionSystem {
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
        // Get collision data first to avoid borrowing conflicts
        let ball_positions: Vec<(specs::Entity, Position)> = (&entities, &positions, &balls)
            .join()
            .map(|(entity, pos, _)| (entity, pos.clone()))
            .collect();

        let paddle_positions: Vec<(specs::Entity, Position)> = (&entities, &positions, &paddles)
            .join()
            .map(|(entity, pos, _)| (entity, pos.clone()))
            .collect();

        // Process collisions
        let mut scored_this_frame = false;
        for (ball_entity, ball_pos) in &ball_positions {
            if scored_this_frame {
                break; // Skip processing other balls if we've already scored
            }

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

                        // Add minimal spin based on hit position
                        let paddle_center = paddle_pos.y + PADDLE_HEIGHT / 2.0;
                        let hit_pos = ball_pos.y + BALL_SIZE / 2.0;
                        let spin_factor = (hit_pos - paddle_center) / (PADDLE_HEIGHT / 2.0);
                        vel.y += spin_factor * 50.0; // Minimal spin for better control

                        // Ensure ball doesn't get too fast
                        let speed = (vel.x * vel.x + vel.y * vel.y).sqrt();
                        if speed > BALL_SPEED * 1.5 {
                            vel.x = vel.x / speed * BALL_SPEED * 1.0;
                            vel.y = vel.y / speed * BALL_SPEED * 1.0;
                        }
                    }
                    break; // Only handle first collision
                }
            }

            // Check for scoring
            if ball_pos.x < -BALL_SIZE {
                score.ai_score += 1;
                reset_ball_positions(&mut positions, &mut velocities, &balls);
                scored_this_frame = true;
            } else if ball_pos.x > WINDOW_WIDTH as f32 {
                score.player_score += 1;
                reset_ball_positions(&mut positions, &mut velocities, &balls);
                scored_this_frame = true;
            }
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
        // Always start towards player (left) after reset
        vel.x = -BALL_SPEED;
        vel.y = (rand::random::<f32>() - 0.5) * BALL_SPEED * 0.8;
    }
}

pub struct ImprovedPongGameLogicSystem;
impl<'a> System<'a> for ImprovedPongGameLogicSystem {
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

    // Create ball (center) - start moving towards player (left)
    world
        .create_entity_with_components()
        .with(Position::new(
            WINDOW_WIDTH as f32 / 2.0 - BALL_SIZE / 2.0,
            WINDOW_HEIGHT as f32 / 2.0 - BALL_SIZE / 2.0,
        ))
        .with(Velocity::new(
            -BALL_SPEED,
            (rand::random::<f32>() - 0.5) * BALL_SPEED * 0.5,
        ))
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

fn reset_ball(world: &mut World, speed_multiplier: f32) {
    let mut positions = world.write_storage::<Position>();
    let mut velocities = world.write_storage::<Velocity>();
    let balls = world.read_storage::<Ball>();

    for (pos, vel, _) in (&mut positions, &mut velocities, &balls).join() {
        pos.x = WINDOW_WIDTH as f32 / 2.0 - BALL_SIZE / 2.0;
        pos.y = WINDOW_HEIGHT as f32 / 2.0 - BALL_SIZE / 2.0;
        // Always start towards player (left) after reset
        vel.x = -BALL_SPEED * speed_multiplier;
        vel.y = (rand::random::<f32>() - 0.5) * BALL_SPEED * speed_multiplier * 0.8;
    }
}
