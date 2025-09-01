# Entity Component System (ECS)

## Overview
The ECS is the core architectural pattern that powers our modular game engine. It provides a data-oriented approach to game development that emphasizes composition over inheritance.

## Core Concepts

### Entities
- **Purpose**: Unique identifiers for game objects
- **Implementation**: Simple ID types with no data
- **Usage**: Created via `World::create_entity()` and destroyed via `World::delete_entity()`

### Components
- **Purpose**: Pure data structures attached to entities
- **Characteristics**:
  - No behavior or methods (data-only)
  - Small, focused data structures
  - Can be attached/removed dynamically
- **Examples**: `Position`, `Velocity`, `Renderable`, `Health`

### Systems
- **Purpose**: Logic that operates on components
- **Characteristics**:
  - Pure functions that process data
  - Operate on collections of components
  - No side effects (except through ECS)
- **Execution**: Run in parallel when possible

## Architecture Benefits

### Performance
- **Cache-friendly**: Components stored contiguously in memory
- **Parallel processing**: Systems can run concurrently
- **Query optimization**: Efficient component iteration

### Modularity
- **Loose coupling**: Systems don't depend on each other
- **Composability**: Mix and match components freely
- **Extensibility**: Add new components/systems without breaking existing code

### Maintainability
- **Clear separation**: Data vs logic separation
- **Testability**: Systems can be unit tested independently
- **Debugging**: Easy to inspect entity state

## Usage Patterns

### Creating Entities
```rust
let player = world.create_entity()
    .with(Position { x: 0.0, y: 0.0 })
    .with(Velocity { x: 0.0, y: 0.0 })
    .with(Renderable { sprite: "player.png" })
    .build();
```

### System Implementation
```rust
impl<'a> System<'a> for PhysicsSystem {
    type SystemData = (WriteStorage<'a, Position>, ReadStorage<'a, Velocity>);

    fn run(&mut self, (mut positions, velocities): Self::SystemData) {
        for (position, velocity) in (&mut positions, &velocities).join() {
            position.x += velocity.x * delta_time;
            position.y += velocity.y * delta_time;
        }
    }
}
```

### Querying Components
```rust
// Find all entities with health below 50%
let low_health_entities: Vec<Entity> = (&healths, &entities)
    .join()
    .filter(|(health, _)| health.current < health.max * 0.5)
    .map(|(_, entity)| entity)
    .collect();
```

## Integration Points
- **Resource System**: ECS World holds global resources
- **Event System**: Systems can publish/subscribe to events
- **Plugin System**: Plugins can register new components and systems
- **Game Loop**: Systems executed in defined order each frame

## Best Practices
1. Keep components small and focused
2. Use archetypes for common entity types
3. Prefer composition over complex inheritance
4. Profile system performance regularly
5. Document component dependencies clearly