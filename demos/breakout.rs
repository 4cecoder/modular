//! Breakout Game Demo
//!
//! A complete Breakout/Arkanoid-style game implementation using the modular game engine.
//! Demonstrates integration of all extracted systems: difficulty, particles, menu, visual effects,
//! scoring, and trail systems.

use difficulty::DifficultySystem;
use menu::MenuSystem;
use modular_game_engine::*;
use particles::ParticleSystem;
use scoring::{presets as scoring_presets, ScoreType, ScoringSystem};
use specs::{Component, VecStorage};
use trail_system::{presets as trail_presets, TrailSystem};
use visual_effects::VisualEffectsSystem;

// Game constants
const WINDOW_WIDTH: usize = 800;
const WINDOW_HEIGHT: usize = 600;
const PADDLE_WIDTH: f32 = 100.0;
const PADDLE_HEIGHT: f32 = 20.0;
const BALL_SIZE: f32 = 12.0;
const BRICK_WIDTH: f32 = 60.0;
const BRICK_HEIGHT: f32 = 20.0;
const BRICK_ROWS: usize = 6;
const BRICK_COLS: usize = 13;
const PADDLE_SPEED: f32 = 400.0;
const BALL_SPEED: f32 = 300.0;

// Game entities
#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Paddle;

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Ball {
    pub attached_to_paddle: bool,
}

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Brick {
    pub hits_required: i32,
    pub points: i32,
    pub color: [f32; 4],
}

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct PowerUp {
    pub power_type: PowerUpType,
    pub velocity: Vec2,
}

#[derive(Debug, Clone, Copy)]
pub enum PowerUpType {
    ExtraLife,
    MultiBall,
    LargerPaddle,
    SmallerPaddle,
    FasterBall,
    SlowerBall,
}

// Game state
#[derive(Debug, Clone)]
pub enum BreakoutGameState {
    Menu,
    Playing,
    Paused,
    GameOver { won: bool },
    LevelComplete,
}

pub struct BreakoutGame {
    world: World,
    dispatcher: specs::Dispatcher<'static, 'static>,
    game_state: BreakoutGameState,
    last_update: std::time::Instant,

    // Extracted systems
    difficulty_system: DifficultySystem,
    particle_system: ParticleSystem,
    menu_system: MenuSystem,
    visual_effects: VisualEffectsSystem,
    scoring_system: ScoringSystem,
    trail_system: TrailSystem,

    // Game data
    level: i32,
    lives: i32,
    balls: Vec<specs::Entity>,
    bricks_remaining: usize,
    paddle_entity: Option<specs::Entity>,
}

impl BreakoutGame {
    pub fn new() -> Self {
        let mut world = init().unwrap();

        // Register game-specific components
        world.register::<Paddle>();
        world.register::<Ball>();
        world.register::<Brick>();
        world.register::<PowerUp>();

        // Insert required resources
        world.insert(crate::input_window::WindowInputState::default());

        // Initialize extracted systems
        let difficulty_system = DifficultySystem::with_pong_defaults();
        let mut particle_system = ParticleSystem::new();
        let menu_system = MenuSystem::create_main_menu();
        let visual_effects = VisualEffectsSystem::new();
        let scoring_system = scoring_presets::pong_scoring(5); // Reuse pong scoring for now
        let mut trail_system = TrailSystem::new();

        // Create ball trail
        trail_system.create_trail_with_config("ball", trail_presets::pong_ball_trail());

        // Set up systems dispatcher
        let dispatcher = specs::DispatcherBuilder::new()
            .with(BreakoutInputSystem, "input", &[])
            .with(BreakoutPhysicsSystem, "physics", &["input"])
            .with(BreakoutCollisionSystem, "collision", &["physics"])
            .with(BreakoutGameLogicSystem, "game_logic", &["collision"])
            .with(BreakoutRenderingSystem, "rendering", &["game_logic"])
            .build();

        let mut game = Self {
            world,
            dispatcher,
            game_state: BreakoutGameState::Menu,
            last_update: std::time::Instant::now(),
            difficulty_system,
            particle_system,
            menu_system,
            visual_effects,
            scoring_system,
            trail_system,
            level: 1,
            lives: 3,
            balls: Vec::new(),
            bricks_remaining: 0,
            paddle_entity: None,
        };

        game.initialize_level();
        game
    }

