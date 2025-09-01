# AI System

## Overview
The AI System manages intelligent behavior for non-player characters (NPCs), enemies, and other autonomous entities. It provides a flexible framework for implementing various AI patterns including pathfinding, decision making, and behavioral states.

## Core Architecture

### AI Controller
Central component for AI entities:

```rust
#[derive(Component, Debug, Clone)]
pub struct AIController {
    pub behavior_tree: BehaviorTree,
    pub blackboard: Blackboard,
    pub current_state: AIState,
    pub target_entity: Option<Entity>,
}
```

### Behavior Trees
Hierarchical structure for complex AI behavior:

```rust
pub enum BehaviorNode {
    Sequence(Vec<BehaviorNode>),
    Selector(Vec<BehaviorNode>),
    Action(Box<dyn AIAction>),
    Condition(Box<dyn AICondition>),
    Decorator(Box<dyn AIDecorator>, Box<BehaviorNode>),
}
```

## AI Components

### Basic AI Components
```rust
#[derive(Component, Debug, Clone)]
pub struct EnemyAI {
    pub ai_type: AIType,
    pub detection_range: f32,
    pub attack_range: f32,
    pub move_speed: f32,
}

#[derive(Component, Debug, Clone)]
pub struct NPCAI {
    pub personality: Personality,
    pub schedule: Schedule,
    pub relationships: HashMap<Entity, Relationship>,
}
```

### Pathfinding Components
```rust
#[derive(Component, Debug, Clone)]
pub struct PathFollower {
    pub path: Vec<Vec2>,
    pub current_waypoint: usize,
    pub speed: f32,
    pub tolerance: f32,
}

#[derive(Component, Debug, Clone)]
pub struct NavigationMesh {
    pub nodes: Vec<NavNode>,
    pub edges: Vec<NavEdge>,
}
```

## Decision Making

### Finite State Machines
Simple state-based AI:

```rust
#[derive(Debug, Clone)]
pub enum AIState {
    Idle,
    Patrol { waypoints: Vec<Vec2>, current: usize },
    Chase { target: Entity },
    Attack { target: Entity },
    Flee { from: Entity },
    Dead,
}

impl AIState {
    pub fn update(&self, context: &AIContext) -> AIState {
        match self {
            AIState::Idle => {
                if context.can_see_player() {
                    AIState::Chase { target: context.player_entity }
                } else {
                    AIState::Patrol { waypoints: context.patrol_points.clone(), current: 0 }
                }
            }
            // ... other state transitions
        }
    }
}
```

### Goal-Oriented Action Planning (GOAP)
Complex decision making with planning:

```rust
#[derive(Debug, Clone)]
pub struct GOAPAction {
    pub name: String,
    pub cost: f32,
    pub preconditions: HashMap<String, bool>,
    pub effects: HashMap<String, bool>,
}

#[derive(Debug, Clone)]
pub struct GOAPPlanner {
    pub actions: Vec<GOAPAction>,
    pub current_plan: Vec<GOAPAction>,
}

impl GOAPPlanner {
    pub fn plan(&self, current_state: &WorldState, goal: &WorldState) -> Option<Vec<GOAPAction>> {
        // A* search through action space
    }
}
```

## Pathfinding

### A* Pathfinding
Grid-based pathfinding:

```rust
pub struct AStarPathfinder {
    grid: Grid,
    heuristic: Box<dyn Heuristic>,
}

impl Pathfinder for AStarPathfinder {
    fn find_path(&self, start: Vec2, goal: Vec2) -> Option<Vec<Vec2>> {
        // A* algorithm implementation
    }
}
```

### Navigation Meshes
Advanced pathfinding for complex environments:

```rust
pub struct NavMesh {
    triangles: Vec<NavTriangle>,
    edges: Vec<NavEdge>,
}

impl NavMesh {
    pub fn find_path(&self, start: Vec2, goal: Vec2) -> Option<Vec<Vec2>> {
        // NavMesh pathfinding
    }
}
```

### Steering Behaviors
Local movement behaviors:

```rust
pub enum SteeringBehavior {
    Seek(Vec2),
    Flee(Vec2),
    Pursue(Entity),
    Evade(Entity),
    Wander,
    FollowPath(Vec<Vec2>),
    Separation(Vec<Entity>),
    Alignment(Vec<Entity>),
    Cohesion(Vec<Entity>),
}
```

## Sensory Systems

### Vision
```rust
#[derive(Component, Debug, Clone)]
pub struct Vision {
    pub range: f32,
    pub angle: f32,
    pub facing: Vec2,
}

impl Vision {
    pub fn can_see(&self, position: Vec2, target: Vec2) -> bool {
        let distance = (target - position).magnitude();
        if distance > self.range {
            return false;
        }

        let direction = (target - position).normalize();
        let angle = direction.angle_between(self.facing);
        angle.abs() <= self.angle / 2.0
    }
}
```

