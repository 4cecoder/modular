//! Particle System
//!
//! A flexible particle system for creating visual effects like explosions,
//! trails, sparks, and other dynamic visual feedback. Extracted from the Pong game.

use crate::Vec2;

/// Individual particle with physics and visual properties
#[derive(Debug, Clone)]
pub struct Particle {
    /// Current position
    pub position: Vec2,
    /// Current velocity
    pub velocity: Vec2,
    /// Current acceleration (for gravity, wind, etc.)
    pub acceleration: Vec2,
    /// Current life (0.0 to 1.0, where 1.0 is full life)
    pub life: f32,
    /// Maximum life duration
    pub max_life: f32,
    /// Current size
    pub size: f32,
    /// Initial size
    pub initial_size: f32,
    /// Current color (RGBA)
    pub color: [f32; 4],
    /// Initial color
    pub initial_color: [f32; 4],
    /// Rotation angle in radians
    pub rotation: f32,
    /// Rotation speed in radians per second
    pub rotation_speed: f32,
    /// Texture index (for systems with multiple particle textures)
    pub texture_index: usize,
    /// Custom data for game-specific behavior
    pub user_data: f32,
}

impl Particle {
    /// Create a new particle with default values
    pub fn new() -> Self {
        Self {
            position: Vec2::new(0.0, 0.0),
            velocity: Vec2::new(0.0, 0.0),
            acceleration: Vec2::new(0.0, 0.0),
            life: 1.0,
            max_life: 1.0,
            size: 1.0,
            initial_size: 1.0,
            color: [1.0, 1.0, 1.0, 1.0],
            initial_color: [1.0, 1.0, 1.0, 1.0],
            rotation: 0.0,
            rotation_speed: 0.0,
            texture_index: 0,
            user_data: 0.0,
        }
    }

    /// Check if particle is still alive
    pub fn is_alive(&self) -> bool {
        self.life > 0.0
    }

    /// Get the normalized life (0.0 to 1.0)
    pub fn normalized_life(&self) -> f32 {
        self.life / self.max_life
    }

    /// Update particle physics and life
    pub fn update(&mut self, delta_time: f32) {
        // Update physics
        self.velocity = self.velocity + self.acceleration * delta_time;
        self.position = self.position + self.velocity * delta_time;

        // Update rotation
        self.rotation += self.rotation_speed * delta_time;

        // Update life
        self.life -= delta_time;
        if self.life < 0.0 {
            self.life = 0.0;
        }

        // Update size based on life (optional fade out)
        let life_ratio = self.normalized_life();
        self.size = self.initial_size * life_ratio;

        // Update color alpha based on life
        self.color[3] = self.initial_color[3] * life_ratio;
    }
}

/// Configuration for particle emission
#[derive(Debug, Clone)]
pub struct ParticleEmitterConfig {
    /// Position where particles are emitted
    pub position: Vec2,
    /// Direction of emission (normalized vector)
    pub direction: Vec2,
    /// Spread angle in radians (0 = straight line, PI = full circle)
    pub spread: f32,
    /// Number of particles to emit per second
    pub rate: f32,
    /// Initial speed of particles
    pub speed: f32,
    /// Speed variation (± this value)
    pub speed_variation: f32,
    /// Initial size of particles
    pub size: f32,
    /// Size variation (± this value)
    pub size_variation: f32,
    /// Life duration of particles
    pub life: f32,
    /// Life variation (± this value)
    pub life_variation: f32,
    /// Initial color
    pub color: [f32; 4],
    /// Color variation (± this value for each component)
    pub color_variation: [f32; 4],
    /// Gravity acceleration
    pub gravity: Vec2,
    /// Rotation speed
    pub rotation_speed: f32,
    /// Rotation speed variation
    pub rotation_variation: f32,
    /// Texture index
    pub texture_index: usize,
    /// Whether emission is active
    pub active: bool,
    /// Maximum number of particles this emitter can have
    pub max_particles: usize,
}

impl Default for ParticleEmitterConfig {
    fn default() -> Self {
        Self {
            position: Vec2::new(0.0, 0.0),
            direction: Vec2::new(0.0, -1.0),    // Up
            spread: std::f32::consts::PI / 4.0, // 45 degrees
            rate: 10.0,
            speed: 100.0,
            speed_variation: 20.0,
            size: 5.0,
            size_variation: 2.0,
            life: 2.0,
            life_variation: 0.5,
            color: [1.0, 1.0, 1.0, 1.0],
            color_variation: [0.1, 0.1, 0.1, 0.0],
            gravity: Vec2::new(0.0, 100.0),
            rotation_speed: 0.0,
            rotation_variation: 0.0,
            texture_index: 0,
            active: true,
            max_particles: 100,
        }
    }
}

/// Particle emitter that creates and manages particles
#[derive(Debug, Clone)]
pub struct ParticleEmitter {
    pub config: ParticleEmitterConfig,
    /// Time accumulator for emission timing
    pub emission_timer: f32,
    /// Particles managed by this emitter
    pub particles: Vec<Particle>,
}

