//! Trail System
//!
//! A specialized system for creating dynamic trail effects behind moving objects.
//! Perfect for balls, projectiles, particles, and other fast-moving entities.

use crate::Vec2;
use std::collections::VecDeque;

/// Individual trail segment
#[derive(Debug, Clone)]
pub struct TrailSegment {
    pub position: Vec2,
    pub color: [f32; 4],
    pub size: f32,
    pub life: f32,
    pub max_life: f32,
}

impl TrailSegment {
    pub fn new(position: Vec2, color: [f32; 4], size: f32, life: f32) -> Self {
        Self {
            position,
            color,
            size,
            life,
            max_life: life,
        }
    }

    /// Update the segment's life
    pub fn update(&mut self, delta_time: f32) -> bool {
        self.life -= delta_time;
        self.life > 0.0
    }

    /// Get the normalized life (0.0 to 1.0)
    pub fn normalized_life(&self) -> f32 {
        self.life / self.max_life
    }

    /// Get the current alpha based on life
    pub fn alpha(&self) -> f32 {
        self.normalized_life()
    }
}

/// Trail configuration
#[derive(Debug, Clone)]
pub struct TrailConfig {
    /// Maximum number of segments in the trail
    pub max_segments: usize,
    /// Time between creating new segments
    pub segment_interval: f32,
    /// Life duration of each segment
    pub segment_life: f32,
    /// Base color of the trail
    pub base_color: [f32; 4],
    /// Base size of trail segments
    pub base_size: f32,
    /// Whether to fade the trail over time
    pub fade_enabled: bool,
    /// Whether to shrink segments over time
    pub shrink_enabled: bool,
    /// Minimum alpha for faded segments
    pub min_alpha: f32,
    /// Speed at which segments fade
    pub fade_speed: f32,
    /// Whether the trail follows velocity direction
    pub velocity_based: bool,
    /// Minimum distance between segments
    pub min_distance: f32,
}

impl Default for TrailConfig {
    fn default() -> Self {
        Self {
            max_segments: 20,
            segment_interval: 0.05, // 20 FPS
            segment_life: 1.0,
            base_color: [1.0, 1.0, 1.0, 1.0],
            base_size: 3.0,
            fade_enabled: true,
            shrink_enabled: true,
            min_alpha: 0.1,
            fade_speed: 2.0,
            velocity_based: false,
            min_distance: 5.0,
        }
    }
}

/// Individual trail
#[derive(Debug, Clone)]
pub struct Trail {
    pub config: TrailConfig,
    pub segments: VecDeque<TrailSegment>,
    pub last_position: Vec2,
    pub last_velocity: Vec2,
    pub time_since_last_segment: f32,
    pub enabled: bool,
}

impl Trail {
    /// Create a new trail with default configuration
    pub fn new() -> Self {
        Self {
            config: TrailConfig::default(),
            segments: VecDeque::new(),
            last_position: Vec2::new(0.0, 0.0),
            last_velocity: Vec2::new(0.0, 0.0),
            time_since_last_segment: 0.0,
            enabled: true,
        }
    }

    /// Create a trail with custom configuration
    pub fn with_config(config: TrailConfig) -> Self {
        Self {
            config,
            segments: VecDeque::new(),
            last_position: Vec2::new(0.0, 0.0),
            last_velocity: Vec2::new(0.0, 0.0),
            time_since_last_segment: 0.0,
            enabled: true,
        }
    }

    /// Update the trail with new position and velocity
    pub fn update(&mut self, delta_time: f32, position: Vec2, velocity: Vec2) {
        if !self.enabled {
            return;
        }

        // Update existing segments
        self.segments.retain_mut(|segment| segment.update(delta_time));

        // Check if we should create a new segment
        self.time_since_last_segment += delta_time;
        let distance_moved = (position - self.last_position).magnitude();

        if self.time_since_last_segment >= self.config.segment_interval &&
           distance_moved >= self.config.min_distance {

            self.add_segment(position, velocity);
            self.time_since_last_segment = 0.0;
            self.last_position = position;
            self.last_velocity = velocity;
        }
    }

