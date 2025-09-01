# Physics System

## Overview
The Physics System manages all physical interactions in the game world, including movement, collision detection, and force application. It operates on entities with physics-related components to create realistic and responsive gameplay.

## Core Responsibilities

### Movement Integration
Updates entity positions based on velocity and acceleration over time.

#### Velocity Integration
```rust
// Position += Velocity * DeltaTime
position.x += velocity.x * delta_time;
position.y += velocity.y * delta_time;
```

#### Acceleration Integration
```rust
// Velocity += Acceleration * DeltaTime
velocity.x += acceleration.x * delta_time;
velocity.y += acceleration.y * delta_time;
```

### Force Application
Applies various forces to entities for realistic movement.

#### Gravity
```rust
#[derive(Debug, Clone)]
pub struct Gravity {
    pub force: Vec2,
}

impl Default for Gravity {
    fn default() -> Self {
        Self {
            force: Vec2::new(0.0, -9.81), // Earth gravity
        }
    }
}
```

#### Friction
```rust
#[derive(Component, Debug, Clone, Copy)]
pub struct Friction {
    pub coefficient: f32,
}
```

#### Damping
```rust
#[derive(Component, Debug, Clone, Copy)]
pub struct Damping {
    pub linear: f32,
    pub angular: f32,
}
```

## Collision Detection

### Broad Phase
Initial filtering to reduce collision checks.

#### Spatial Partitioning
- **Grid-based**: Divide space into uniform cells
- **Quadtree**: Hierarchical space division
- **Sweep and Prune**: Sort entities by position

#### Implementation
```rust
pub struct BroadPhase {
    grid: SpatialGrid,
    cell_size: f32,
}

impl BroadPhase {
    pub fn find_potential_collisions(&self, entities: &[Entity]) -> Vec<(Entity, Entity)> {
        // Return pairs that might collide
    }
}
```

### Narrow Phase
Precise collision detection for potential pairs.

#### Shape-based Collision
- **Circle vs Circle**: Distance check
- **Rectangle vs Rectangle**: AABB overlap
- **Circle vs Rectangle**: Point-in-rectangle + distance
- **Polygon vs Polygon**: SAT (Separating Axis Theorem)

#### Collision Response
```rust
pub struct Collision {
    pub entity_a: Entity,
    pub entity_b: Entity,
    pub normal: Vec2,
    pub penetration: f32,
    pub contact_point: Vec2,
}
```

## Physics World

### Configuration
```rust
#[derive(Debug, Clone)]
pub struct PhysicsConfig {
    pub gravity: Vec2,
    pub time_step: f32,
    pub velocity_iterations: u32,
    pub position_iterations: u32,
    pub allow_sleep: bool,
}
```

### World State
```rust
pub struct PhysicsWorld {
    config: PhysicsConfig,
    broad_phase: BroadPhase,
    narrow_phase: NarrowPhase,
    constraints: Vec<Box<dyn Constraint>>,
}
```

## Integration with ECS

### Required Components
- `Position` - Current location
- `Velocity` - Movement speed/direction
- `Acceleration` - Rate of velocity change
- `Mass` - Physical mass (optional)
- `Collider` - Collision shape

### System Implementation
```rust
impl<'a> System<'a> for PhysicsSystem {
    type SystemData = (
        WriteStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
        ReadStorage<'a, Acceleration>,
        ReadStorage<'a, Mass>,
        ReadStorage<'a, Collider>,
    );

    fn run(&mut self, (mut positions, mut velocities, accelerations, masses, colliders): Self::SystemData) {
        // 1. Apply forces (gravity, friction, etc.)
        self.apply_forces(&mut velocities, &accelerations, &masses);

        // 2. Integrate motion
        self.integrate_motion(&mut positions, &velocities);

        // 3. Detect collisions
        let collisions = self.detect_collisions(&positions, &colliders);

        // 4. Resolve collisions
        self.resolve_collisions(&mut positions, &mut velocities, &collisions);
    }
}
```

## Advanced Features

### Joints and Constraints
Connect entities with physical relationships.

#### Distance Joint
```rust
pub struct DistanceJoint {
    pub entity_a: Entity,
    pub entity_b: Entity,
    pub target_distance: f32,
    pub stiffness: f32,
}
```

#### Revolute Joint
```rust
pub struct RevoluteJoint {
    pub entity_a: Entity,
    pub entity_b: Entity,
    pub anchor: Vec2,
    pub motor_speed: f32,
}
```

### Raycasting
Query the physics world for intersections.

```rust
pub struct RaycastResult {
    pub hit_entity: Option<Entity>,
    pub hit_point: Vec2,
    pub hit_normal: Vec2,
    pub distance: f32,
}

impl PhysicsWorld {
    pub fn raycast(&self, origin: Vec2, direction: Vec2, max_distance: f32) -> Option<RaycastResult> {
        // Implementation
    }
}
```

## Performance Optimizations

### Spatial Queries
- Use spatial hashing for fast neighbor queries
- Implement object culling for off-screen entities
- Cache collision pairs when possible

### Multithreading
- Parallel broad phase collision detection
- Concurrent narrow phase for independent pairs
- Async physics simulation for complex scenes

### Sleeping
- Put inactive objects to sleep to save CPU
- Wake objects when they interact with active objects
- Configurable sleep thresholds

## Integration Points

### Game Systems
- **Rendering**: Uses physics positions for drawing
- **Input**: Applies forces based on player input
- **AI**: Queries physics for pathfinding and navigation
- **Audio**: Triggers sounds based on collisions

### Events
- **Collision Events**: Published when entities collide
- **Trigger Events**: Published when entities enter trigger volumes
- **Physics Events**: Published for significant physics changes

## Best Practices

### Performance
1. Use simple collision shapes when possible
2. Implement object layers to reduce collision checks
3. Profile physics performance regularly
4. Use fixed time steps for consistent simulation

### Stability
1. Clamp velocities to prevent instability
2. Use warm starting for iterative solvers
3. Implement position correction for overlap resolution
4. Handle edge cases (zero mass, infinite forces)

### Debugging
1. Visualize collision shapes and normals
2. Log collision events for analysis
3. Implement physics debugging tools
4. Use slow-motion for complex interactions