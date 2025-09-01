//! Example: Using the Extracted Systems
//!
//! This example demonstrates how to use all the systems extracted from the Pong game
//! in a modular, reusable way. Each system is completely independent and can be
//! used in any game.

use modular_game_engine::*;
use difficulty::{DifficultySystem, DifficultyLevel, DifficultyValue};
use particles::{ParticleSystem, ParticleEmitter, ParticleEmitterConfig};
use menu::{MenuSystem, MenuItem, menu_items, MenuAction};
use visual_effects::{VisualEffectsSystem, ScreenShake, ColorTransition, PulseEffect};
use enhanced_ai::{AISystem, PongPaddleAI, AIDifficulty};
use scoring::{ScoringSystem, ScoreType, WinCondition, presets as scoring_presets};
use trail_system::{TrailSystem, presets as trail_presets};

fn main() {
    println!("🎮 Modular Game Engine - Extracted Systems Demo");
    println!("==============================================");

    // 1. DIFFICULTY SYSTEM - Generic difficulty management
    println!("\n1. Difficulty System:");
    let mut difficulty_system = DifficultySystem::with_pong_defaults();

    // Register custom aspects for a space shooter
    difficulty_system.register_aspect("enemy_speed", "Speed of enemy ships");
    difficulty_system.register_aspect("player_health", "Starting player health");
    difficulty_system.register_aspect("powerup_frequency", "How often powerups spawn");

    // Set to hard difficulty
    difficulty_system.set_difficulty(DifficultyLevel::Hard);
    println!("  AI Speed Multiplier: {:.2}", difficulty_system.ai_speed_multiplier());
    println!("  Ball Speed Multiplier: {:.2}", difficulty_system.ball_speed_multiplier());
    println!("  Max Score: {}", difficulty_system.max_score());

    // 2. PARTICLE SYSTEM - Visual effects
    println!("\n2. Particle System:");
    let mut particle_system = ParticleSystem::new();

    // Create explosion effect
    let explosion_id = particle_system.create_explosion(Vec2::new(100.0, 100.0), 1.5);
    println!("  Created explosion with {} particles", particle_system.total_particle_count());

    // Create spark effect
    let spark_id = particle_system.create_sparks(Vec2::new(200.0, 200.0), Vec2::new(0.0, -1.0));
    println!("  Created spark effect");

    // Update particles
    particle_system.update(0.016); // ~60 FPS
    println!("  After update: {} particles remaining", particle_system.total_particle_count());

    // 3. MENU SYSTEM - UI and navigation
    println!("\n3. Menu System:");
    let mut menu_system = MenuSystem::create_main_menu();

    // Add custom menu items
    menu_system.add_item(menu_items::button(
        "custom_game",
        "Custom Game",
        MenuAction::Custom("start_custom".to_string())
    ));

    menu_system.add_item(menu_items::toggle(
        "sound",
        "Sound Effects",
        true,
        MenuAction::ToggleSetting("sound_enabled".to_string())
    ));

    println!("  Menu has {} items", menu_system.items.len());
    println!("  First item: {}", menu_system.items[0].get_text());

    // 4. VISUAL EFFECTS SYSTEM - Screen effects
    println!("\n4. Visual Effects System:");
    let mut visual_system = VisualEffectsSystem::new();

    // Add screen shake
    visual_system.shake_screen(5.0, 0.5, 10.0);
    println!("  Added screen shake effect");

    // Add color transition
    let transition_id = visual_system.add_color_transition(
        visual_effects::effects::warning_flash()
    );
    println!("  Added warning flash transition");

    // Add UI pulse effect
    let pulse_id = visual_system.add_pulse(
        visual_system.create_ui_pulse(1.0)
    );
    println!("  Added UI pulse effect");

    // 5. ENHANCED AI SYSTEM - Smart AI behaviors
    println!("\n5. Enhanced AI System:");
    let mut ai_system = AISystem::new();

    // Register Pong AI
    ai_system.register_pong_ai("pong_paddle", 300.0, AIDifficulty::Normal);
    println!("  Registered Pong AI");

    // Register chase AI
    ai_system.register_chase_ai("enemy_ship", 150.0, 30.0, AIDifficulty::Hard);
    println!("  Registered chase AI");

    // Update AI context
    ai_system.update_context(
        "pong_paddle",
        Vec2::new(400.0, 300.0),
        Vec2::new(0.0, 0.0),
        Some(enhanced_ai::AITarget::new(
            Vec2::new(350.0, 250.0), // Ball position
            Vec2::new(100.0, 50.0),  // Ball velocity
            Vec2::new(400.0, 300.0)  // AI position
        ))
    );

    // Get AI decision
    if let Some(decision) = ai_system.get_decision("pong_paddle") {
        println!("  AI Decision: Move velocity ({:.1}, {:.1})",
                decision.desired_velocity.x, decision.desired_velocity.y);
    }

    // 6. SCORING SYSTEM - Game statistics and achievements
    println!("\n6. Scoring System:");
    let mut scoring_system = scoring_presets::pong_scoring(5);

    // Add some scores
    scoring_system.add_score("player1", ScoreType::Points, 3);
    scoring_system.add_score("player2", ScoreType::Points, 2);
    scoring_system.add_score("player1", ScoreType::Points, 2);

    println!("  Player 1 score: {}", scoring_system.get_score("player1", &ScoreType::Points));
    println!("  Player 2 score: {}", scoring_system.get_score("player2", &ScoreType::Points));

    // Check win conditions
    match scoring_system.check_win_conditions() {
        scoring::GameResult::Win { winner, reason } => {
            println!("  Winner: {} ({})", winner, reason);
        }
        _ => println!("  Game ongoing")
    }

    // 7. TRAIL SYSTEM - Dynamic visual trails
    println!("\n7. Trail System:");
    let mut trail_system = TrailSystem::new();

    // Create ball trail
    trail_system.create_trail_with_config("ball", trail_presets::pong_ball_trail());
    println!("  Created ball trail");

    // Create spaceship trail
    trail_system.create_trail_with_config("ship", trail_presets::spaceship_trail());
    println!("  Created spaceship trail");

    // Update trails
    trail_system.update_trail("ball", 0.016, Vec2::new(100.0, 100.0), Vec2::new(200.0, 0.0));
    trail_system.update_trail("ball", 0.016, Vec2::new(120.0, 100.0), Vec2::new(200.0, 0.0));

    println!("  Ball trail segments: {}", trail_system.get_trail("ball").unwrap().segment_count());

    // 8. SYSTEM INTEGRATION EXAMPLE
    println!("\n8. System Integration Example:");
    println!("  Creating a complete game setup...");

    // Create all systems
    let mut game_difficulty = DifficultySystem::with_pong_defaults();
    let mut game_particles = ParticleSystem::new();
    let mut game_menu = MenuSystem::create_difficulty_menu();
    let mut game_visuals = VisualEffectsSystem::new();
    let mut game_ai = AISystem::new();
    let mut game_scoring = scoring_presets::pong_scoring(5);
    let mut game_trails = TrailSystem::new();

    // Configure for a specific game mode
    game_difficulty.set_difficulty(DifficultyLevel::Hard);
    game_ai.register_pong_ai("player_ai", 350.0, AIDifficulty::from_multiplier(
        game_difficulty.ai_speed_multiplier()
    ));

    // Add visual flair
    game_particles.create_explosion(Vec2::new(400.0, 300.0), 2.0);
    game_trails.create_trail_with_config("game_ball", trail_presets::pong_ball_trail());
    game_visuals.shake_screen(3.0, 0.3, 8.0);

    println!("  ✓ Difficulty: {:?}", game_difficulty.get_current_level());
    println!("  ✓ AI Agents: {}", game_ai.get_agent_ids().len());
    println!("  ✓ Particle Effects: {}", game_particles.total_particle_count());
    println!("  ✓ Menu Items: {}", game_menu.items.len());
    println!("  ✓ Visual Effects: {}", game_visuals.active_effects_count());
    println!("  ✓ Scoring Rules: {}", game_scoring.win_conditions.len());
    println!("  ✓ Trail Effects: {}", game_trails.get_trail_ids().len());

    println!("\n🎉 All systems successfully integrated!");
    println!("   Each system is completely modular and reusable.");
    println!("   You can mix and match them for any game type.");
}