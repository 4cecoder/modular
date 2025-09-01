//! Visual Effects System
//!
//! Enhances the renderer_2d with advanced visual effects like glow,
//! trails, screen shake, and post-processing effects.

use crate::Vec2;
use std::collections::VecDeque;

/// Glow effect configuration
#[derive(Debug, Clone)]
pub struct GlowEffect {
    pub color: [f32; 4],
    pub intensity: f32,
    pub size: f32,
    pub layers: u32,
}

impl Default for GlowEffect {
    fn default() -> Self {
        Self {
            color: [1.0, 1.0, 1.0, 1.0],
            intensity: 1.0,
            size: 2.0,
            layers: 3,
        }
    }
}

/// Trail point for trail effects
#[derive(Debug, Clone)]
pub struct TrailPoint {
    pub position: Vec2,
    pub color: [f32; 4],
    pub size: f32,
    pub life: f32,
    pub max_life: f32,
}

impl TrailPoint {
    pub fn new(position: Vec2, color: [f32; 4], size: f32, life: f32) -> Self {
        Self {
            position,
            color,
            size,
            life,
            max_life: life,
        }
    }

    pub fn update(&mut self, delta_time: f32) -> bool {
        self.life -= delta_time;
        self.life > 0.0
    }

    pub fn alpha(&self) -> f32 {
        self.life / self.max_life
    }
}

/// Trail effect configuration
#[derive(Debug, Clone)]
pub struct TrailEffect {
    pub points: VecDeque<TrailPoint>,
    pub max_points: usize,
    pub fade_speed: f32,
    pub min_alpha: f32,
}

impl TrailEffect {
    pub fn new(max_points: usize) -> Self {
        Self {
            points: VecDeque::new(),
            max_points,
            fade_speed: 2.0,
            min_alpha: 0.0,
        }
    }

    pub fn add_point(&mut self, position: Vec2, color: [f32; 4], size: f32, life: f32) {
        let point = TrailPoint::new(position, color, size, life);
        self.points.push_front(point);

        // Remove old points if we exceed max
        while self.points.len() > self.max_points {
            self.points.pop_back();
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.points.retain_mut(|point| point.update(delta_time));
    }

    pub fn clear(&mut self) {
        self.points.clear();
    }
}

/// Screen shake effect
#[derive(Debug, Clone)]
pub struct ScreenShake {
    pub intensity: f32,
    pub duration: f32,
    pub frequency: f32,
    pub time: f32,
    pub active: bool,
}

impl ScreenShake {
    pub fn new(intensity: f32, duration: f32, frequency: f32) -> Self {
        Self {
            intensity,
            duration,
            frequency,
            time: 0.0,
            active: false,
        }
    }

    pub fn update(&mut self, delta_time: f32) -> Vec2 {
        if !self.active {
            return Vec2::new(0.0, 0.0);
        }

        self.time += delta_time;

        if self.time >= self.duration {
            self.active = false;
            return Vec2::new(0.0, 0.0);
        }

        let progress = self.time / self.duration;
        let current_intensity = self.intensity * (1.0 - progress);

        let offset_x = (self.time * self.frequency).sin() * current_intensity;
        let offset_y = (self.time * self.frequency * 1.5).cos() * current_intensity;

        Vec2::new(offset_x, offset_y)
    }

    pub fn trigger(&mut self, intensity: f32, duration: f32) {
        self.intensity = intensity;
        self.duration = duration;
        self.time = 0.0;
        self.active = true;
    }

    pub fn stop(&mut self) {
        self.active = false;
    }
}

/// Color transition effect
#[derive(Debug, Clone)]
pub struct ColorTransition {
    pub start_color: [f32; 4],
    pub end_color: [f32; 4],
    pub duration: f32,
    pub time: f32,
    pub active: bool,
    pub loop_effect: bool,
}

impl ColorTransition {
    pub fn new(start_color: [f32; 4], end_color: [f32; 4], duration: f32) -> Self {
        Self {
            start_color,
            end_color,
            duration,
            time: 0.0,
            active: false,
            loop_effect: false,
        }
    }

    pub fn start(&mut self) {
        self.time = 0.0;
        self.active = true;
    }

