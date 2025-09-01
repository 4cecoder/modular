# Running the Modular Game Engine Demos

This guide shows you how to run the different demos that showcase the modular game engine's capabilities.

## Available Demos

### 1. ECS Demo (`ecs_demo`)
**What it does:** Demonstrates the Entity Component System functionality
- Shows entity creation and management
- Displays system execution and data flow
- Tracks entity count and player health in real-time

**How to run:**
```bash
cargo run --bin ecs_demo
```

**What you'll see:**
```
Frame | Entities | Player Health | Enemy Count
-------|----------|---------------|------------
   10 |        9 | 100/100       |          5
   20 |        9 | 100/100       |          5
```

### 2. Physics Demo (`physics_demo`)
**What it does:** Shows realistic physics simulation
- Ball movement with velocity and acceleration
- Collision detection between objects
- Force application and material properties
- Real-time physics debugging

**How to run:**
```bash
cargo run --bin physics_demo
```

**What you'll see:**
```
Frame | Ball 1 Pos | Ball 2 Pos | Collisions
-------|-----------|-----------|-----------
    0 | (   0.8,   0.4) | (  99.5,   0.3) |         6
   30 | (  25.8,  23.8) | (  84.5,  21.3) |         6
```

### 3. Simple Graphical Pong (`simple_graphical_pong`)
**What it does:** Pong game with terminal-based graphics
- Complete Pong gameplay with paddles and ball
- ASCII art graphics in the terminal
- Real-time game state display
- Menu system and game controls

**How to run:**
```bash
cargo run --bin simple_graphical_pong
```

**What you'll see:**
```
╔══════════════════════════════════════════════════════════════╗
║                    PONG - Modular Engine                    ║
║                                                              ║
║                   Press SPACE to Start                      ║
║                   W/S: Move Paddle                          ║
╚══════════════════════════════════════════════════════════════╝
```

### 4. Window Pong (`window_pong`)
**What it does:** Full graphical Pong game in a real window
- Proper window with pixel-perfect graphics
- Real-time 2D rendering using minifb
- Complete game with menu, gameplay, and game over states
- Smooth paddle and ball movement

**How to run:**
```bash
cargo run --bin window_pong
```

**What you'll see:**
- A 800x600 window opens with the Pong game
- Main menu with title and instructions
- Gameplay with colored paddles and ball
- Real-time score display
- Game over screen with winner announcement

## Controls

### All Demos
- **ESC**: Exit or pause/resume
- **Q**: Quit (where applicable)

### Pong Games
- **W/S**: Move player paddle up/down
- **SPACE**: Start game / Return to menu
- **ESC**: Pause during gameplay

## Demo Features Showcase

### ECS Demo
- ✅ Entity creation and management
- ✅ Component-based architecture
- ✅ System execution pipeline
- ✅ Real-time data updates

### Physics Demo
- ✅ Realistic ball physics
- ✅ Collision detection
- ✅ Force application
- ✅ Material properties

### Simple Graphical Pong
- ✅ Complete game implementation
- ✅ Terminal-based graphics
- ✅ Game state management
- ✅ Input handling

### Window Pong
- ✅ Real window with graphics
- ✅ 2D rendering pipeline
- ✅ Pixel-perfect graphics
- ✅ Complete game experience

## System Integration

Each demo showcases different aspects of the modular engine:

| Demo | ECS | Physics | Input | AI | Rendering | Audio |
|------|-----|---------|-------|----|-----------|-------|
| ecs_demo | ✅ | ✅ | ❌ | ❌ | ❌ | ❌ |
| physics_demo | ✅ | ✅ | ❌ | ❌ | ❌ | ❌ |
| simple_graphical_pong | ✅ | ✅ | ✅ | ✅ | ✅ | ❌ |
| window_pong | ✅ | ✅ | ✅ | ✅ | ✅ | ❌ |

## Performance Expectations

- **ECS Demo**: 300+ frames, 9 entities, 60+ FPS
- **Physics Demo**: Real-time physics, collision detection
- **Simple Graphical Pong**: Terminal rendering, smooth gameplay
- **Window Pong**: 800x600 window, real-time graphics

## Troubleshooting

### Common Issues

**Demo doesn't start:**
- Make sure you have the correct dependencies installed
- Check that you're in the project root directory
- Ensure `cargo` is in your PATH

**Window doesn't appear:**
- For `window_pong`, make sure you have a graphical environment
- Check that your display server is running (X11, Wayland, etc.)
- Try running in a different terminal or environment

**Performance issues:**
- The demos are unoptimized for performance benchmarking
- Close other applications for better performance
- Try running in release mode: `cargo run --release --bin demo_name`

### Getting Help

If you encounter issues:
1. Check the console output for error messages
2. Verify your Rust installation: `rustc --version`
3. Make sure dependencies are installed: `cargo check`
4. Try running a simpler demo first

## Next Steps

After running the demos, you can:

1. **Modify the code** to experiment with different mechanics
2. **Create your own games** using the modular architecture
3. **Add new systems** like audio, advanced AI, or networking
4. **Optimize performance** for your specific use cases

The modular design makes it easy to mix and match systems to create your perfect game engine!