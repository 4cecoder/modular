//! Audio Demo
//!
//! This demo showcases the audio system by playing a sound effect when a key is pressed.

use modular_game_engine::*;
use std::time::Instant;
use winit::{
        event::{ElementState, Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

// Game constants
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

fn main() {
    println!("🎮 Audio Demo");
    println!("=====================");
    println!("Press SPACE to play a sound!");
    println!("Press ESC to exit.");

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Audio Demo - Modular Game Engine")
        .with_inner_size(winit::dpi::PhysicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT))
        .build(&event_loop)
        .unwrap();

    // Initialize audio system
    let audio_manager = audio::AudioManager::new();

    // Load sound effect
        // Sound is loaded and played directly in play_sound for this simple demo

    let mut last_update = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        let now = Instant::now();
                let _delta_time = now.duration_since(last_update).as_secs_f32();
        last_update = now;

        match event {
            Event::WindowEvent { event, window_id } if window_id == window.id() => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput { input, .. } => {
                    if input.state == ElementState::Pressed {
                        if let Some(VirtualKeyCode::Space) = input.virtual_keycode {
                                                        audio_manager.play_sound("assets/audio/click.wav").unwrap();
                            println!("Playing click sound!");
                        }
                        if let Some(VirtualKeyCode::Escape) = input.virtual_keycode {
                            *control_flow = ControlFlow::Exit;
                        }
                    }
                }
                _ => {}
            },
            Event::MainEventsCleared => {
                // No game logic update needed for this simple demo
            }
            _ => {}
        }
    });
}