    /// Add a new segment to the trail
    pub fn add_segment(&mut self, position: Vec2, velocity: Vec2) {
        let mut color = self.config.base_color;
        let mut size = self.config.base_size;

        // Modify color and size based on velocity if enabled
        if self.config.velocity_based {
            let speed = velocity.magnitude();
            let speed_factor = (speed / 500.0).min(1.0); // Normalize to max expected speed

            // Faster = more opaque/brighter
            color[3] = (color[3] * (0.5 + speed_factor * 0.5)).min(1.0);
            size = size * (0.8 + speed_factor * 0.4);
        }

        let segment = TrailSegment::new(position, color, size, self.config.segment_life);
        self.segments.push_front(segment);

        // Remove old segments if we exceed the maximum
        while self.segments.len() > self.config.max_segments {
            self.segments.pop_back();
        }
    }

    /// Clear all segments
    pub fn clear(&mut self) {
        self.segments.clear();
    }

    /// Set the trail enabled/disabled
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        if !enabled {
            self.clear();
        }
    }

    /// Get the number of active segments
    pub fn segment_count(&self) -> usize {
        self.segments.len()
    }

    /// Check if the trail has any segments
    pub fn has_segments(&self) -> bool {
        !self.segments.is_empty()
    }

    /// Get all segments for rendering
    pub fn get_segments(&self) -> &VecDeque<TrailSegment> {
        &self.segments
    }
}

/// Main trail system that manages multiple trails
pub struct TrailSystem {
    trails: std::collections::HashMap<String, Trail>,
}

impl TrailSystem {
    /// Create a new trail system
    pub fn new() -> Self {
        Self {
            trails: std::collections::HashMap::new(),
        }
    }

    /// Add a trail with the given ID
    pub fn add_trail(&mut self, id: &str, trail: Trail) {
        self.trails.insert(id.to_string(), trail);
    }

    /// Create and add a trail with default configuration
    pub fn create_trail(&mut self, id: &str) -> &mut Trail {
        let trail = Trail::new();
        self.trails.insert(id.to_string(), trail);
        self.trails.get_mut(id).unwrap()
    }

    /// Create and add a trail with custom configuration
    pub fn create_trail_with_config(&mut self, id: &str, config: TrailConfig) -> &mut Trail {
        let trail = Trail::with_config(config);
        self.trails.insert(id.to_string(), trail);
        self.trails.get_mut(id).unwrap()
    }

    /// Get a trail by ID
    pub fn get_trail(&self, id: &str) -> Option<&Trail> {
        self.trails.get(id)
    }

    /// Get a mutable trail by ID
    pub fn get_trail_mut(&mut self, id: &str) -> Option<&mut Trail> {
        self.trails.get_mut(id)
    }

    /// Update a specific trail
    pub fn update_trail(&mut self, id: &str, delta_time: f32, position: Vec2, velocity: Vec2) {
        if let Some(trail) = self.trails.get_mut(id) {
            trail.update(delta_time, position, velocity);
        }
    }

    /// Update all trails
    pub fn update_all(&mut self, delta_time: f32) {
        // Note: In practice, you'd need to provide position/velocity for each trail
        // This is a simplified version
    }

    /// Remove a trail
    pub fn remove_trail(&mut self, id: &str) {
        self.trails.remove(id);
    }

    /// Clear all trails
    pub fn clear_all(&mut self) {
        self.trails.clear();
    }

    /// Get all trail IDs
    pub fn get_trail_ids(&self) -> Vec<String> {
        self.trails.keys().cloned().collect()
    }

    /// Enable/disable a trail
    pub fn set_trail_enabled(&mut self, id: &str, enabled: bool) {
        if let Some(trail) = self.trails.get_mut(id) {
            trail.set_enabled(enabled);
        }
    }

    /// Get total number of segments across all trails
    pub fn total_segments(&self) -> usize {
        self.trails.values().map(|trail| trail.segment_count()).sum()
    }
}

impl Default for TrailSystem {
    fn default() -> Self {
        Self::new()
    }
}

/// Preset trail configurations
pub mod presets {
    use super::*;

    /// Create a Pong ball trail
    pub fn pong_ball_trail() -> TrailConfig {
        TrailConfig {
            max_segments: 20,
            segment_interval: 0.02, // 50 FPS
            segment_life: 0.5,
            base_color: [0.5, 0.5, 1.0, 0.8], // Light blue
            base_size: 2.0,
            fade_enabled: true,
            shrink_enabled: true,
            min_alpha: 0.1,
            fade_speed: 3.0,
            velocity_based: true,
            min_distance: 3.0,
        }
    }

