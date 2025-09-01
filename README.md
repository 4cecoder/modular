# Modular Game Engine

A modular game engine built with Rust, designed for creating "dream games" through clean, composable systems. Each system is isolated and can be developed, tested, and optimized independently.

## 🏗️ Architecture

The engine follows a modular architecture with these core principles:

- **Entity Component System (ECS)**: Data-oriented architecture for performance
- **System Isolation**: Each system operates independently with clear interfaces
- **Plugin Architecture**: Extensible through dynamic plugin loading
- **Event-Driven**: Decoupled communication between systems
- **Resource Management**: Efficient asset loading and caching

## 📁 Project Structure

```
modular_game_engine/
├── src/
│   ├── lib.rs              # Main library interface
│   ├── ecs.rs              # Entity Component System
│   ├── components.rs       # Game components
│   ├── systems.rs          # Core game systems
│   ├── physics.rs          # Physics simulation
│   ├── rendering.rs        # Rendering pipeline
│   ├── input.rs            # Input handling
│   ├── ai.rs               # AI and pathfinding
│   ├── audio.rs            # Audio system
│   ├── ui.rs               # User interface
│   ├── resources.rs        # Asset management
│   ├── plugins.rs          # Plugin system
│   ├── events.rs           # Event system
│   └── game_loop.rs        # Main game loop
├── demos/
│   ├── ecs_demo.rs         # ECS functionality demo
│   ├── physics_demo.rs     # Physics simulation demo
│   └── rendering_demo.rs   # Rendering pipeline demo
├── docs/
│   └── systems/            # System documentation
└── Cargo.toml
```

## 🚀 Quick Start

### Prerequisites

- Rust 1.70 or later
- Cargo package manager

### Building the Engine

```bash
# Clone the repository
git clone https://github.com/4cecoder/modular
cd modular

# Build the library
cargo build

# Run tests
cargo test

# Build with optimizations
cargo build --release
```

### Running Demos

Each demo showcases a specific system in isolation:

```bash
# ECS Demo - Entity Component System functionality
cargo run --bin ecs_demo

# Physics Demo - Physics simulation and collision
cargo run --bin physics_demo

# Rendering Demo - Sprite rendering and camera systems
cargo run --bin rendering_demo
```

## 🎮 Core Systems

### Entity Component System (ECS)

The foundation of the engine using the Specs crate:

```rust
use modular_game_engine::*;

// Create a world
let mut world = init()?;

// Create entities with components
world.create_entity_with_components()
    .with(Position::new(0.0, 0.0))
    .with(Velocity::new(10.0, 5.0))
    .with(Renderable::new("player.png".to_string()))
    .build();

// Run systems
dispatcher.dispatch(&world);
```

**Key Features:**
- Component-based entity composition
- Efficient system iteration
- Parallel system execution
- Dynamic component addition/removal

### Physics System

Realistic physics simulation with collision detection:

```rust
// Create physics entities
world.create_entity_with_components()
    .with(Position::new(0.0, 0.0))
    .with(Velocity::new(10.0, 0.0))
    .with(Acceleration::new(0.0, -9.81)) // Gravity
    .with(Mass(1.0))
    .with(Collider::new_circle(10.0))
    .build();
```

**Features:**
- Velocity and acceleration integration
- Collision detection (circle, rectangle)
- Force application
- Physics materials (restitution, friction)

### Rendering System

Efficient sprite rendering with camera support:

```rust
// Create renderable entities
world.create_entity_with_components()
    .with(Position::new(100.0, 100.0))
    .with(Renderable {
        sprite_id: "player".to_string(),
        layer: 1,
        visible: true,
        scale: 1.0,
    })
    .with(Animation::new(vec!["frame1".to_string(), "frame2".to_string()], 0.2))
    .build();

// Create camera
world.create_entity_with_components()
    .with(Camera2D {
        position: Vec2::new(0.0, 0.0),
        zoom: 1.0,
        rotation: 0.0,
        viewport_size: Vec2::new(800.0, 600.0),
    })
    .build();
```

**Features:**
- Layered rendering
- Camera following and zooming
- Sprite animation
- Viewport culling

### Input System

Unified input handling across platforms:

```rust
// Input is handled automatically by the InputSystem
// Access current input state
let input_state = world.read_resource::<InputState>();

if input_state.keys_pressed.contains(&VirtualKeyCode::Space) {
    // Handle spacebar press
}
```

**Features:**
- Keyboard and mouse input
- Action mapping system
- Input buffering
- Platform abstraction