    pub fn update(&mut self, delta_time: f32) -> [f32; 4] {
        if !self.active {
            return self.end_color;
        }

        self.time += delta_time;

        let t = if self.loop_effect {
            (self.time / self.duration).fract()
        } else {
            (self.time / self.duration).min(1.0)
        };

        if !self.loop_effect && self.time >= self.duration {
            self.active = false;
            return self.end_color;
        }

        // Interpolate between start and end colors
        let mut result = [0.0; 4];
        for i in 0..4 {
            result[i] = self.start_color[i] + (self.end_color[i] - self.start_color[i]) * t;
        }

        result
    }

    pub fn set_loop(&mut self, loop_effect: bool) {
        self.loop_effect = loop_effect;
    }
}

/// Pulse effect for UI elements
#[derive(Debug, Clone)]
pub struct PulseEffect {
    pub base_scale: f32,
    pub amplitude: f32,
    pub frequency: f32,
    pub time: f32,
    pub active: bool,
}

impl PulseEffect {
    pub fn new(base_scale: f32, amplitude: f32, frequency: f32) -> Self {
        Self {
            base_scale,
            amplitude,
            frequency,
            time: 0.0,
            active: true,
        }
    }

    pub fn update(&mut self, delta_time: f32) -> f32 {
        if !self.active {
            return self.base_scale;
        }

        self.time += delta_time;
        self.base_scale + self.amplitude * (self.time * self.frequency).sin()
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }
}

/// Main visual effects system
pub struct VisualEffectsSystem {
    pub glow_effects: Vec<GlowEffect>,
    pub trail_effects: Vec<TrailEffect>,
    pub screen_shake: ScreenShake,
    pub color_transitions: Vec<ColorTransition>,
    pub pulse_effects: Vec<PulseEffect>,
    pub time: f32,
}

impl VisualEffectsSystem {
    pub fn new() -> Self {
        Self {
            glow_effects: Vec::new(),
            trail_effects: Vec::new(),
            screen_shake: ScreenShake::new(0.0, 0.0, 0.0),
            color_transitions: Vec::new(),
            pulse_effects: Vec::new(),
            time: 0.0,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.time += delta_time;

        // Update trail effects
        for trail in &mut self.trail_effects {
            trail.update(delta_time);
        }

        // Update pulse effects
        for pulse in &mut self.pulse_effects {
            pulse.update(delta_time);
        }

        // Remove completed effects
        self.trail_effects.retain(|trail| !trail.points.is_empty());
        self.color_transitions
            .retain(|transition| transition.active || transition.loop_effect);
    }

    /// Add a glow effect
    pub fn add_glow(&mut self, effect: GlowEffect) -> usize {
        self.glow_effects.push(effect);
        self.glow_effects.len() - 1
    }

    /// Add a trail effect
    pub fn add_trail(&mut self, effect: TrailEffect) -> usize {
        self.trail_effects.push(effect);
        self.trail_effects.len() - 1
    }

    /// Add a color transition
    pub fn add_color_transition(&mut self, transition: ColorTransition) -> usize {
        self.color_transitions.push(transition);
        self.color_transitions.len() - 1
    }

    /// Add a pulse effect
    pub fn add_pulse(&mut self, effect: PulseEffect) -> usize {
        self.pulse_effects.push(effect);
        self.pulse_effects.len() - 1
    }

    /// Trigger screen shake
    pub fn shake_screen(&mut self, intensity: f32, duration: f32, frequency: f32) {
        self.screen_shake.trigger(intensity, duration);
        self.screen_shake.frequency = frequency;
    }

    /// Get current screen shake offset
    pub fn get_screen_shake_offset(&mut self, delta_time: f32) -> Vec2 {
        self.screen_shake.update(delta_time)
    }

    /// Create a preset explosion glow effect
    pub fn create_explosion_glow(&mut self, intensity: f32) -> usize {
        let effect = GlowEffect {
            color: [1.0, 0.5, 0.0, 1.0], // Orange
            intensity: intensity * 2.0,
            size: 3.0 * intensity,
            layers: 4,
        };
        self.add_glow(effect)
    }

    /// Create a preset energy glow effect
    pub fn create_energy_glow(&mut self, intensity: f32) -> usize {
        let effect = GlowEffect {
            color: [0.0, 1.0, 1.0, 1.0], // Cyan
            intensity: intensity * 1.5,
            size: 2.0 * intensity,
            layers: 3,
        };
        self.add_glow(effect)
    }

    /// Create a preset ball trail
    pub fn create_ball_trail(&mut self, max_points: usize) -> usize {
        let mut trail = TrailEffect::new(max_points);
        trail.fade_speed = 3.0;
        trail.min_alpha = 0.1;
        self.add_trail(trail)
    }

    /// Add a point to a trail effect
    pub fn add_trail_point(
        &mut self,
        trail_index: usize,
        position: Vec2,
        color: [f32; 4],
        size: f32,
        life: f32,
    ) {
        if let Some(trail) = self.trail_effects.get_mut(trail_index) {
            trail.add_point(position, color, size, life);
        }
    }

    /// Create a preset damage flash transition
    pub fn create_damage_flash(&mut self) -> usize {
        let transition = ColorTransition::new(
            [1.0, 1.0, 1.0, 1.0], // Normal
            [1.0, 0.0, 0.0, 0.8], // Red flash
            0.2,                  // Quick flash
        );
        self.add_color_transition(transition)
    }

    /// Create a preset UI pulse effect
    pub fn create_ui_pulse(&mut self, base_scale: f32) -> usize {
        let pulse = PulseEffect::new(base_scale, 0.1, 4.0);
        self.add_pulse(pulse)
    }

    /// Clear all effects
    pub fn clear(&mut self) {
        self.glow_effects.clear();
        self.trail_effects.clear();
        self.color_transitions.clear();
        self.pulse_effects.clear();
        self.screen_shake.stop();
    }

    /// Get the number of active effects
    pub fn active_effects_count(&self) -> usize {
        self.glow_effects.len()
            + self.trail_effects.len()
            + self.color_transitions.len()
            + self.pulse_effects.len()
            + if self.screen_shake.active { 1 } else { 0 }
    }
}

impl Default for VisualEffectsSystem {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper functions for common visual effects
pub mod effects {
    use super::*;

