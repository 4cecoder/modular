//! Graphical Pong Demo
//!
//! A complete graphical Pong game with main menu, using wgpu for rendering.
//! Features proper graphics, menu system, and game states.

use modular_game_engine::*;
use specs::{VecStorage, DenseVecStorage};
use specs_derive::{Component, storage};
use winit::{
    event::{Event, WindowEvent, KeyboardInput, ElementState, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, Window},
};
use wgpu::{util::DeviceExt, SurfaceError, StoreOp};
use std::time::{Duration, Instant};

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

#[derive(Debug, Clone)]
pub struct PongGame {
    world: World,
    dispatcher: specs::Dispatcher<'static, 'static>,
    game_state: GameState,
    last_update: Instant,
    score: (u32, u32), // (player, ai)
}

// Vertex for rendering
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}

impl Vertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

// Rendering data
struct RenderData {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    num_vertices: u32,
}

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Graphical Pong - Modular Game Engine")
        .with_inner_size(winit::dpi::PhysicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT))
        .build(&event_loop)
        .unwrap();

    let mut pong_game = PongGame::new();
    let mut render_data = pollster::block_on(create_render_data(&window));

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent { event, window_id } if window_id == window.id() => {
                match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::KeyboardInput { input, .. } => {
                        handle_keyboard_input(&mut pong_game, input);
                    }
                    WindowEvent::Resized(size) => {
                        render_data.config.width = size.width;
                        render_data.config.height = size.height;
                        render_data.surface.configure(&render_data.device, &render_data.config);
                    }
                    _ => {}
                }
            }
            Event::MainEventsCleared => {
                pong_game.update();
                window.request_redraw();
            }
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                update_render_data(&mut render_data, &pong_game);
                match render(&render_data, &pong_game) {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost) => {
                        render_data.surface.configure(&render_data.device, &render_data.config);
                    }
                    Err(e) => eprintln!("Render error: {:?}", e),
                }
            }
            _ => {}
        }
    });
}

impl PongGame {
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
                if let Some(score) = self.world.read_resource::<Score>().first() {
                    self.score = (score.player_score, score.ai_score);
                }

                // Check for game end
                if self.score.0 >= 5 {
                    self.game_state = GameState::GameOver { winner: "Player".to_string() };
                } else if self.score.1 >= 5 {
                    self.game_state = GameState::GameOver { winner: "AI".to_string() };
                }
            }
            _ => {}
        }
    }

    fn start_game(&mut self) {
        self.game_state = GameState::Playing;
        self.score = (0, 0);

        // Reset score in world
        if let Some(mut score) = self.world.write_resource::<Score>().first_mut() {
            score.player_score = 0;
            score.ai_score = 0;
        }

        // Reset ball position
        reset_ball(&mut self.world);
    }

    fn reset_game(&mut self) {
        self.game_state = GameState::Menu;
        self.score = (0, 0);
        reset_ball(&mut self.world);
    }
}

fn handle_keyboard_input(game: &mut PongGame, input: KeyboardInput) {
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
            (VirtualKeyCode::Space, ElementState::Pressed) => {
                match game.game_state {
                    GameState::Menu => game.start_game(),
                    GameState::GameOver { .. } => game.reset_game(),
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

async fn create_render_data(window: &Window) -> RenderData {
    let size = window.inner_size();

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        dx12_shader_compiler: Default::default(),
    });

    let surface = unsafe { instance.create_surface(window).unwrap() };

    let adapter = instance.request_adapter(
        &wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        },
    ).await.unwrap();

    let (device, queue) = adapter.request_device(
        &wgpu::DeviceDescriptor {
            features: wgpu::Features::empty(),
            limits: wgpu::Limits::default(),
            label: None,
        },
        None,
    ).await.unwrap();

    let surface_caps = surface.get_capabilities(&adapter);
    let surface_format = surface_caps.formats.iter()
        .copied()
        .find(|f| f.is_srgb())
        .unwrap_or(surface_caps.formats[0]);

    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Fifo,
        alpha_mode: wgpu::CompositeAlphaMode::Auto,
        view_formats: vec![],
    };
    surface.configure(&device, &config);

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
    });

    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&render_pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[Vertex::desc()],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[Some(wgpu::ColorTargetState {
                format: config.format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
    });

    // Create initial vertex buffer (will be updated each frame)
    let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Vertex Buffer"),
        size: 1024 * 64, // Enough for many vertices
        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    RenderData {
        surface,
        device,
        queue,
        config,
        render_pipeline,
        vertex_buffer,
        num_vertices: 0,
    }
}

fn update_render_data(render_data: &mut RenderData, game: &PongGame) {
    let mut vertices = Vec::new();

    match game.game_state {
        GameState::Menu => {
            // Draw menu
            draw_text(&mut vertices, "PONG", -0.3, 0.3, 0.1, [1.0, 1.0, 1.0]);
            draw_text(&mut vertices, "Press SPACE to Start", -0.4, 0.0, 0.05, [0.8, 0.8, 0.8]);
            draw_text(&mut vertices, "ESC to Pause", -0.3, -0.1, 0.04, [0.6, 0.6, 0.6]);
        }
        GameState::Playing | GameState::Paused => {
            // Draw game objects
            draw_game_objects(&mut vertices, &game.world);

            // Draw score
            draw_score(&mut vertices, game.score.0, game.score.1);

            if let GameState::Paused = game.game_state {
                draw_text(&mut vertices, "PAUSED", -0.2, 0.0, 0.08, [1.0, 1.0, 0.0]);
                draw_text(&mut vertices, "Press ESC to Resume", -0.4, -0.1, 0.04, [0.8, 0.8, 0.8]);
            }
        }
        GameState::GameOver { ref winner } => {
            draw_text(&mut vertices, "GAME OVER", -0.3, 0.2, 0.08, [1.0, 0.0, 0.0]);
            draw_text(&mut vertices, &format!("{} Wins!", winner), -0.3, 0.0, 0.06, [1.0, 1.0, 1.0]);
            draw_text(&mut vertices, "Press SPACE to Menu", -0.4, -0.2, 0.04, [0.8, 0.8, 0.8]);
            draw_score(&mut vertices, game.score.0, game.score.1);
        }
    }

    // Update vertex buffer
    render_data.num_vertices = vertices.len() as u32;
    render_data.queue.write_buffer(
        &render_data.vertex_buffer,
        0,
        bytemuck::cast_slice(&vertices),
    );
}

