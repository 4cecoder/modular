//! Complete Game Example
//!
//! Demonstrates the full modular game engine with:
//! - Window management
//! - 2D rendering
//! - Input handling
//! - Game state management
//! - ECS integration

use modular_game_engine::*;

// Game constants
const WINDOW_WIDTH: usize = 800;
const WINDOW_HEIGHT: usize = 600;
const PADDLE_WIDTH: f32 = 20.0;
const PADDLE_HEIGHT: f32 = 100.0;
const BALL_SIZE: f32 = 15.0;
const PADDLE_SPEED: f32 = 300.0;
const BALL_SPEED: f32 = 400.0;

// Game entity data
struct GameEntities {
    player_paddle: specs::Entity,
    ai_paddle: specs::Entity,
    ball: specs::Entity,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎮 Complete Game Engine Demo");
    println!("============================");
    println!("This demo showcases the full modular game engine:");
    println!("- Window management");
    println!("- 2D rendering system");
    println!("- Input handling");
    println!("- Game state management");
    println!("- ECS integration");
    println!();
    println!("Controls:");
    println!("  W/S: Move paddle");
    println!("  SPACE: Start game / Menu navigation");
    println!("  ESC: Pause / Menu");
    println!("  Q: Quit");
    println!();

    // Initialize the game world
    let mut world = init().unwrap();

    // Register game-specific components
    world.register::<Paddle>();
    world.register::<Ball>();
    world.register::<Score>();

    // Create game entities
    let entities = create_game_entities(&mut world);

    // Set up systems
    let mut dispatcher = specs::DispatcherBuilder::new()
        .with(PongInputSystem, "input", &[])
        .with(PongAISystem, "ai", &["input"])
        .with(PhysicsSystem, "physics", &["ai"])
        .with(PongCollisionSystem, "collision", &["physics"])
        .with(PongGameLogicSystem, "game_logic", &["collision"])
        .build();

    // Initialize window and rendering
    let window_config = window::WindowConfig {
        title: "Complete Game Engine Demo".to_string(),
        width: WINDOW_WIDTH,
        height: WINDOW_HEIGHT,
        resizable: false,
        vsync: true,
    };

    let mut render_context = renderer_2d::RenderContext::new(window_config)?;
    let mut input_manager = input_window::WindowInputManager::new();
    let mut game_controller = input_window::WindowGameController::new();

    // Initialize game state manager
    let mut state_manager = game_state::StateManager::new(WINDOW_WIDTH, WINDOW_HEIGHT);

    // Register game states
    state_manager.register_state(Box::new(game_state::MenuState::new()));
    state_manager.register_state(Box::new(GameplayState::new(entities, dispatcher)));
    state_manager.register_state(Box::new(game_state::PauseState));
    state_manager.register_state(Box::new(game_state::GameOverState::new(0)));

    // Start with menu
    state_manager.switch_to("menu".to_string())?;

    // Main game loop
    let mut last_time = std::time::Instant::now();

    while !render_context.should_close() {
        let current_time = std::time::Instant::now();
        let delta_time = current_time.duration_since(last_time).as_secs_f32();
        last_time = current_time;

        // Update input
        input_manager.update(render_context.window.window_ref());
        game_controller.update(input_manager.state());

        // Update game state
        let state_transition = state_manager.update(delta_time);

        // Handle state transitions
        match state_transition {
            game_state::StateTransition::Switch(state_id) => {
                if let Err(e) = state_manager.switch_to(state_id) {
                    eprintln!("Failed to switch state: {}", e);
                }
            }
            game_state::StateTransition::Push(state_id) => {
                if let Err(e) = state_manager.push_state(state_id) {
                    eprintln!("Failed to push state: {}", e);
                }
            }
            game_state::StateTransition::Pop => {
                if let Err(e) = state_manager.pop_state() {
                    eprintln!("Failed to pop state: {}", e);
                }
            }
            game_state::StateTransition::Quit => {
                break;
            }
            game_state::StateTransition::None => {}
        }

        // Handle input for state transitions
        if let Some(transition) = state_manager.handle_input(input_manager.state()) {
            match transition {
                game_state::StateTransition::Switch(state_id) => {
                    if let Err(e) = state_manager.switch_to(state_id) {
                        eprintln!("Failed to switch state: {}", e);
                    }
                }
                game_state::StateTransition::Push(state_id) => {
                    if let Err(e) = state_manager.push_state(state_id) {
                        eprintln!("Failed to push state: {}", e);
                    }
                }
                game_state::StateTransition::Pop => {
                    if let Err(e) = state_manager.pop_state() {
                        eprintln!("Failed to pop state: {}", e);
                    }
                }
                game_state::StateTransition::Quit => {
                    break;
                }
                game_state::StateTransition::None => {}
            }
        }

        // Render current state
        state_manager.render();

        // Present frame
        if let Err(e) = render_context.present() {
            eprintln!("Render error: {}", e);
            break;
        }

        // Update window
        render_context.update();

        // Small delay to prevent excessive CPU usage
        std::thread::sleep(std::time::Duration::from_millis(16));
    }