### Hearing
```rust
#[derive(Component, Debug, Clone)]
pub struct Hearing {
    pub range: f32,
    pub sensitivity: f32,
}

impl Hearing {
    pub fn can_hear(&self, position: Vec2, sound_position: Vec2, volume: f32) -> bool {
        let distance = (sound_position - position).magnitude();
        let effective_volume = volume / (distance * distance);
        effective_volume >= self.sensitivity
    }
}
```

## Group AI

### Flocking
Coordinated group movement:

```rust
pub struct Flock {
    pub members: Vec<Entity>,
    pub cohesion_weight: f32,
    pub separation_weight: f32,
    pub alignment_weight: f32,
}

impl Flock {
    pub fn update(&mut self, positions: &ReadStorage<Position>, velocities: &ReadStorage<Velocity>) {
        // Apply flocking rules
    }
}
```

### Squad Tactics
Coordinated combat behavior:

```rust
pub struct Squad {
    pub leader: Entity,
    pub members: Vec<Entity>,
    pub formation: Formation,
    pub tactics: Tactics,
}

impl Squad {
    pub fn execute_tactic(&self, tactic: Tactic, context: &AIContext) {
        // Execute squad-level behavior
    }
}
```

## Learning and Adaptation

### Behavior Learning
Simple reinforcement learning:

```rust
pub struct QLearning {
    pub q_table: HashMap<(AIState, Action), f32>,
    pub learning_rate: f32,
    pub discount_factor: f32,
    pub exploration_rate: f32,
}

impl QLearning {
    pub fn learn(&mut self, state: AIState, action: Action, reward: f32, next_state: AIState) {
        // Update Q-values
    }
}
```

### Dynamic Difficulty
Adjust AI behavior based on player performance:

```rust
pub struct DynamicDifficulty {
    pub player_skill: f32,
    pub ai_adjustments: HashMap<String, f32>,
}

impl DynamicDifficulty {
    pub fn adjust_ai(&self, ai_entity: Entity, world: &World) {
        // Modify AI parameters based on player skill
    }
}
```

## Integration with ECS

### AI System
```rust
impl<'a> System<'a> for AISystem {
    type SystemData = (
        WriteStorage<'a, AIController>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Velocity>,
        WriteStorage<'a, PathFollower>,
    );

    fn run(&mut self, (mut ai_controllers, positions, velocities, mut path_followers): Self::SystemData) {
        for (ai_controller, position, velocity, path_follower) in
            (&mut ai_controllers, &positions, &velocities, &mut path_followers).join() {

            // Update AI behavior
            ai_controller.update(position, velocity, path_follower);
        }
    }
}
```

## Performance Optimizations

### Spatial Partitioning
Use spatial data structures for efficient AI queries:

```rust
pub struct AISpatialIndex {
    quadtree: QuadTree<Entity>,
}

impl AISpatialIndex {
    pub fn find_nearby_entities(&self, position: Vec2, radius: f32) -> Vec<Entity> {
        // Query entities within radius
    }
}
```

### AI LOD
Reduce AI complexity for distant entities:

```rust
pub enum AIComplexity {
    Full,      // Full AI processing
    Simplified, // Basic behaviors only
    Dormant,   // Minimal processing
}

impl AIComplexity {
    pub fn based_on_distance(distance: f32) -> Self {
        if distance < 50.0 {
            AIComplexity::Full
        } else if distance < 200.0 {
            AIComplexity::Simplified
        } else {
            AIComplexity::Dormant
        }
    }
}
```

## Best Practices

### Design
1. Start with simple state machines
2. Use behavior trees for complex NPCs
3. Implement sensory systems for realistic behavior
4. Balance AI challenge with player enjoyment

### Performance
1. Use spatial partitioning for large worlds
2. Implement AI level of detail
3. Cache pathfinding results when possible
4. Profile AI performance regularly

### Debugging
1. Visualize AI state and decision making
2. Log AI behavior for analysis
3. Implement AI debugging tools
4. Create unit tests for AI components

### Balancing
1. Playtest AI behavior extensively
2. Adjust parameters based on player feedback
3. Implement dynamic difficulty when appropriate
4. Consider different skill levels

## Integration Points

### Game Systems
- **Physics**: Use physics for movement and collision
- **Rendering**: Visualize AI state for debugging
- **Audio**: Play AI-related sound effects
- **UI**: Show AI information in debug mode

### Events
- **AI Events**: Published for AI state changes
- **Combat Events**: Trigger AI responses
- **World Events**: Affect AI behavior

### Persistence
- **Save/Load**: Store AI state
- **Procedural**: Generate varied AI behavior
- **Progression**: AI improves with game progression