# Event System

## Overview
The Event System provides a decoupled communication mechanism for all game systems. It enables loose coupling between components, systems, and plugins by allowing them to publish and subscribe to events without direct dependencies.

## Core Architecture

### Event Bus
Central event distribution hub:

```rust
pub struct EventBus {
    subscribers: HashMap<EventType, Vec<EventSubscription>>,
    event_queue: VecDeque<Event>,
    processing: bool,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum EventType {
    Game,
    Input,
    Physics,
    Rendering,
    Audio,
    UI,
    Custom(String),
}
```

### Event Structure
Standardized event format:

```rust
#[derive(Debug, Clone)]
pub struct Event {
    pub id: EventId,
    pub event_type: EventType,
    pub timestamp: SystemTime,
    pub data: EventData,
    pub source: Option<Entity>,
}

#[derive(Debug, Clone)]
pub enum EventData {
    Game(GameEvent),
    Input(InputEvent),
    Physics(PhysicsEvent),
    // ... other event types
    Custom(serde_json::Value),
}
```

## Event Types

### Game Events
Core game state events:

```rust
#[derive(Debug, Clone)]
pub enum GameEvent {
    GameStarted,
    GamePaused,
    GameResumed,
    GameEnded { score: u32 },
    LevelLoaded { level_id: String },
    PlayerDied,
    ScoreChanged { new_score: u32 },
}
```

### Input Events
User input notifications:

```rust
#[derive(Debug, Clone)]
pub enum InputEvent {
    KeyPressed { key: KeyCode },
    KeyReleased { key: KeyCode },
    MouseMoved { position: Vec2, delta: Vec2 },
    MouseButtonPressed { button: MouseButton },
    MouseButtonReleased { button: MouseButton },
    GamepadConnected { id: GamepadId },
    GamepadDisconnected { id: GamepadId },
}
```

### Physics Events
Collision and physics notifications:

```rust
#[derive(Debug, Clone)]
pub enum PhysicsEvent {
    Collision {
        entity_a: Entity,
        entity_b: Entity,
        normal: Vec2,
        impulse: f32,
    },
    TriggerEntered {
        entity: Entity,
        trigger: Entity,
    },
    TriggerExited {
        entity: Entity,
        trigger: Entity,
    },
}
```

### Rendering Events
Visual system notifications:

```rust
#[derive(Debug, Clone)]
pub enum RenderingEvent {
    CameraChanged { camera: Entity },
    LightAdded { light: Entity },
    LightRemoved { light: Entity },
    MaterialChanged { entity: Entity },
}
```

## Event Subscription

### Subscription Types
Different ways to subscribe to events:

```rust
pub enum EventSubscription {
    Function(Box<dyn Fn(&Event)>),
    System(Entity),
    Plugin(String),
}

impl EventBus {
    pub fn subscribe<F>(&mut self, event_type: EventType, callback: F)
    where
        F: Fn(&Event) + 'static,
    {
        let subscription = EventSubscription::Function(Box::new(callback));
        self.subscribers
            .entry(event_type)
            .or_insert(Vec::new())
            .push(subscription);
    }

    pub fn subscribe_system(&mut self, event_type: EventType, system_entity: Entity) {
        let subscription = EventSubscription::System(system_entity);
        self.subscribers
            .entry(event_type)
            .or_insert(Vec::new())
            .push(subscription);
    }
}
```

### Filtered Subscriptions
Subscribe to specific event conditions:

```rust
pub struct EventFilter {
    pub event_type: EventType,
    pub conditions: Vec<EventCondition>,
}

pub enum EventCondition {
    SourceEntity(Entity),
    TargetEntity(Entity),
    Custom(Box<dyn Fn(&Event) -> bool>),
}

impl EventBus {
    pub fn subscribe_filtered<F>(&mut self, filter: EventFilter, callback: F)
    where
        F: Fn(&Event) + 'static,
    {
        let filtered_callback = move |event: &Event| {
            if filter.matches(event) {
                callback(event);
            }
        };

        self.subscribe(filter.event_type.clone(), filtered_callback);
    }
}

impl EventFilter {
    pub fn matches(&self, event: &Event) -> bool {
        self.conditions.iter().all(|condition| condition.check(event))
    }
}
```

## Event Publishing

### Synchronous Publishing
Immediate event processing:

```rust
impl EventBus {
    pub fn publish(&mut self, event: Event) {
        self.event_queue.push_back(event);

        if !self.processing {
            self.process_events();
        }
    }

    fn process_events(&mut self) {
        self.processing = true;

        while let Some(event) = self.event_queue.pop_front() {
            if let Some(subscribers) = self.subscribers.get(&event.event_type) {
                for subscriber in subscribers {
                    subscriber.handle_event(&event);
                }
            }
        }

        self.processing = false;
    }
}
```

### Asynchronous Publishing
Queued event processing:

