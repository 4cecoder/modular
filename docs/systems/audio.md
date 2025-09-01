# Audio System

## Overview
The Audio System manages all sound playback, music, and audio effects in the game. It provides a comprehensive audio engine with support for 2D/3D spatial audio, dynamic mixing, and various audio formats.

## Core Architecture

### Audio Engine
Central audio processing hub:

```rust
pub struct AudioEngine {
    device: AudioDevice,
    context: AudioContext,
    mixer: AudioMixer,
    resource_manager: AudioResourceManager,
    spatial_audio: SpatialAudioProcessor,
}
```

### Audio Sources
Different types of audio playback:

```rust
pub enum AudioSource {
    SoundEffect {
        buffer: AudioBuffer,
        volume: f32,
        pitch: f32,
        loop: bool,
    },
    Music {
        stream: AudioStream,
        volume: f32,
        fade_time: f32,
    },
    Ambient {
        buffer: AudioBuffer,
        volume: f32,
        position: Vec3,
    },
}
```

## Audio Components

### Basic Audio Components
```rust
#[derive(Component, Debug, Clone)]
pub struct AudioEmitter {
    pub sound_id: String,
    pub volume: f32,
    pub pitch: f32,
    pub loop: bool,
    pub playing: bool,
}

#[derive(Component, Debug, Clone)]
pub struct AudioListener {
    pub active: bool,
    pub volume: f32,
}
```

### Spatial Audio Components
```rust
#[derive(Component, Debug, Clone)]
pub struct SpatialAudio {
    pub position: Vec3,
    pub velocity: Vec3,
    pub direction: Vec3,
    pub inner_angle: f32,
    pub outer_angle: f32,
    pub outer_gain: f32,
}

#[derive(Component, Debug, Clone)]
pub struct ReverbZone {
    pub position: Vec3,
    pub size: Vec3,
    pub reverb_preset: ReverbPreset,
}
```

## Audio Playback

### Sound Effects
Short audio clips for game events:

```rust
pub struct SoundEffect {
    pub buffer: AudioBuffer,
    pub category: AudioCategory,
    pub attenuation: AttenuationModel,
}

impl SoundEffect {
    pub fn play_at(&self, position: Vec3) -> AudioInstance {
        // Create spatial audio instance
    }
}
```

### Music Playback
Background music with crossfading:

```rust
pub struct MusicPlayer {
    pub current_track: Option<AudioStream>,
    pub next_track: Option<AudioStream>,
    pub fade_time: f32,
    pub fade_progress: f32,
}

impl MusicPlayer {
    pub fn play_track(&mut self, track: AudioStream, fade_time: f32) {
        // Crossfade to new track
    }
}
```

### Audio Categories
Organize audio by type for volume control:

```rust
#[derive(Debug, Clone, Copy)]
pub enum AudioCategory {
    Master,
    Music,
    SoundEffects,
    Ambient,
    Voice,
    UI,
}

#[derive(Debug, Clone)]
pub struct AudioSettings {
    pub master_volume: f32,
    pub category_volumes: HashMap<AudioCategory, f32>,
}
```

## Spatial Audio

### 3D Audio Processing
Position audio in 3D space:

```rust
pub struct SpatialAudioProcessor {
    listener_position: Vec3,
    listener_orientation: Quat,
}

impl SpatialAudioProcessor {
    pub fn process_source(&self, source: &SpatialAudio) -> ProcessedAudio {
        // Calculate volume and panning based on position
        let distance = (source.position - self.listener_position).magnitude();
        let volume = self.calculate_attenuation(distance, source.attenuation);

        let direction = (source.position - self.listener_position).normalize();
        let panning = self.calculate_panning(direction);

        ProcessedAudio { volume, panning, .. }
    }
}
```

### Attenuation Models
Control how sound volume decreases with distance:

```rust
pub enum AttenuationModel {
    None,
    Linear { max_distance: f32 },
    Inverse { rolloff: f32 },
    Exponential { rolloff: f32 },
}

impl AttenuationModel {
    pub fn calculate_volume(&self, distance: f32) -> f32 {
        match self {
            AttenuationModel::None => 1.0,
            AttenuationModel::Linear { max_distance } => {
                (max_distance - distance).max(0.0) / max_distance
            }
            // ... other models
        }
    }
}
```

## Audio Effects

### Reverb
Simulate acoustic environments:

```rust
#[derive(Debug, Clone)]
pub struct ReverbEffect {
    pub preset: ReverbPreset,
    pub wet_dry_mix: f32,
    pub room_size: f32,
    pub damping: f32,
}

#[derive(Debug, Clone)]
pub enum ReverbPreset {
    Generic,
    PaddedCell,
    Room,
    Bathroom,
    LivingRoom,
    StoneRoom,
    Auditorium,
    ConcertHall,
}
```

