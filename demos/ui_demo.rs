//! UI Demo
//!
//! This demo showcases basic UI elements and interaction using the new robust UI system.

use minifb::{Window, WindowOptions};
use modular_game_engine::*;
use std::cell::RefCell; // For mutable interior of button_clicks
use std::rc::Rc; // For shared ownership of button_clicks
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
    renderer
        .load_font("game_font", "assets/fonts/DejaVuSans.ttf")
        .unwrap();
    renderer.set_default_font("game_font");

    let mut input_manager = input_window::WindowInputManager::new();
    let mut ui_manager = ui::UIManager::new();

    // Shared state for button clicks
    let button_clicks = Rc::new(RefCell::new(0));

    // Create a button
    let click_button_id = "click_me_button";
    let clicks_label_id = "clicks_label";

    let button_x = (WINDOW_WIDTH / 2 - 100) as f32;
    let button_y = (WINDOW_HEIGHT / 2 - 25) as f32;
    let button_width = 200.0;
    let button_height = 50.0;

    let clicks_label_pos = Vec2::new(
        (WINDOW_WIDTH / 2 - 50) as f32,
        (WINDOW_HEIGHT / 2 + 50) as f32,
    );

    // Toggle and Slider demo IDs and positions
    let toggle_id = "music_toggle";
    let toggle_label_pos = Vec2::new(20.0, 20.0);

    let slider_id = "volume_slider";
    let slider_pos = Vec2::new(20.0, 60.0);
    let slider_size = Vec2::new(200.0, 16.0);

    // Clone Rc for the callback
    let button_clicks_clone = Rc::clone(&button_clicks);
    let clicks_label_id_clone = clicks_label_id.to_string(); // Clone for the callback

    let button = ui::Button::new(
        click_button_id,
        "Click Me!",
        Vec2::new(button_x, button_y),
        Vec2::new(button_width, button_height),
    )
    .on_click(Box::new(move || {
        *button_clicks_clone.borrow_mut() += 1;
        println!(
            "Button clicked! Total clicks: {}",
            button_clicks_clone.borrow()
        );
        // Update the label text directly from here (requires mutable access to UIManager)
        // This pattern is a bit tricky with current UIManager design,
        // typically you'd emit an event and a system would update the label.
        // For simplicity in demo, we'll update it in the main loop.
    }));
    ui_manager.add_widget(ui::Widget::Button(button));

    // Create a label to display clicks
    let clicks_label = ui::Label::new(
        clicks_label_id,
        &format!("Clicks: {}", button_clicks.borrow()),
        clicks_label_pos,
    );
    ui_manager.add_widget(ui::Widget::Label(clicks_label));

    // Add a toggle for music on/off
    let music_on = true;
    let music_toggle_label = "Music";
    let toggle = ui::Toggle::new(toggle_id, music_toggle_label, toggle_label_pos, music_on)
        .on_change(Box::new(move |on: bool| {
            println!("Music toggled: {}", on);
        }));
    ui_manager.add_widget(ui::Widget::Toggle(toggle));

    // Add a volume slider
    let slider = ui::Slider::new(slider_id, slider_pos, slider_size, 0.0, 1.0, 0.8).on_change(
        Box::new(move |v: f32| {
            println!("Volume changed: {:.2}", v);
        }),
    );
    ui_manager.add_widget(ui::Widget::Slider(slider));

    let mut last_update = Instant::now();

    // Main game loop
    while window.is_open() && !input_manager.should_quit() {
        let now = Instant::now();
        let delta_time = now.duration_since(last_update).as_secs_f32();
        last_update = now;

        // Update input manager
        input_manager.update(&window);
        let input_state = input_manager.state();

        // Handle UI events
        let ui_events = ui_manager.handle_input(input_state);
        for event in ui_events {
            match event {
                ui::UiEvent::Click(id) => {
                    if id == click_button_id {
                        // The button_clicks is already updated by the callback
                        // Now update the label text using the UI manager helper
                        if let Some(label) = ui_manager.get_label_mut(&clicks_label_id_clone) {
                            label.set_text(&format!("Clicks: {}", button_clicks.borrow()));
                        }
                    }
                }
            }
        }

        // Update UI manager
        ui_manager.update(delta_time);

        // Render
        renderer.clear(renderer_2d::Color::rgb(20, 20, 30));
        ui_manager.render(&mut renderer);

        // Update minifb window
        window
            .update_with_buffer(renderer.buffer(), WINDOW_WIDTH, WINDOW_HEIGHT)
            .unwrap();

        std::thread::sleep(std::time::Duration::from_millis(16));
    }

    println!("\nUI Demo closed. Thanks for playing!");
}
