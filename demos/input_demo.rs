//! Input Demo
//!
//! This demo showcases the input system by displaying the current state of keyboard keys and mouse position.

use modular_game_engine::*;
use std::{thread, time::Duration, time::Instant};

// Game constants
const WINDOW_WIDTH: usize = 800;
const WINDOW_HEIGHT: usize = 600;

fn main() {
    println!("🎮 Input Demo");
    println!("=====================");
    println!("Press W, A, S, D, Space, ESC and move the mouse to see input states.");

    // Note: we intentionally avoid creating a separate winit event loop here.
    // The engine's window system uses `minifb` for this demo. Creating both
    // a `winit` event loop and a `minifb` window can cause platform-specific
    // issues (especially on macOS). Use a single windowing backend instead.

    let mut render_context = renderer_2d::RenderContext::new(window::WindowConfig {
        title: "Input Demo - Modular Game Engine".to_string(),
        width: WINDOW_WIDTH,
        height: WINDOW_HEIGHT,
        resizable: false,
        vsync: true,
    })
    .unwrap();

    let mut input_manager = input_window::WindowInputManager::new();

    // Diagnostic: print whether the underlying window reports that it's open
    println!(
        "Window open after creation: {}",
        render_context.window.window_ref().is_open()
    );

    let mut last_update = Instant::now();

    // Simple main loop driven by the minifb window. This avoids mixing
    // different windowing/event loop systems and makes the demo portable
    // across platforms where running multiple event loops on the main thread
    // can fail to display a window.
    let mut frame_counter: u32 = 0;

    while !render_context.should_close() {
        let now = Instant::now();
        let _delta_time = now.duration_since(last_update).as_secs_f32();
        last_update = now;

        // Update the window first so minifb events are processed before we
        // sample input state.
        render_context.update();

        input_manager.update(render_context.window.window_ref());
        let input_state = input_manager.state();

        // Render
        render_context
            .renderer
            .clear(renderer_2d::Color::rgb(20, 20, 30));

        let mut y_offset = 50;
        let x_offset = 50;
        let line_height = 30;

        render_context.renderer.draw_text(
            &format!(
                "Mouse Position: ({}, {})",
                input_state.mouse_position.0, input_state.mouse_position.1
            ),
            x_offset,
            y_offset,
            renderer_2d::Color::WHITE,
            1,
        );
        y_offset += line_height;

        render_context.renderer.draw_text(
            &format!(
                "Mouse Left Button: {}",
                input_state.is_mouse_button_pressed(input_window::MouseButton::Left)
            ),
            x_offset,
            y_offset,
            renderer_2d::Color::WHITE,
            1,
        );
        y_offset += line_height;

        render_context.renderer.draw_text(
            &format!(
                "Mouse Right Button: {}",
                input_state.is_mouse_button_pressed(input_window::MouseButton::Right)
            ),
            x_offset,
            y_offset,
            renderer_2d::Color::WHITE,
            1,
        );
        y_offset += line_height;

        render_context.renderer.draw_text(
            &format!("Key W: {}", input_state.is_key_pressed(minifb::Key::W)),
            x_offset,
            y_offset,
            renderer_2d::Color::WHITE,
            1,
        );
        y_offset += line_height;

        render_context.renderer.draw_text(
            &format!("Key A: {}", input_state.is_key_pressed(minifb::Key::A)),
            x_offset,
            y_offset,
            renderer_2d::Color::WHITE,
            1,
        );
        y_offset += line_height;

        render_context.renderer.draw_text(
            &format!("Key S: {}", input_state.is_key_pressed(minifb::Key::S)),
            x_offset,
            y_offset,
            renderer_2d::Color::WHITE,
            1,
        );
        y_offset += line_height;

        render_context.renderer.draw_text(
            &format!("Key D: {}", input_state.is_key_pressed(minifb::Key::D)),
            x_offset,
            y_offset,
            renderer_2d::Color::WHITE,
            1,
        );
        y_offset += line_height;

        render_context.renderer.draw_text(
            &format!(
                "Key Space: {}",
                input_state.is_key_pressed(minifb::Key::Space)
            ),
            x_offset,
            y_offset,
            renderer_2d::Color::WHITE,
            1,
        );
        y_offset += line_height;

        render_context.renderer.draw_text(
            &format!(
                "Key ESC: {}",
                input_state.is_key_pressed(minifb::Key::Escape)
            ),
            x_offset,
            y_offset,
            renderer_2d::Color::WHITE,
            1,
        );
        y_offset += line_height;

        if let Err(e) = render_context.present() {
            eprintln!("Error presenting frame: {}", e);
            break;
        }

        // Diagnostic: print window open state each frame (throttled)
        frame_counter += 1;
        if frame_counter % 60 == 0 {
            println!(
                "Frame {} - window.is_open: {}",
                frame_counter,
                render_context.window.window_ref().is_open()
            );
            println!(
                "Mouse pos: {:?}, Left: {}, Right: {}",
                input_state.mouse_position,
                input_state.is_mouse_button_pressed(input_window::MouseButton::Left),
                input_state.is_mouse_button_pressed(input_window::MouseButton::Right)
            );
        }

        // Basic quit check from input manager or window manager
        if input_manager.should_quit() || render_context.should_close() {
            break;
        }

        // Sleep a short time to avoid maxing CPU (approx ~60 FPS)
        thread::sleep(Duration::from_millis(16));
    }
}