### Filters
Modify audio frequency content:

```rust
pub enum AudioFilter {
    LowPass { cutoff: f32, resonance: f32 },
    HighPass { cutoff: f32, resonance: f32 },
    BandPass { center: f32, bandwidth: f32 },
    Notch { center: f32, bandwidth: f32 },
}
```

### Dynamic Range Compression
Control audio dynamics:

```rust
pub struct Compressor {
    pub threshold: f32,
    pub ratio: f32,
    pub attack: f32,
    pub release: f32,
    pub makeup_gain: f32,
}
```

## Audio Resources

### Resource Management
Load and cache audio assets:

```rust
pub struct AudioResourceManager {
    sounds: HashMap<String, AudioBuffer>,
    music: HashMap<String, AudioStream>,
    cache: LruCache<String, AudioBuffer>,
}

impl AudioResourceManager {
    pub fn load_sound(&mut self, id: &str, path: &str) -> Result<(), AudioError> {
        // Load and cache audio file
    }

    pub fn get_sound(&self, id: &str) -> Option<&AudioBuffer> {
        self.cache.get(id).or_else(|| self.sounds.get(id))
    }
}
```

### Streaming Audio
Handle large audio files:

```rust
pub struct AudioStreamer {
    file: File,
    buffer: RingBuffer<f32>,
    decoder: AudioDecoder,
}

impl AudioStreamer {
    pub fn fill_buffer(&mut self) -> Result<(), AudioError> {
        // Stream audio data from disk
    }
}
```

## Integration with ECS

### Audio System
```rust
impl<'a> System<'a> for AudioSystem {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, AudioEmitter>,
        ReadStorage<'a, AudioListener>,
        ReadStorage<'a, SpatialAudio>,
    );

    fn run(&mut self, (positions, emitters, listeners, spatial): Self::SystemData) {
        // Update listener position
        for (position, listener) in (&positions, &listeners).join() {
            if listener.active {
                self.audio_engine.set_listener_position(position.0);
            }
        }

        // Update spatial audio
        for (position, emitter, spatial) in (&positions, &emitters, &spatial).join() {
            self.audio_engine.update_spatial_source(emitter, position.0, spatial);
        }
    }
}
```

## Performance Optimizations

### Audio Pooling
Reuse audio instances:

```rust
pub struct AudioPool {
    available: Vec<AudioInstance>,
    max_instances: usize,
}

impl AudioPool {
    pub fn play_sound(&mut self, sound: &SoundEffect) -> Option<AudioInstance> {
        if let Some(instance) = self.available.pop() {
            instance.play(sound);
            Some(instance)
        } else if self.available.len() < self.max_instances {
            Some(AudioInstance::new(sound))
        } else {
            None
        }
    }
}
```

### Distance Culling
Only process nearby audio sources:

```rust
impl AudioEngine {
    pub fn cull_distant_sources(&mut self, listener_pos: Vec3, max_distance: f32) {
        self.sources.retain(|source| {
            let distance = (source.position - listener_pos).magnitude();
            distance <= max_distance
        });
    }
}
```

### Audio Threading
Process audio on separate thread:

```rust
pub struct AudioThread {
    command_queue: mpsc::Receiver<AudioCommand>,
    audio_engine: AudioEngine,
}

impl AudioThread {
    pub fn run(mut self) {
        while let Ok(command) = self.command_queue.recv() {
            self.process_command(command);
        }
    }
}
```

## Best Practices

### Design
1. Use appropriate audio categories
2. Implement spatial audio for immersion
3. Balance audio levels carefully
4. Consider audio accessibility options

### Performance
1. Pool audio instances
2. Cull distant audio sources
3. Use streaming for music
4. Profile audio performance

### Quality
1. Use high-quality audio assets
2. Implement proper mixing
3. Add audio effects sparingly
4. Test on target hardware

### Debugging
1. Visualize audio sources
2. Log audio events
3. Implement audio debugging tools
4. Monitor audio performance

## Integration Points

### Game Systems
- **Physics**: Audio responds to collisions
- **Rendering**: Visual feedback for audio
- **Input**: Audio feedback for actions
- **UI**: Sound effects for interface

### Events
- **Audio Events**: Published for audio state changes
- **Game Events**: Trigger audio playback
- **System Events**: Handle audio device changes

### Persistence
- **Settings**: Store audio preferences
- **State**: Save audio state
- **Resources**: Cache audio assets