fn draw_game_objects(vertices: &mut Vec<Vertex>, world: &World) {
    let positions = world.read_storage::<Position>();
    let renderables = world.read_storage::<Renderable>();
    let paddles = world.read_storage::<Paddle>();
    let balls = world.read_storage::<Ball>();

    // Draw paddles
    for (pos, _, paddle) in (&positions, &renderables, &paddles).join() {
        let color = if paddle.player_controlled {
            [0.0, 0.8, 0.0] // Green for player
        } else {
            [0.8, 0.0, 0.0] // Red for AI
        };
        draw_rectangle(vertices, pos.x, pos.y, PADDLE_WIDTH, PADDLE_HEIGHT, color);
    }

    // Draw ball
    for (pos, _, _) in (&positions, &renderables, &balls).join() {
        draw_rectangle(vertices, pos.x, pos.y, BALL_SIZE, BALL_SIZE, [1.0, 1.0, 1.0]);
    }

    // Draw center line
    for i in 0..20 {
        let y = -WINDOW_HEIGHT as f32 / 2.0 + i as f32 * 60.0;
        draw_rectangle(vertices, -2.0, y, 4.0, 20.0, [0.5, 0.5, 0.5]);
    }
}

fn draw_rectangle(vertices: &mut Vec<Vertex>, x: f32, y: f32, width: f32, height: f32, color: [f32; 3]) {
    let x = (x - WINDOW_WIDTH as f32 / 2.0) / (WINDOW_WIDTH as f32 / 2.0);
    let y = (y - WINDOW_HEIGHT as f32 / 2.0) / (WINDOW_HEIGHT as f32 / 2.0);
    let w = width / (WINDOW_WIDTH as f32 / 2.0);
    let h = height / (WINDOW_HEIGHT as f32 / 2.0);

    vertices.extend_from_slice(&[
        Vertex { position: [x, y], color },
        Vertex { position: [x + w, y], color },
        Vertex { position: [x + w, y + h], color },
        Vertex { position: [x, y], color },
        Vertex { position: [x + w, y + h], color },
        Vertex { position: [x, y + h], color },
    ]);
}

fn draw_text(vertices: &mut Vec<Vertex>, text: &str, x: f32, y: f32, size: f32, color: [f32; 3]) {
    // Simple text rendering (just draw rectangles for each character)
    let mut current_x = x;
    for ch in text.chars() {
        if ch != ' ' {
            draw_rectangle(vertices, current_x * WINDOW_WIDTH as f32, y * WINDOW_HEIGHT as f32, size * 20.0, size * 30.0, color);
        }
        current_x += size * 1.2;
    }
}

fn draw_score(vertices: &mut Vec<Vertex>, player_score: u32, ai_score: u32) {
    // Draw player score (left side)
    draw_text(vertices, &player_score.to_string(), -0.8, 0.8, 0.1, [0.0, 0.8, 0.0]);

    // Draw AI score (right side)
    draw_text(vertices, &ai_score.to_string(), 0.7, 0.8, 0.1, [0.8, 0.0, 0.0]);
}

fn render(render_data: &RenderData, game: &PongGame) -> Result<(), wgpu::SurfaceError> {
    let output = render_data.surface.get_current_texture()?;
    let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

    let mut encoder = render_data.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Render Encoder"),
    });

    {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.1,
                        b: 0.1,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });

        render_pass.set_pipeline(&render_data.render_pipeline);
        render_pass.set_vertex_buffer(0, render_data.vertex_buffer.slice(..));
        render_pass.draw(0..render_data.num_vertices, 0..1);
    }

    render_data.queue.submit(std::iter::once(encoder.finish()));
    output.present();

    Ok(())
}

// Reuse the existing Pong systems and entities from the console version
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
        .with(Score { player_score: 0, ai_score: 0 })
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

// Game-specific components (reuse from console version)
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

// Game systems (simplified versions for graphics demo)
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
            for (paddle_pos, _, _) in (&positions, &velocities, &paddles).join() {
                if check_paddle_ball_collision(ball_pos, paddle_pos) {
                    if let Some(vel) = velocities.get_mut(ball_entity) {
                        vel.x = -vel.x;
                        let paddle_center = paddle_pos.y + PADDLE_HEIGHT / 2.0;
                        let hit_pos = ball_pos.y + BALL_SIZE / 2.0;
                        let spin_factor = (hit_pos - paddle_center) / (PADDLE_HEIGHT / 2.0);
                        vel.y += spin_factor * 100.0;

                        let speed = (vel.x * vel.x + vel.y * vel.y).sqrt();
                        if speed > BALL_SPEED * 1.5 {
                            vel.x = vel.x / speed * BALL_SPEED * 1.2;
                            vel.y = vel.y / speed * BALL_SPEED * 1.2;
                        }
                    }
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