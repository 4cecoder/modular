//! UI Demo
//!
//! This demo showcases basic UI elements and interaction.

use modular_game_engine::*;
use minifb::{Window, WindowOptions};
use std::time::Instant;

// Game constants
const WINDOW_WIDTH: usize = 800;
const WINDOW_HEIGHT: usize = 600;

fn main() {
    println!("🎮 UI Demo");
    println!("=====================\n");
    println!("Click the button!");
    println!("Press ESC or Q to exit.");

    let mut window = Window::new(
        "UI Demo - Modular Game Engine",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut renderer = renderer_2d::Renderer2D::new(WINDOW_WIDTH, WINDOW_HEIGHT);

    // Load a custom font
    renderer.load_font("game_font", "assets/fonts/DejaVuSans.ttf").unwrap();
    renderer.set_default_font("game_font");

    let mut input_manager = input_window::WindowInputManager::new();

    let mut button_clicks = 0;

    let mut last_update = Instant::now();

    // Main game loop
    while window.is_open() && !input_manager.should_quit() {
        let now = Instant::now();
        let _delta_time = now.duration_since(last_update).as_secs_f32();
        last_update = now;

        // Update input manager
        input_manager.update(&window);
        let input_state = input_manager.state();

        // UI Logic
        let button_x = (WINDOW_WIDTH / 2 - 100) as i32;
        let button_y = (WINDOW_HEIGHT / 2 - 25) as i32;
        let button_width = 200;
        let button_height = 50;

        let mouse_x = input_state.mouse_pos().0;
        let mouse_y = input_state.mouse_pos().1;

        let mouse_over_button = mouse_x >= button_x
            && mouse_x <= button_x + button_width
            && mouse_y >= button_y
            && mouse_y <= button_y + button_height;

                        if mouse_over_button && input_state.is_mouse_button_just_pressed(input_window::MouseButton::Left) {
            button_clicks += 1;
        }

        // Render
        renderer.clear(renderer_2d::Color::rgb(20, 20, 30));

        // Draw button
        renderer.draw_rect(
            button_x,
            button_y,
            button_width,
            button_height,
            if mouse_over_button { renderer_2d::Color::rgb(100, 100, 200) } else { renderer_2d::Color::rgb(50, 50, 150) },
        );

        renderer.draw_text_centered(
            "Click Me!",
            WINDOW_WIDTH / 2,
            (WINDOW_HEIGHT / 2 - 5) as usize,
            renderer_2d::Color::WHITE,
            2,
        );

        // Draw click counter
        renderer.draw_text_centered(
            &format!("Clicks: {}", button_clicks),
            WINDOW_WIDTH / 2,
            (WINDOW_HEIGHT / 2 + 50) as usize,
            renderer_2d::Color::WHITE,
            2,
        );

        // Update minifb window
        window
            .update_with_buffer(renderer.buffer(), WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap();

        std::thread::sleep(std::time::Duration::from_millis(16));
    }

    println!("\nUI Demo closed. Thanks for playing!");
}