    /// Create a rainbow color transition
    pub fn rainbow_transition(duration: f32) -> ColorTransition {
        let mut transition = ColorTransition::new(
            [1.0, 0.0, 0.0, 1.0], // Red
            [1.0, 0.0, 1.0, 1.0], // Magenta
            duration,
        );
        transition.set_loop(true);
        transition
    }

    /// Create a breathing pulse effect
    pub fn breathing_pulse(base_scale: f32) -> PulseEffect {
        PulseEffect::new(base_scale, 0.05, 2.0)
    }

    /// Create a warning flash effect
    pub fn warning_flash() -> ColorTransition {
        ColorTransition::new(
            [1.0, 1.0, 1.0, 1.0], // Normal
            [1.0, 1.0, 0.0, 0.5], // Yellow warning
            0.5,
        )
    }

    /// Create a slow glow effect
    pub fn slow_glow(intensity: f32) -> GlowEffect {
        GlowEffect {
            color: [1.0, 1.0, 1.0, 0.8],
            intensity,
            size: 1.5,
            layers: 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trail_effect() {
        let mut trail = TrailEffect::new(10);
        trail.add_point(Vec2::new(0.0, 0.0), [1.0, 1.0, 1.0, 1.0], 5.0, 1.0);
        assert_eq!(trail.points.len(), 1);

        trail.update(0.5);
        assert_eq!(trail.points.len(), 1);
        assert_eq!(trail.points[0].life, 0.5);
    }

    #[test]
    fn test_screen_shake() {
        let mut shake = ScreenShake::new(10.0, 1.0, 5.0);
        let offset = shake.update(0.1);
        assert!(offset.x.abs() > 0.0 || offset.y.abs() > 0.0);
    }

    #[test]
    fn test_color_transition() {
        let mut transition = ColorTransition::new([0.0, 0.0, 0.0, 1.0], [1.0, 1.0, 1.0, 1.0], 1.0);
        transition.start();

        let color = transition.update(0.5);
        assert!(color[0] > 0.0 && color[0] < 1.0); // Should be interpolated
    }

    #[test]
    fn test_pulse_effect() {
        let mut pulse = PulseEffect::new(1.0, 0.2, 1.0);
        let scale = pulse.update(0.1);
        assert!(scale >= 0.8 && scale <= 1.2); // Should vary around base scale
    }

    #[test]
    fn test_visual_effects_system() {
        let mut system = VisualEffectsSystem::new();

        let trail_idx = system.create_ball_trail(20);
        system.add_trail_point(
            trail_idx,
            Vec2::new(10.0, 10.0),
            [1.0, 0.0, 0.0, 1.0],
            3.0,
            1.0,
        );

        system.update(0.1);
        assert_eq!(system.active_effects_count(), 1);

        system.clear();
        assert_eq!(system.active_effects_count(), 0);
    }
}
