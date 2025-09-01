//! Game loop module
//!
//! Main game loop with fixed timestep and frame rate management.

use std::time::{Duration, Instant};

/// Game loop configuration
pub struct GameLoopConfig {
    pub target_fps: u32,
    pub max_frame_time: Duration,
}

impl Default for GameLoopConfig {
    fn default() -> Self {
        Self {
            target_fps: 60,
            max_frame_time: Duration::from_millis(100),
        }
    }
}

/// Game loop runner
pub struct GameLoop {
    config: GameLoopConfig,
    last_time: Instant,
    accumulator: Duration,
    frame_count: u64,
}

impl GameLoop {
    pub fn new(config: GameLoopConfig) -> Self {
        Self {
            config,
            last_time: Instant::now(),
            accumulator: Duration::ZERO,
            frame_count: 0,
        }
    }

    pub fn run<F>(&mut self, mut update_fn: F)
    where
        F: FnMut(f32),
    {
        let target_frame_time = Duration::from_secs(1) / self.config.target_fps;

        loop {
            let current_time = Instant::now();
            let mut delta_time = current_time.duration_since(self.last_time);
            self.last_time = current_time;

            // Prevent spiral of death
            if delta_time > self.config.max_frame_time {
                delta_time = self.config.max_frame_time;
            }

            self.accumulator += delta_time;

            // Update with fixed timestep
            while self.accumulator >= target_frame_time {
                let dt = target_frame_time.as_secs_f32();
                update_fn(dt);
                self.accumulator -= target_frame_time;
                self.frame_count += 1;
            }

            // Optional: render with interpolation
            // let alpha = self.accumulator.as_secs_f32() / target_frame_time.as_secs_f32();
            // render_fn(alpha);
        }
    }
}