    println!("Game closed. Thanks for playing!");
    Ok(())
}

fn create_game_entities(world: &mut World) -> GameEntities {
    // Create player paddle (left side)
    let player_paddle = world.create_entity_with_components()
        .with(Position::new(50.0, WINDOW_HEIGHT as f32 / 2.0 - PADDLE_HEIGHT / 2.0))
        .with(Velocity::new(0.0, 0.0))
        .with(Renderable::new("player_paddle".to_string()))
        .with(Paddle { player_controlled: true })
        .with(Collider::new_rectangle(PADDLE_WIDTH, PADDLE_HEIGHT))
        .build();

    // Create AI paddle (right side)
    let ai_paddle = world.create_entity_with_components()
        .with(Position::new(WINDOW_WIDTH as f32 - 50.0 - PADDLE_WIDTH, WINDOW_HEIGHT as f32 / 2.0 - PADDLE_HEIGHT / 2.0))
        .with(Velocity::new(0.0, 0.0))
        .with(Renderable::new("ai_paddle".to_string()))
        .with(Paddle { player_controlled: false })
        .with(Collider::new_rectangle(PADDLE_WIDTH, PADDLE_HEIGHT))
        .build();

    // Create ball (center)
    let ball = world.create_entity_with_components()
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

    GameEntities {
        player_paddle,
        ai_paddle,
        ball,
    }
}

// Custom gameplay state that integrates with ECS
struct GameplayState {
    entities: GameEntities,
    dispatcher: specs::Dispatcher<'static, 'static>,
    world: World,
    score: (u32, u32),
    game_time: f32,
}

impl GameplayState {
    fn new(entities: GameEntities, dispatcher: specs::Dispatcher<'static, 'static>) -> Self {
        let world = init().unwrap();
        // Note: In a real implementation, you'd pass the world from main
        // For this demo, we create a new one

        Self {
            entities,
            dispatcher,
            world,
            score: (0, 0),
            game_time: 0.0,
        }
    }
}

impl game_state::GameState for GameplayState {
    fn on_enter(&mut self, _context: &mut game_state::StateContext) {
        println!("🎮 Starting gameplay!");
        self.score = (0, 0);
        self.game_time = 0.0;

        // Reset ball position
        reset_ball(&mut self.world);
    }

    fn update(&mut self, context: &mut game_state::StateContext, delta_time: f32) -> game_state::StateTransition {
        self.game_time += delta_time;

        // Update time resource
        self.world.write_resource::<Time>().delta = delta_time;
        self.world.write_resource::<Time>().elapsed += delta_time;

        // Run game systems
        self.dispatcher.dispatch(&mut self.world);
        self.world.maintain();

        // Update score from world
        let score_resource = self.world.read_resource::<Score>();
        self.score = (score_resource.player_score, score_resource.ai_score);

        // Check for game end
        if self.score.0 >= 5 {
            return game_state::StateTransition::Switch("game_over".to_string());
        } else if self.score.1 >= 5 {
            return game_state::StateTransition::Switch("game_over".to_string());
        }

        game_state::StateTransition::None
    }

    fn handle_input(&mut self, _context: &mut game_state::StateContext, input: &input_window::WindowInputState) -> Option<game_state::StateTransition> {
        use minifb::Key;

        if input.is_key_just_pressed(Key::Escape) {
            return Some(game_state::StateTransition::Push("pause".to_string()));
        }

        None
    }

    fn render(&mut self, context: &mut game_state::StateContext) {
        // In a real implementation, this would render the game using the 2D renderer
        println!("🎮 Gameplay - Score: {} | AI: {} | Time: {:.1}s",
                self.score.0, self.score.1, self.game_time);
    }

    fn id(&self) -> game_state::StateId {
        "gameplay".to_string()
    }
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

// Game systems (reuse from window pong)
use specs::{System, ReadStorage, WriteStorage, Read, Write, Entities, Join};

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
                // Input is handled by the window input system
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
        ReadStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
        ReadStorage<'a, Ball>,
        ReadStorage<'a, Paddle>,
        Write<'a, Score>,
    );

    fn run(&mut self, (entities, positions, mut velocities, balls, paddles, mut score): Self::SystemData) {
        for (ball_entity, ball_pos, _) in (&entities, &positions, &balls).join() {
            // Ball collision with top/bottom walls
            if ball_pos.y <= 0.0 || ball_pos.y >= WINDOW_HEIGHT as f32 - BALL_SIZE {
                if let Some(vel) = velocities.get_mut(ball_entity) {
                    vel.y = -vel.y;
                }
            }

            // Ball collision with paddles
            for (paddle_pos, _, _) in (&positions, &paddles).join() {
                if check_paddle_ball_collision(ball_pos, paddle_pos) {
                    if let Some(vel) = velocities.get_mut(ball_entity) {
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