```rust
pub struct AsyncEventBus {
    sender: mpsc::Sender<Event>,
    receiver: mpsc::Receiver<Event>,
    subscribers: HashMap<EventType, Vec<EventSubscription>>,
}

impl AsyncEventBus {
    pub fn publish_async(&self, event: Event) {
        let _ = self.sender.send(event);
    }

    pub fn process_async_events(&mut self) {
        while let Ok(event) = self.receiver.try_recv() {
            if let Some(subscribers) = self.subscribers.get(&event.event_type) {
                for subscriber in subscribers {
                    subscriber.handle_event(&event);
                }
            }
        }
    }
}
```

## Event Handling

### Event Handlers
Process events in systems:

```rust
impl<'a> System<'a> for EventHandlerSystem {
    type SystemData = (
        Read<'a, EventBus>,
        WriteStorage<'a, Health>,
        WriteStorage<'a, Score>,
    );

    fn run(&mut self, (event_bus, mut healths, mut scores): Self::SystemData) {
        for event in event_bus.get_events() {
            match &event.data {
                EventData::Game(GameEvent::PlayerDied) => {
                    // Handle player death
                }
                EventData::Physics(PhysicsEvent::Collision { entity_a, entity_b, .. }) => {
                    // Handle collision
                }
                _ => {}
            }
        }
    }
}
```

### Event Chains
Events can trigger other events:

```rust
impl EventBus {
    pub fn publish_chain(&mut self, initial_event: Event) {
        let mut current_event = initial_event;
        let mut chain_depth = 0;

        loop {
            if chain_depth >= MAX_CHAIN_DEPTH {
                warn!("Event chain too deep, breaking");
                break;
            }

            self.publish(current_event.clone());

            // Check if this event should trigger another
            if let Some(next_event) = self.get_chained_event(&current_event) {
                current_event = next_event;
                chain_depth += 1;
            } else {
                break;
            }
        }
    }
}
```

## Event Persistence

### Event Logging
Record events for debugging and replay:

```rust
pub struct EventLogger {
    log_file: File,
    enabled: bool,
}

impl EventLogger {
    pub fn log_event(&mut self, event: &Event) {
        if self.enabled {
            let log_entry = serde_json::to_string(event).unwrap();
            writeln!(self.log_file, "{}", log_entry).unwrap();
        }
    }
}
```

### Event Replay
Replay recorded events:

```rust
pub struct EventReplay {
    events: Vec<Event>,
    current_index: usize,
    speed: f32,
}

impl EventReplay {
    pub fn load_from_file(&mut self, path: &Path) -> Result<(), ReplayError> {
        let file = File::open(path)?;
        let events: Vec<Event> = serde_json::from_reader(file)?;
        self.events = events;
        self.current_index = 0;
        Ok(())
    }

    pub fn update(&mut self, delta_time: f32, event_bus: &mut EventBus) {
        let events_to_process = (delta_time * self.speed) as usize;

        for _ in 0..events_to_process {
            if self.current_index < self.events.len() {
                let event = &self.events[self.current_index];
                event_bus.publish(event.clone());
                self.current_index += 1;
            }
        }
    }
}
```

## Performance Optimizations

### Event Pooling
Reuse event objects:

```rust
pub struct EventPool {
    pool: Vec<Event>,
    available: Vec<usize>,
}

impl EventPool {
    pub fn get_event(&mut self, event_type: EventType, data: EventData) -> Event {
        let event = if let Some(index) = self.available.pop() {
            let mut event = self.pool.swap_remove(index);
            event.event_type = event_type;
            event.data = data;
            event.timestamp = SystemTime::now();
            event
        } else {
            Event::new(event_type, data)
        };

        event
    }

    pub fn return_event(&mut self, event: Event) {
        self.available.push(self.pool.len());
        self.pool.push(event);
    }
}
```

### Subscriber Culling
Remove inactive subscribers:

```rust
impl EventBus {
    pub fn cull_subscribers(&mut self) {
        for subscribers in self.subscribers.values_mut() {
            subscribers.retain(|subscriber| subscriber.is_active());
        }

        // Remove empty subscriber lists
        self.subscribers.retain(|_, subscribers| !subscribers.is_empty());
    }
}
```

## Best Practices

### Design
1. Use specific event types over generic ones
2. Include relevant context in event data
3. Keep events immutable
4. Document event schemas

### Performance
1. Use event pooling for frequent events
2. Implement subscriber culling
3. Consider event batching for high-frequency events
4. Profile event system performance

### Debugging
1. Log important events
2. Implement event visualization
3. Use event breakpoints in debugger
4. Monitor event queue size

### Testing
1. Test event publishing and subscription
2. Verify event data integrity
3. Test event chains and dependencies
4. Mock event bus for unit tests

## Integration Points

### Game Systems
- **ECS**: Systems can publish and subscribe to events
- **Physics**: Collision events trigger responses
- **UI**: User interactions generate events
- **Audio**: Events trigger sound playback

### Plugins
- **Communication**: Plugins use events to communicate
- **Extensibility**: New event types can be added
- **Isolation**: Events provide clean plugin boundaries

### Networking
- **Synchronization**: Events can be networked
- **Prediction**: Client-side event prediction
- **Reconciliation**: Server authoritative events

### Persistence
- **Recording**: Events can be recorded for replay
- **Analytics**: Event data for game analytics
- **Save/Load**: Events can trigger state serialization