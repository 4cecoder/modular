//! Particle Demo
//!
//! This demo showcases the particle system by emitting particles on mouse clicks.

use modular_game_engine::*;
use std::time::Instant;
use winit::{event_loop::{ControlFlow, EventLoop}, window::WindowBuilder};

// Game constants
const WINDOW_WIDTH: usize = 800;
const WINDOW_HEIGHT: usize = 600;

// Particle system for visual effects (copied from pong.rs)
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

fn main() {
    println!("🎮 Particle Demo");
    println!("=====================");
    println!("Click anywhere to emit particles!");

    let event_loop = EventLoop::new();
        let _window = WindowBuilder::new()
        .with_title("Particle Demo - Modular Game Engine")
        .with_inner_size(winit::dpi::PhysicalSize::new(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32))
        .build(&event_loop)
        .unwrap();

    let mut render_context = renderer_2d::RenderContext::new(window::WindowConfig {
        title: "Particle Demo - Modular Game Engine".to_string(),
        width: WINDOW_WIDTH,
        height: WINDOW_HEIGHT,
        resizable: false,
        vsync: true,
    }).unwrap();

    let mut input_manager = input_window::WindowInputManager::new();
    let mut particle_system = ParticleSystem::new();

    let mut last_update = Instant::now();

        event_loop.run(move |_event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        let now = Instant::now();
        let delta_time = now.duration_since(last_update).as_secs_f32();
        last_update = now;

        input_manager.update(render_context.window.window_ref());
        let input_state = input_manager.state();

        // Emit particles on mouse click
        if input_state.is_mouse_button_pressed(input_window::MouseButton::Left) {
            particle_system.emit(
                input_state.mouse_position.0 as f32,
                input_state.mouse_position.1 as f32,
                50,
                renderer_2d::Color::rgb(255, rand::random::<u8>(), 0),
            );
        }

        // Update particles
        particle_system.update(delta_time);

        // Render
        render_context.renderer.clear(renderer_2d::Color::rgb(20, 20, 30));
        particle_system.render(&mut render_context.renderer);

        render_context.present().unwrap();
        render_context.update();

        if render_context.should_close() {
            *control_flow = ControlFlow::Exit;
        }
    });
}
