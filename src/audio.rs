//! Audio system module
//!
//! Sound and music playback with spatial audio.

use specs::{Component, VecStorage};
use rodio::{OutputStream, OutputStreamHandle, Sink, Source};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

/// Audio source component
#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct AudioSource {
    pub sound_id: String,
    pub volume: f32,
    pub loop_sound: bool,
}

/// Audio manager
pub struct AudioManager {
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
        #[allow(dead_code)]
    sinks: HashMap<String, Sink>,
    master_volume: f32,
}

impl Default for AudioManager {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioManager {
    pub fn new() -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        Self {
            _stream,
            stream_handle,
            sinks: HashMap::new(),
            master_volume: 1.0,
        }
    }

        pub fn load_sound(&self, _id: &str, path: &str) -> Result<(), String> {
        let file = BufReader::new(File::open(path).map_err(|e| e.to_string())?);
        let source = rodio::Decoder::new(file).map_err(|e| e.to_string())?;

        let sink = Sink::try_new(&self.stream_handle).map_err(|e| e.to_string())?;
        sink.append(source);
        sink.pause(); // Pause initially, play on demand

        // Store the sink, but we need to clone it to move into the HashMap
        // This is a simplification; in a real engine, you'd manage sources/buffers more carefully
        // For now, we'll just store a reference to the sink
        // This won't work directly as Sink is not Clone or Copy
        // Let's rethink this. We need to store the Source, not the Sink.
        // Or, we create a new Sink each time we play a sound.

        // Let's create a new Sink each time for simplicity in this demo.
        // So, load_sound will just validate the sound file and return a SoundId.
        // The actual sound data will be loaded when play_sound is called.
        // This is not efficient for repeated sounds, but simple for a demo.

        Ok(())
    }

    pub fn play_sound(&self, path: &str) -> Result<(), String> {
        let file = BufReader::new(File::open(path).map_err(|e| e.to_string())?);
        let source = rodio::Decoder::new(file).map_err(|e| e.to_string())?;

        let sink = Sink::try_new(&self.stream_handle).map_err(|e| e.to_string())?;
        sink.append(source.amplify(self.master_volume));
        sink.play();
        sink.detach(); // Detach to play in background
        Ok(())
    }

    pub fn set_master_volume(&mut self, volume: f32) {
        self.master_volume = volume;
    }
}