impl ParticleEmitter {
    /// Create a new emitter with default configuration
    pub fn new() -> Self {
        Self {
            config: ParticleEmitterConfig::default(),
            emission_timer: 0.0,
            particles: Vec::new(),
        }
    }

    /// Create a new emitter with custom configuration
    pub fn with_config(config: ParticleEmitterConfig) -> Self {
        Self {
            config,
            emission_timer: 0.0,
            particles: Vec::new(),
        }
    }

    /// Update the emitter and all its particles
    pub fn update(&mut self, delta_time: f32) {
        // Update existing particles
        self.particles.retain_mut(|particle| {
            particle.update(delta_time);
            particle.is_alive()
        });

        // Emit new particles if active
        if self.config.active {
            self.emission_timer += delta_time;
            let emission_interval = 1.0 / self.config.rate;

            while self.emission_timer >= emission_interval
                && self.particles.len() < self.config.max_particles
            {
                self.emit_particle();
                self.emission_timer -= emission_interval;
            }
        }
    }

    /// Emit a single particle
    pub fn emit_particle(&mut self) {
        let mut particle = Particle::new();

        // Set position
        particle.position = self.config.position;

        // Calculate direction with spread
        let angle_variation = (rand::random::<f32>() - 0.5) * self.config.spread;
        let base_angle = self.config.direction.y.atan2(self.config.direction.x);
        let final_angle = base_angle + angle_variation;

        // Set velocity
        let speed =
            self.config.speed + (rand::random::<f32>() - 0.5) * 2.0 * self.config.speed_variation;
        particle.velocity = Vec2::new(final_angle.cos() * speed, final_angle.sin() * speed);

        // Set acceleration (gravity)
        particle.acceleration = self.config.gravity;

        // Set life
        particle.max_life =
            self.config.life + (rand::random::<f32>() - 0.5) * 2.0 * self.config.life_variation;
        particle.life = particle.max_life;

        // Set size
        particle.initial_size =
            self.config.size + (rand::random::<f32>() - 0.5) * 2.0 * self.config.size_variation;
        particle.size = particle.initial_size;

        // Set color
        particle.initial_color = self.config.color;
        for i in 0..4 {
            particle.color[i] = (self.config.color[i]
                + (rand::random::<f32>() - 0.5) * 2.0 * self.config.color_variation[i])
                .clamp(0.0, 1.0);
        }
        particle.initial_color = particle.color;

        // Set rotation
        particle.rotation_speed = self.config.rotation_speed
            + (rand::random::<f32>() - 0.5) * 2.0 * self.config.rotation_variation;

        // Set texture
        particle.texture_index = self.config.texture_index;

        self.particles.push(particle);
    }

    /// Emit a burst of particles immediately
    pub fn burst(&mut self, count: usize) {
        for _ in 0..count {
            if self.particles.len() < self.config.max_particles {
                self.emit_particle();
            }
        }
    }

    /// Clear all particles
    pub fn clear(&mut self) {
        self.particles.clear();
    }

    /// Get the number of active particles
    pub fn particle_count(&self) -> usize {
        self.particles.len()
    }

    /// Check if emitter has any active particles
    pub fn has_particles(&self) -> bool {
        !self.particles.is_empty()
    }
}

/// Main particle system that manages multiple emitters
pub struct ParticleSystem {
    emitters: Vec<ParticleEmitter>,
    /// Global gravity for all emitters
    global_gravity: Vec2,
    /// Time scale for slow motion effects
    time_scale: f32,
}

impl ParticleSystem {
    /// Create a new particle system
    pub fn new() -> Self {
        Self {
            emitters: Vec::new(),
            global_gravity: Vec2::new(0.0, 100.0),
            time_scale: 1.0,
        }
    }

    /// Add a new emitter to the system
    pub fn add_emitter(&mut self, emitter: ParticleEmitter) -> usize {
        self.emitters.push(emitter);
        self.emitters.len() - 1
    }

    /// Remove an emitter by index
    pub fn remove_emitter(&mut self, index: usize) {
        if index < self.emitters.len() {
            self.emitters.remove(index);
        }
    }

    /// Get mutable reference to an emitter
    pub fn get_emitter_mut(&mut self, index: usize) -> Option<&mut ParticleEmitter> {
        self.emitters.get_mut(index)
    }

    /// Get reference to an emitter
    pub fn get_emitter(&self, index: usize) -> Option<&ParticleEmitter> {
        self.emitters.get(index)
    }

    /// Update all emitters and their particles
    pub fn update(&mut self, delta_time: f32) {
        let scaled_delta = delta_time * self.time_scale;

        for emitter in &mut self.emitters {
            // Apply global gravity if emitter doesn't have its own
            if emitter.config.gravity.x == 0.0 && emitter.config.gravity.y == 0.0 {
                emitter.config.gravity = self.global_gravity;
            }
            emitter.update(scaled_delta);
        }

        // Remove empty emitters
        self.emitters
            .retain(|emitter| emitter.config.active || emitter.has_particles());
    }