### AI System

Intelligent agent behavior with pathfinding:

```rust
// Create AI entities
world.create_entity_with_components()
    .with(Position::new(0.0, 0.0))
    .with(AIState {
        current_state: "patrol".to_string(),
        target_position: Some(Vec2::new(100.0, 0.0)),
    })
    .with(Path::new(vec![Vec2::new(50.0, 0.0), Vec2::new(100.0, 0.0)]))
    .build();
```

**Features:**
- Finite state machines
- A* pathfinding
- Steering behaviors
- Group AI coordination

### Audio System

Spatial audio with effects processing:

```rust
// Create audio sources
world.create_entity_with_components()
    .with(Position::new(0.0, 0.0))
    .with(AudioSource {
        sound_id: "background_music".to_string(),
        volume: 0.8,
        loop_sound: true,
    })
    .with(SpatialAudio {
        position: Vec3::new(0.0, 0.0, 0.0),
        range: 100.0,
    })
    .build();
```

**Features:**
- 3D spatial audio
- Reverb and effects
- Dynamic mixing
- Streaming audio

### UI System

Component-based user interface:

```rust
// Create UI elements
world.create_entity_with_components()
    .with(UIElement {
        element_type: UIElementType::Button {
            text: "Play Game".to_string(),
        },
        position: Vec2::new(0.0, 0.0),
        size: Vec2::new(200.0, 50.0),
    })
    .build();
```

**Features:**
- Component-based UI
- Styling system
- Layout engine
- Event handling

## 🔧 Development Workflow

### 1. System Isolation
Each system is developed independently:

```bash
# Run only the physics demo
cargo run --bin physics_demo

# Test only physics components
cargo test physics
```

### 2. Incremental Integration
Add systems one by one to your game:

```rust
let mut dispatcher = DispatcherBuilder::new()
    .with(PhysicsSystem, "physics", &[])
    .with(RenderingSystem, "rendering", &["physics"])
    .with(AISystem, "ai", &["physics"])
    .build();
```

### 3. Plugin Development
Extend functionality with plugins:

```rust
// Create a plugin
pub struct MyPlugin;

impl Plugin for MyPlugin {
    fn initialize(&mut self, context: &mut PluginContext) -> Result<(), PluginError> {
        // Add custom components and systems
        Ok(())
    }
}
```

## 📊 Performance

The modular architecture provides excellent performance:

- **ECS**: Cache-friendly data layout
- **Parallel Systems**: Independent systems run concurrently
- **Resource Pooling**: Reuse expensive resources
- **Spatial Partitioning**: Efficient queries for large worlds

## 🧪 Testing

Run the test suite:

```bash
# Run all tests
cargo test

# Run specific system tests
cargo test physics
cargo test rendering

# Run benchmarks
cargo bench
```

## 📚 Documentation

Detailed documentation for each system:

- [ECS System](docs/systems/ecs.md)
- [Component System](docs/systems/components.md)
- [Physics System](docs/systems/physics.md)
- [Rendering System](docs/systems/rendering.md)
- [Input System](docs/systems/input.md)
- [AI System](docs/systems/ai.md)
- [Audio System](docs/systems/audio.md)
- [UI System](docs/systems/ui.md)
- [Resource Management](docs/systems/resource_management.md)
- [Plugin System](docs/systems/plugin_system.md)
- [Event System](docs/systems/event_system.md)

## 🎯 Demo Showcase

### ECS Demo
```bash
cargo run --bin ecs_demo
```
Creates entities with different components and demonstrates system execution.

### Physics Demo
```bash
cargo run --bin physics_demo
```
Shows physics simulation with bouncing balls, collision detection, and force application.

### Rendering Demo
```bash
cargo run --bin rendering_demo
```
Demonstrates sprite rendering, animation, camera following, and layered rendering.

## 🚀 Next Steps

1. **Add More Demos**: Create demos for input, AI, audio, and UI systems
2. **Graphics Backend**: Integrate wgpu for actual rendering
3. **Asset Pipeline**: Add asset processing and optimization
4. **Networking**: Multiplayer support
5. **Scripting**: Lua or custom scripting language
6. **Tools**: Level editor, profiler, debugger

## 🤝 Contributing

1. Choose a system to work on
2. Create isolated demos and tests
3. Follow the modular architecture principles
4. Document your changes
5. Submit a pull request

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

---

**Built with ❤️ and Rust for creating dream games through modular excellence.**