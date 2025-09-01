# Component System

## Overview
Components are the pure data structures that define entity behavior and appearance. Each component represents a single aspect of an entity's state, following the single responsibility principle.

## Core Component Categories

### Transform Components
Handle position, rotation, and scale in 2D/3D space.

#### Position
```rust
#[derive(Component, Debug, Clone, Copy)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32, // Optional for 2D games
}
```
- **Purpose**: World-space coordinates
- **Usage**: Required for all renderable entities
- **Systems**: Physics, Rendering, Collision

#### Rotation
```rust
#[derive(Component, Debug, Clone, Copy)]
pub struct Rotation {
    pub angle: f32, // Radians
}
```
- **Purpose**: Entity orientation
- **Usage**: For sprites that can rotate
- **Systems**: Rendering, Physics

#### Scale
```rust
#[derive(Component, Debug, Clone, Copy)]
pub struct Scale {
    pub x: f32,
    pub y: f32,
}
```
- **Purpose**: Entity size scaling
- **Usage**: For dynamic sizing effects
- **Systems**: Rendering

### Physics Components
Handle movement, collision, and physical properties.

#### Velocity
```rust
#[derive(Component, Debug, Clone, Copy)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
```
- **Purpose**: Linear movement speed and direction
- **Usage**: For moving entities
- **Systems**: Physics

#### Acceleration
```rust
#[derive(Component, Debug, Clone, Copy)]
pub struct Acceleration {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
```
- **Purpose**: Rate of velocity change
- **Usage**: For thrust, gravity, friction
- **Systems**: Physics

#### Mass
```rust
#[derive(Component, Debug, Clone, Copy)]
pub struct Mass {
    pub value: f32,
}
```
- **Purpose**: Physical mass for realistic physics
- **Usage**: For collision responses and momentum
- **Systems**: Physics

### Rendering Components
Control visual appearance and rendering properties.

#### Renderable
```rust
#[derive(Component, Debug, Clone)]
pub struct Renderable {
    pub sprite_id: String,
    pub layer: i32,
    pub visible: bool,
}
```
- **Purpose**: Basic rendering information
- **Usage**: Required for all visible entities
- **Systems**: Rendering

#### Sprite
```rust
#[derive(Component, Debug, Clone)]
pub struct Sprite {
    pub texture_id: String,
    pub frame: usize,
    pub animation_speed: f32,
}
```
- **Purpose**: Animated sprite information
- **Usage**: For animated characters/objects
- **Systems**: Rendering, Animation

#### Color
```rust
#[derive(Component, Debug, Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
```
- **Purpose**: Tint and transparency
- **Usage**: For visual effects and states
- **Systems**: Rendering

### Gameplay Components
Define game-specific behavior and state.

#### Health
```rust
#[derive(Component, Debug, Clone, Copy)]
pub struct Health {
    pub current: f32,
    pub maximum: f32,
}
```
- **Purpose**: Entity vitality
- **Usage**: For damageable entities
- **Systems**: Combat, UI

#### Player
```rust
#[derive(Component, Debug, Clone)]
pub struct Player {
    pub id: u32,
    pub name: String,
}
```
- **Purpose**: Player-specific data
- **Usage**: For player entities
- **Systems**: Input, UI, Save/Load

#### Enemy
```rust
#[derive(Component, Debug, Clone)]
pub struct Enemy {
    pub enemy_type: EnemyType,
    pub difficulty: f32,
}
```
- **Purpose**: Enemy-specific data
- **Usage**: For AI-controlled enemies
- **Systems**: AI, Combat

### Collision Components
Handle collision detection and response.

#### Collider
```rust
#[derive(Component, Debug, Clone)]
pub struct Collider {
    pub shape: CollisionShape,
    pub is_trigger: bool,
}
```
- **Purpose**: Collision boundaries
- **Usage**: For physical interactions
- **Systems**: Physics, Collision

#### CollisionShape
```rust
pub enum CollisionShape {
    Circle { radius: f32 },
    Rectangle { width: f32, height: f32 },
    Polygon { vertices: Vec<Point> },
}
```
- **Purpose**: Different collision geometries
- **Usage**: Flexible collision detection
- **Systems**: Collision

## Component Organization

### Module Structure
```
src/components/
├── mod.rs
├── transform.rs     // Position, Rotation, Scale
├── physics.rs       // Velocity, Acceleration, Mass
├── rendering.rs     // Renderable, Sprite, Color
├── gameplay.rs      // Health, Player, Enemy
└── collision.rs     // Collider, CollisionShape
```

### Registration
All components must be registered with the ECS World:
```rust
world.register::<Position>();
world.register::<Velocity>();
world.register::<Renderable>();
// ... register all components
```

## Best Practices

### Design Guidelines
1. **Single Responsibility**: Each component should represent one concept
2. **Minimal Data**: Only include necessary fields
3. **Copy vs Clone**: Use Copy for small, frequently accessed components
4. **Optional Fields**: Use Option<T> for optional properties

### Performance Considerations
1. **Memory Layout**: Group frequently accessed components together
2. **Archetype Awareness**: Consider how components combine
3. **Cache Efficiency**: Keep hot data contiguous

### Usage Patterns
1. **Marker Components**: Empty structs for tagging (e.g., `Dead`, `Paused`)
2. **Resource Components**: Components that reference shared resources
3. **State Components**: Components that change frequently during gameplay

## Integration Points
- **Systems**: Components are processed by systems
- **Resources**: Some components may reference global resources
- **Serialization**: Components need to be serializable for save/load
- **Networking**: Components may need network synchronization