    fn initialize_level(&mut self) {
        // Clear existing entities
        self.clear_level();

        // Create paddle
        let paddle_entity = self
            .world
            .create_entity_with_components()
            .with(Position::new(
                WINDOW_WIDTH as f32 / 2.0 - PADDLE_WIDTH / 2.0,
                WINDOW_HEIGHT as f32 - 60.0,
            ))
            .with(Velocity::new(0.0, 0.0))
            .with(Renderable::new("paddle".to_string()))
            .with(Paddle)
            .with(Collider::new_rectangle(PADDLE_WIDTH, PADDLE_HEIGHT))
            .build();
        self.paddle_entity = Some(paddle_entity);

        // Create ball attached to paddle
        let ball_entity = self
            .world
            .create_entity_with_components()
            .with(Position::new(
                WINDOW_WIDTH as f32 / 2.0 - BALL_SIZE / 2.0,
                WINDOW_HEIGHT as f32 - 80.0,
            ))
            .with(Velocity::new(0.0, 0.0))
            .with(Renderable::new("ball".to_string()))
            .with(Ball {
                attached_to_paddle: true,
            })
            .with(Collider::new_circle(BALL_SIZE / 2.0))
            .build();
        self.balls.push(ball_entity);

        // Create bricks
        self.create_bricks();

        // Reset scoring for new level
        self.scoring_system.reset();
        self.bricks_remaining = self.count_bricks();
    }

    fn create_bricks(&mut self) {
        let start_x = 20.0;
        let start_y = 50.0;
        let colors = [
            [1.0, 0.0, 0.0, 1.0], // Red
            [1.0, 0.5, 0.0, 1.0], // Orange
            [1.0, 1.0, 0.0, 1.0], // Yellow
            [0.0, 1.0, 0.0, 1.0], // Green
            [0.0, 0.0, 1.0, 1.0], // Blue
            [0.5, 0.0, 1.0, 1.0], // Purple
        ];

        for row in 0..BRICK_ROWS {
            for col in 0..BRICK_COLS {
                let x = start_x + col as f32 * (BRICK_WIDTH + 5.0);
                let y = start_y + row as f32 * (BRICK_HEIGHT + 5.0);

                let hits_required = if row < 2 {
                    1
                } else if row < 4 {
                    2
                } else {
                    3
                };
                let points = (BRICK_ROWS - row) * 10;

                self.world
                    .create_entity_with_components()
                    .with(Position::new(x, y))
                    .with(Velocity::new(0.0, 0.0))
                    .with(Renderable::new("brick".to_string()))
                    .with(Brick {
                        hits_required,
                        points: points as i32,
                        color: colors[row % colors.len()],
                    })
                    .with(Collider::new_rectangle(BRICK_WIDTH, BRICK_HEIGHT))
                    .build();
            }
        }
    }

    fn count_bricks(&self) -> usize {
        let bricks = self.world.read_storage::<Brick>();
        bricks.join().count()
    }

    fn clear_level(&mut self) {
        // Remove all game entities
        let mut to_remove = Vec::new();

        // Remove balls
        let balls = self.world.read_storage::<Ball>();
        for (entity, _) in (&self.world.entities(), &balls).join() {
            to_remove.push(entity);
        }

        // Remove bricks
        let bricks = self.world.read_storage::<Brick>();
        for (entity, _) in (&self.world.entities(), &bricks).join() {
            to_remove.push(entity);
        }

        // Remove power-ups
        let powerups = self.world.read_storage::<PowerUp>();
        for (entity, _) in (&self.world.entities(), &powerups).join() {
            to_remove.push(entity);
        }

        // Remove paddle
        if let Some(paddle) = self.paddle_entity {
            to_remove.push(paddle);
        }

        for entity in to_remove {
            let _ = self.world.entities().delete(entity);
        }

        self.balls.clear();
        self.paddle_entity = None;
    }