    /// Create a preset explosion effect
    pub fn create_explosion(&mut self, position: Vec2, intensity: f32) -> usize {
        let config = ParticleEmitterConfig {
            position,
            direction: Vec2::new(0.0, 0.0),     // Radial
            spread: std::f32::consts::PI * 2.0, // Full circle
            rate: 0.0,                          // Burst only
            speed: 50.0 * intensity,
            speed_variation: 20.0 * intensity,
            size: 3.0 * intensity,
            size_variation: 1.0,
            life: 1.0 * intensity,
            life_variation: 0.3,
            color: [1.0, 0.5, 0.0, 1.0], // Orange
            color_variation: [0.2, 0.2, 0.0, 0.0],
            gravity: Vec2::new(0.0, 50.0),
            rotation_speed: 5.0,
            rotation_variation: 2.0,
            texture_index: 0,
            active: false,
            max_particles: 50,
        };

        let mut emitter = ParticleEmitter::with_config(config);
        emitter.burst((20.0 * intensity) as usize);

        self.add_emitter(emitter)
    }

    /// Create a preset trail effect
    pub fn create_trail(&mut self, position: Vec2, velocity: Vec2) -> usize {
        let config = ParticleEmitterConfig {
            position,
            direction: velocity.normalize() * -1.0, // Opposite to movement
            spread: std::f32::consts::PI / 6.0,     // Narrow spread
            rate: 20.0,
            speed: velocity.magnitude() * 0.5,
            speed_variation: 10.0,
            size: 2.0,
            size_variation: 1.0,
            life: 0.5,
            life_variation: 0.2,
            color: [0.5, 0.5, 1.0, 0.8], // Light blue
            color_variation: [0.1, 0.1, 0.1, 0.0],
            gravity: Vec2::new(0.0, 0.0), // No gravity for trail
            rotation_speed: 0.0,
            rotation_variation: 0.0,
            texture_index: 0,
            active: true,
            max_particles: 20,
        };

        let emitter = ParticleEmitter::with_config(config);
        self.add_emitter(emitter)
    }

    /// Create a preset spark effect
    pub fn create_sparks(&mut self, position: Vec2, direction: Vec2) -> usize {
        let config = ParticleEmitterConfig {
            position,
            direction,
            spread: std::f32::consts::PI / 3.0, // 60 degrees
            rate: 30.0,
            speed: 80.0,
            speed_variation: 30.0,
            size: 1.5,
            size_variation: 0.5,
            life: 0.8,
            life_variation: 0.3,
            color: [1.0, 1.0, 0.0, 1.0], // Yellow
            color_variation: [0.0, 0.0, 0.0, 0.0],
            gravity: Vec2::new(0.0, 200.0), // Strong gravity
            rotation_speed: 10.0,
            rotation_variation: 5.0,
            texture_index: 0,
            active: true,
            max_particles: 30,
        };

        let emitter = ParticleEmitter::with_config(config);
        self.add_emitter(emitter)
    }

    /// Set global gravity for all emitters
    pub fn set_global_gravity(&mut self, gravity: Vec2) {
        self.global_gravity = gravity;
    }

    /// Set time scale for slow motion effects
    pub fn set_time_scale(&mut self, scale: f32) {
        self.time_scale = scale;
    }

    /// Get total number of active particles across all emitters
    pub fn total_particle_count(&self) -> usize {
        self.emitters.iter().map(|e| e.particle_count()).sum()
    }

    /// Clear all emitters and particles
    pub fn clear(&mut self) {
        self.emitters.clear();
    }
}

impl Default for ParticleSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_particle_creation() {
        let particle = Particle::new();
        assert!(particle.is_alive());
        assert_eq!(particle.normalized_life(), 1.0);
    }

    #[test]
    fn test_particle_update() {
        let mut particle = Particle::new();
        particle.velocity = Vec2::new(10.0, 0.0);
        particle.max_life = 2.0;
        particle.life = 2.0;

        particle.update(1.0);

        assert_eq!(particle.position.x, 10.0);
        assert_eq!(particle.life, 1.0);
        assert!(particle.is_alive());
    }

    #[test]
    fn test_emitter_burst() {
        let mut emitter = ParticleEmitter::new();
        emitter.config.max_particles = 10;

        emitter.burst(5);
        assert_eq!(emitter.particle_count(), 5);

        emitter.burst(8); // Should be limited to max_particles
        assert_eq!(emitter.particle_count(), 10);
    }

    #[test]
    fn test_particle_system() {
        let mut system = ParticleSystem::new();

        let _emitter_idx = system.create_explosion(Vec2::new(0.0, 0.0), 1.0);
        assert_eq!(system.total_particle_count(), 20);

        system.update(0.1);
        assert!(system.total_particle_count() > 0); // Particles should still exist

        system.clear();
        assert_eq!(system.total_particle_count(), 0);
    }
}