    /// Create a fireball trail
    pub fn fireball_trail() -> TrailConfig {
        TrailConfig {
            max_segments: 15,
            segment_interval: 0.03,
            segment_life: 0.8,
            base_color: [1.0, 0.3, 0.0, 0.9], // Orange-red
            base_size: 4.0,
            fade_enabled: true,
            shrink_enabled: true,
            min_alpha: 0.2,
            fade_speed: 2.0,
            velocity_based: true,
            min_distance: 5.0,
        }
    }

    /// Create a spaceship engine trail
    pub fn spaceship_trail() -> TrailConfig {
        TrailConfig {
            max_segments: 25,
            segment_interval: 0.01, // 100 FPS
            segment_life: 0.3,
            base_color: [0.2, 0.8, 1.0, 0.7], // Cyan
            base_size: 3.0,
            fade_enabled: true,
            shrink_enabled: false,
            min_alpha: 0.0,
            fade_speed: 4.0,
            velocity_based: true,
            min_distance: 2.0,
        }
    }

    /// Create a magic spell trail
    pub fn magic_trail() -> TrailConfig {
        TrailConfig {
            max_segments: 30,
            segment_interval: 0.04,
            segment_life: 1.5,
            base_color: [0.8, 0.2, 1.0, 0.6], // Purple
            base_size: 5.0,
            fade_enabled: true,
            shrink_enabled: true,
            min_alpha: 0.1,
            fade_speed: 1.5,
            velocity_based: false,
            min_distance: 4.0,
        }
    }

    /// Create a simple particle trail
    pub fn simple_particle_trail() -> TrailConfig {
        TrailConfig {
            max_segments: 10,
            segment_interval: 0.1,
            segment_life: 1.0,
            base_color: [1.0, 1.0, 1.0, 0.5], // White semi-transparent
            base_size: 2.0,
            fade_enabled: true,
            shrink_enabled: true,
            min_alpha: 0.0,
            fade_speed: 2.0,
            velocity_based: false,
            min_distance: 8.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trail_segment() {
        let mut segment = TrailSegment::new(Vec2::new(0.0, 0.0), [1.0, 1.0, 1.0, 1.0], 5.0, 1.0);

        assert!(segment.update(0.5));
        assert_eq!(segment.life, 0.5);
        assert_eq!(segment.normalized_life(), 0.5);
        assert_eq!(segment.alpha(), 0.5);

        assert!(!segment.update(0.6)); // Should be dead
    }

    #[test]
    fn test_trail_creation() {
        let trail = Trail::new();
        assert_eq!(trail.config.max_segments, 20);
        assert!(trail.enabled);
        assert!(!trail.has_segments());
    }

    #[test]
    fn test_trail_update() {
        let mut trail = Trail::new();
        trail.config.segment_interval = 0.1;
        trail.config.min_distance = 1.0;

        // First update - should create a segment
        trail.update(0.2, Vec2::new(10.0, 0.0), Vec2::new(100.0, 0.0));
        assert_eq!(trail.segment_count(), 1);

        // Second update - should create another segment
        trail.update(0.2, Vec2::new(20.0, 0.0), Vec2::new(100.0, 0.0));
        assert_eq!(trail.segment_count(), 2);
    }

    #[test]
    fn test_trail_system() {
        let mut system = TrailSystem::new();

        system.create_trail("ball_trail");
        assert!(system.get_trail("ball_trail").is_some());

        system.update_trail("ball_trail", 0.1, Vec2::new(0.0, 0.0), Vec2::new(0.0, 0.0));
        assert_eq!(system.total_segments(), 0); // No movement, no segments

        system.clear_all();
        assert_eq!(system.get_trail_ids().len(), 0);
    }

    #[test]
    fn test_trail_presets() {
        let pong_config = presets::pong_ball_trail();
        assert_eq!(pong_config.max_segments, 20);
        assert_eq!(pong_config.base_color, [0.5, 0.5, 1.0, 0.8]);

        let fireball_config = presets::fireball_trail();
        assert_eq!(fireball_config.base_color, [1.0, 0.3, 0.0, 0.9]);
    }
}