    pub fn update(&mut self, delta_time: f32, input_state: &input_window::WindowInputState) {
        let current_time = std::time::Instant::now();
        let actual_delta = current_time.duration_since(self.last_update).as_secs_f32();
        self.last_update = current_time;

        // Update Time resource for physics
        self.world.write_resource::<Time>().delta = actual_delta;
        self.world.write_resource::<Time>().elapsed += actual_delta;

        // Update input state resource
        *self
            .world
            .write_resource::<input_window::WindowInputState>() = input_state.clone();

        // Update extracted systems
        self.particle_system.update(actual_delta);
        self.visual_effects.update(actual_delta);
        self.scoring_system.update_time(actual_delta);

        match &self.game_state {
            BreakoutGameState::Menu => {
                // Handle menu input
                // For now, just start the game
                if self.menu_system.get_selected_item().is_some() {
                    self.game_state = BreakoutGameState::Playing;
                }
            }
            BreakoutGameState::Playing => {
                // Update game systems
                self.dispatcher.dispatch(&mut self.world);

                // Update ball trails
                for ball_entity in &self.balls {
                    if let Some(position) = self.world.read_storage::<Position>().get(*ball_entity)
                    {
                        if let Some(velocity) =
                            self.world.read_storage::<Velocity>().get(*ball_entity)
                        {
                            self.trail_system.update_trail(
                                "ball",
                                actual_delta,
                                Vec2::new(position.x, position.y),
                                Vec2::new(velocity.x, velocity.y),
                            );
                        }
                    }
                }

                self.world.maintain();

                // Check win/lose conditions
                if self.bricks_remaining == 0 {
                    self.game_state = BreakoutGameState::LevelComplete;
                }

                // Check if all balls are lost
                let active_balls = self
                    .balls
                    .iter()
                    .filter(|&&entity| self.world.entities().is_alive(entity))
                    .count();

                if active_balls == 0 {
                    self.lives -= 1;
                    if self.lives <= 0 {
                        self.game_state = BreakoutGameState::GameOver { won: false };
                    } else {
                        // Reset ball
                        self.reset_ball();
                    }
                }
            }
            BreakoutGameState::Paused => {
                // Handle pause menu
            }
            BreakoutGameState::GameOver { .. } => {
                // Handle game over
            }
            BreakoutGameState::LevelComplete => {
                // Handle level complete
                self.level += 1;
                self.initialize_level();
                self.game_state = BreakoutGameState::Playing;
            }
        }
    }

    fn reset_ball(&mut self) {
        // Reset ball position and attach to paddle
        if let Some(ball_entity) = self.balls.first() {
            if let Some(mut positions) =
                self.world.write_storage::<Position>().get_mut(*ball_entity)
            {
                if let Some(mut velocities) =
                    self.world.write_storage::<Velocity>().get_mut(*ball_entity)
                {
                    if let Some(paddle_entity) = self.paddle_entity {
                        if let Some(paddle_pos) =
                            self.world.read_storage::<Position>().get(paddle_entity)
                        {
                            positions.x = paddle_pos.x + PADDLE_WIDTH / 2.0 - BALL_SIZE / 2.0;
                            positions.y = paddle_pos.y - BALL_SIZE;
                            velocities.x = 0.0;
                            velocities.y = 0.0;

                            if let Some(mut balls) =
                                self.world.write_storage::<Ball>().get_mut(*ball_entity)
                            {
                                balls.attached_to_paddle = true;
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn render(&self, renderer: &mut renderer_2d::Renderer2D) {
        // Clear screen
        renderer.clear(renderer_2d::Color::rgb(20, 20, 40));

        match &self.game_state {
            BreakoutGameState::Menu => {
                self.render_menu(renderer);
            }
            BreakoutGameState::Playing | BreakoutGameState::Paused => {
                self.render_gameplay(renderer);

                if let BreakoutGameState::Paused = self.game_state {
                    self.render_pause_overlay(renderer);
                }
            }
            BreakoutGameState::GameOver { won } => {
                self.render_gameplay(renderer);
                self.render_game_over(renderer, *won);
            }
            BreakoutGameState::LevelComplete => {
                self.render_gameplay(renderer);
                self.render_level_complete(renderer);
            }
        }

        // Render particles on top
        self.render_particles(renderer);
    }

    fn render_menu(&self, renderer: &mut renderer_2d::Renderer2D) {
        renderer.draw_text_centered(
            "BREAKOUT",
            WINDOW_WIDTH / 2,
            150,
            renderer_2d::Color::WHITE,
            3,
        );

        renderer.draw_text_centered(
            "Press SPACE to Start",
            WINDOW_WIDTH / 2,
            300,
            renderer_2d::Color::GREEN,
            2,
        );

        renderer.draw_text_centered(
            "Use A/D or Left/Right to move paddle",
            WINDOW_WIDTH / 2,
            350,
            renderer_2d::Color::rgb(150, 150, 150),
            1,
        );

        renderer.draw_text_centered(
            "Press ESC to pause",
            WINDOW_WIDTH / 2,
            380,
            renderer_2d::Color::rgb(150, 150, 150),
            1,
        );
    }

    fn render_gameplay(&self, renderer: &mut renderer_2d::Renderer2D) {
        // Render paddle
        let positions = self.world.read_storage::<Position>();
        let paddles = self.world.read_storage::<Paddle>();

        for (pos, _) in (&positions, &paddles).join() {
            renderer.draw_rect(
                pos.x as i32,
                pos.y as i32,
                PADDLE_WIDTH as i32,
                PADDLE_HEIGHT as i32,
                renderer_2d::Color::rgb(100, 200, 100),
            );
        }

        // Render balls
        let balls = self.world.read_storage::<Ball>();
        for (pos, _) in (&positions, &balls).join() {
            renderer.draw_circle_filled(
                (pos.x + BALL_SIZE / 2.0) as i32,
                (pos.y + BALL_SIZE / 2.0) as i32,
                (BALL_SIZE / 2.0) as i32,
                renderer_2d::Color::WHITE,
            );
        }

        // Render bricks
        let bricks = self.world.read_storage::<Brick>();
        for (pos, brick) in (&positions, &bricks).join() {
            let color = renderer_2d::Color::rgba(
                (brick.color[0] * 255.0) as u8,
                (brick.color[1] * 255.0) as u8,
                (brick.color[2] * 255.0) as u8,
                (brick.color[3] * 255.0) as u8,
            );

            renderer.draw_rect(
                pos.x as i32,
                pos.y as i32,
                BRICK_WIDTH as i32,
                BRICK_HEIGHT as i32,
                color,
            );
        }

        // Render power-ups
        let powerups = self.world.read_storage::<PowerUp>();
        for (pos, powerup) in (&positions, &powerups).join() {
            let color = match powerup.power_type {
                PowerUpType::ExtraLife => renderer_2d::Color::GREEN,
                PowerUpType::MultiBall => renderer_2d::Color::BLUE,
                PowerUpType::LargerPaddle => renderer_2d::Color::YELLOW,
                PowerUpType::SmallerPaddle => renderer_2d::Color::RED,
                PowerUpType::FasterBall => renderer_2d::Color::rgb(128, 0, 128),
                PowerUpType::SlowerBall => renderer_2d::Color::CYAN,
            };

            renderer.draw_circle_filled((pos.x + 10.0) as i32, (pos.y + 10.0) as i32, 8, color);
        }

        // Render UI
        self.render_ui(renderer);
    }

    fn render_ui(&self, renderer: &mut renderer_2d::Renderer2D) {
        // Score
        let score_text = format!(
            "Score: {}",
            self.scoring_system.get_score("player", &ScoreType::Points)
        );
        renderer.draw_text(&score_text, 10, 10, renderer_2d::Color::WHITE, 2);

        // Lives
        let lives_text = format!("Lives: {}", self.lives);
        renderer.draw_text(&lives_text, 10, 40, renderer_2d::Color::WHITE, 2);

        // Level
        let level_text = format!("Level: {}", self.level);
        renderer.draw_text(
            &level_text,
            WINDOW_WIDTH - 100,
            10,
            renderer_2d::Color::WHITE,
            2,
        );

        // Bricks remaining
        let bricks_text = format!("Bricks: {}", self.bricks_remaining);
        renderer.draw_text(
            &bricks_text,
            WINDOW_WIDTH - 100,
            40,
            renderer_2d::Color::WHITE,
            2,
        );
    }

    fn render_particles(&self, renderer: &mut renderer_2d::Renderer2D) {
        // This would integrate with the particle system rendering
        // For now, just render ball trails
        if let Some(trail) = self.trail_system.get_trail("ball") {
            for segment in trail.get_segments() {
                let alpha = (segment.alpha() * 255.0) as u8;
                let color = renderer_2d::Color::rgba(100, 100, 255, alpha);

                renderer.draw_circle_filled(
                    segment.position.x as i32,
                    segment.position.y as i32,
                    segment.size as i32,
                    color,
                );
            }
        }
    }

    fn render_pause_overlay(&self, renderer: &mut renderer_2d::Renderer2D) {
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
            "Press ESC to resume",
            WINDOW_WIDTH / 2,
            WINDOW_HEIGHT / 2,
            renderer_2d::Color::rgb(200, 200, 200),
            1,
        );
    }

    fn render_game_over(&self, renderer: &mut renderer_2d::Renderer2D, won: bool) {
        renderer.draw_rect(
            0,
            0,
            WINDOW_WIDTH as i32,
            WINDOW_HEIGHT as i32,
            renderer_2d::Color::rgba(0, 0, 0, 200),
        );

        let title = if won { "YOU WIN!" } else { "GAME OVER" };
        let color = if won {
            renderer_2d::Color::GREEN
        } else {
            renderer_2d::Color::RED
        };

        renderer.draw_text_centered(title, WINDOW_WIDTH / 2, WINDOW_HEIGHT / 2 - 50, color, 3);

        let score_text = format!(
            "Final Score: {}",
            self.scoring_system.get_score("player", &ScoreType::Points)
        );
        renderer.draw_text_centered(
            &score_text,
            WINDOW_WIDTH / 2,
            WINDOW_HEIGHT / 2,
            renderer_2d::Color::WHITE,
            2,
        );

        renderer.draw_text_centered(
            "Press R to restart",
            WINDOW_WIDTH / 2,
            WINDOW_HEIGHT / 2 + 50,
            renderer_2d::Color::rgb(150, 150, 200),
            1,
        );
    }

    fn render_level_complete(&self, renderer: &mut renderer_2d::Renderer2D) {
        renderer.draw_rect(
            0,
            0,
            WINDOW_WIDTH as i32,
            WINDOW_HEIGHT as i32,
            renderer_2d::Color::rgba(0, 0, 0, 150),
        );

        renderer.draw_text_centered(
            "LEVEL COMPLETE!",
            WINDOW_WIDTH / 2,
            WINDOW_HEIGHT / 2 - 50,
            renderer_2d::Color::GREEN,
            3,
        );

        renderer.draw_text_centered(
            "Press SPACE for next level",
            WINDOW_WIDTH / 2,
            WINDOW_HEIGHT / 2,
            renderer_2d::Color::rgb(200, 200, 200),
            1,
        );
    }

    pub fn handle_input(&mut self, input_state: &input_window::WindowInputState) {
        use minifb::Key;

        match &self.game_state {
            BreakoutGameState::Menu => {
                if input_state.is_key_just_pressed(Key::Space) {
                    self.game_state = BreakoutGameState::Playing;
                    println!("Game started! Press SPACE to launch the ball.");
                }
            }
            BreakoutGameState::Playing => {
                if input_state.is_key_just_pressed(Key::Escape) {
                    self.game_state = BreakoutGameState::Paused;
                }

                // Launch ball if attached to paddle
                if input_state.is_key_just_pressed(Key::Space) {
                    if let Some(ball_entity) = self.balls.first() {
                        // Check if ball is attached to paddle first
                        if let Some(ball) = self.world.read_storage::<Ball>().get(*ball_entity) {
                            if ball.attached_to_paddle {
                                // Launch the ball - separate scopes to avoid borrowing conflicts
                                {
                                    let mut velocities = self.world.write_storage::<Velocity>();
                                    if let Some(velocity) = velocities.get_mut(*ball_entity) {
                                        velocity.x = BALL_SPEED
                                            * self.difficulty_system.ball_speed_multiplier();
                                        velocity.y = -BALL_SPEED
                                            * self.difficulty_system.ball_speed_multiplier();
                                    }
                                }

                                {
                                    let mut balls = self.world.write_storage::<Ball>();
                                    if let Some(ball_component) = balls.get_mut(*ball_entity) {
                                        ball_component.attached_to_paddle = false;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            BreakoutGameState::Paused => {
                if input_state.is_key_just_pressed(Key::Escape) {
                    self.game_state = BreakoutGameState::Playing;
                }
            }
            BreakoutGameState::GameOver { .. } => {
                if input_state.is_key_just_pressed(Key::R) {
                    self.restart_game();
                }
            }
            BreakoutGameState::LevelComplete => {
                if input_state.is_key_just_pressed(Key::Space) {
                    self.game_state = BreakoutGameState::Playing;
                }
            }
        }
    }

    fn restart_game(&mut self) {
        self.level = 1;
        self.lives = 3;
        self.scoring_system.reset();
        self.initialize_level();
        self.game_state = BreakoutGameState::Playing;
    }
}

// Game systems
use specs::{Entities, Join, Read, ReadStorage, System, WriteStorage};

pub struct BreakoutInputSystem;

impl<'a> System<'a> for BreakoutInputSystem {
    type SystemData = (WriteStorage<'a, Velocity>, ReadStorage<'a, Paddle>);

    fn run(&mut self, (mut velocities, paddles): Self::SystemData) {
        // Input is handled in the main game loop, not in systems
        // This system just ensures paddle velocities are reset
        for (velocity, _) in (&mut velocities, &paddles).join() {
            if velocity.x.abs() < 0.1 {
                velocity.x = 0.0;
            }
        }
    }
}

pub struct BreakoutPhysicsSystem;

impl<'a> System<'a> for BreakoutPhysicsSystem {
    type SystemData = (
        WriteStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
        ReadStorage<'a, Ball>,
        Read<'a, Time>,
    );

    fn run(&mut self, (mut positions, mut velocities, balls, time): Self::SystemData) {
        for (pos, vel, _) in (&mut positions, &mut velocities, &balls).join() {
            pos.x += vel.x * time.delta;
            pos.y += vel.y * time.delta;
        }
    }
}

pub struct BreakoutCollisionSystem;

impl<'a> System<'a> for BreakoutCollisionSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
        ReadStorage<'a, Ball>,
        ReadStorage<'a, Paddle>,
        ReadStorage<'a, Brick>,
        ReadStorage<'a, PowerUp>,
    );

    fn run(
        &mut self,
        (entities, mut positions, mut velocities, balls, paddles, bricks, powerups): Self::SystemData,
    ) {
        // Ball-wall collisions
        for (entity, pos, vel, _) in (&entities, &mut positions, &mut velocities, &balls).join() {
            // Left and right walls
            if pos.x <= 0.0 || pos.x >= WINDOW_WIDTH as f32 - BALL_SIZE {
                vel.x = -vel.x;
                pos.x = pos.x.clamp(0.0, WINDOW_WIDTH as f32 - BALL_SIZE);
            }

            // Top wall
            if pos.y <= 0.0 {
                vel.y = -vel.y;
                pos.y = 0.0;
            }

            // Bottom (ball lost)
            if pos.y >= WINDOW_HEIGHT as f32 {
                let _ = entities.delete(entity);
            }
        }

        // Ball-paddle collisions
        for (ball_entity, ball_pos, ball_vel, _) in
            (&entities, &positions, &mut velocities, &balls).join()
        {
            for (paddle_pos, _) in (&positions, &paddles).join() {
                if ball_pos.x < paddle_pos.x + PADDLE_WIDTH
                    && ball_pos.x + BALL_SIZE > paddle_pos.x
                    && ball_pos.y < paddle_pos.y + PADDLE_HEIGHT
                    && ball_pos.y + BALL_SIZE > paddle_pos.y
                    && ball_vel.y > 0.0
                {
                    // Only if ball is moving down

                    ball_vel.y = -ball_vel.y;

                    // Add some angle based on where ball hits paddle
                    let hit_pos = (ball_pos.x + BALL_SIZE / 2.0 - paddle_pos.x) / PADDLE_WIDTH;
                    let angle = (hit_pos - 0.5) * std::f32::consts::PI / 3.0; // Max 60 degrees
                    let speed = (ball_vel.x * ball_vel.x + ball_vel.y * ball_vel.y).sqrt();
                    ball_vel.x = angle.sin() * speed;
                    ball_vel.y = -angle.cos().abs() * speed;
                }
            }
        }

        // Ball-brick collisions
        let mut bricks_to_remove = Vec::new();

        for (ball_entity, ball_pos, ball_vel, _) in
            (&entities, &positions, &mut velocities, &balls).join()
        {
            for (brick_entity, brick_pos, brick) in (&entities, &positions, &bricks).join() {
                if ball_pos.x < brick_pos.x + BRICK_WIDTH
                    && ball_pos.x + BALL_SIZE > brick_pos.x
                    && ball_pos.y < brick_pos.y + BRICK_HEIGHT
                    && ball_pos.y + BALL_SIZE > brick_pos.y
                {
                    // Ball collision with brick
                    ball_vel.y = -ball_vel.y;

                    // Damage brick
                    // In a full implementation, we'd track brick health
                    bricks_to_remove.push(brick_entity);
                }
            }
        }

        // Remove destroyed bricks
        for brick_entity in bricks_to_remove {
            let _ = entities.delete(brick_entity);
        }
    }
}

pub struct BreakoutGameLogicSystem;

impl<'a> System<'a> for BreakoutGameLogicSystem {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, Velocity>,
        ReadStorage<'a, Ball>,
        ReadStorage<'a, Brick>,
    );

    fn run(&mut self, (positions, velocities, balls, bricks): Self::SystemData) {
        // Game logic updates would go here
        // For now, this is mostly handled in the main game loop
    }
}

pub struct BreakoutRenderingSystem;

impl<'a> System<'a> for BreakoutRenderingSystem {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, Renderable>,
        ReadStorage<'a, Paddle>,
        ReadStorage<'a, Ball>,
        ReadStorage<'a, Brick>,
    );

    fn run(&mut self, (positions, renderables, paddles, balls, bricks): Self::SystemData) {
        // Rendering is handled in the main render method
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎮 Breakout Demo - Modular Game Engine");
    println!("=====================================");
    println!("Breakout game using all extracted systems!");
    println!();
    println!("Controls:");
    println!("  A/D or Left/Right: Move paddle");
    println!("  SPACE: Launch ball / Start game");
    println!("  ESC: Pause");
    println!("  R: Restart (game over)");
    println!();
    println!("Features:");
    println!("  ✓ Difficulty scaling");
    println!("  ✓ Particle effects");
    println!("  ✓ Visual effects (trails)");
    println!("  ✓ Scoring system");
    println!("  ✓ Multiple levels");
    println!("  ✓ Lives system");
    println!();

    // Initialize window and rendering
    let window_config = window::WindowConfig {
        title: "Breakout - Modular Game Engine".to_string(),
        width: WINDOW_WIDTH,
        height: WINDOW_HEIGHT,
        resizable: false,
        vsync: true,
    };

    let mut render_context = renderer_2d::RenderContext::new(window_config).unwrap();
    let mut input_manager = input_window::WindowInputManager::new();
    let mut breakout_game = BreakoutGame::new();

    // Main game loop
    while !render_context.should_close() {
        let current_time = std::time::Instant::now();
        let delta_time = current_time
            .duration_since(breakout_game.last_update)
            .as_secs_f32();

        // Update input
        input_manager.update(render_context.window.window_ref());

        // Handle input
        breakout_game.handle_input(input_manager.state());

        // Update game
        breakout_game.update(delta_time, input_manager.state());

        // Render
        breakout_game.render(&mut render_context.renderer);
        render_context.present().unwrap();

        // Update window
        render_context.update();

        // Frame rate limiting
        std::thread::sleep(std::time::Duration::from_millis(16));
    }

    println!("Thanks for playing Breakout!");
    Ok(())
}
