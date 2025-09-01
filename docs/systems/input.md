# Input System

## Overview
The Input System manages all user input devices and translates raw input events into game actions. It provides a unified interface for handling keyboard, mouse, gamepad, and touch input across different platforms.

## Core Architecture

### Input Manager
Central hub for all input processing:

```rust
pub struct InputManager {
    keyboard: KeyboardState,
    mouse: MouseState,
    gamepads: HashMap<GamepadId, GamepadState>,
    actions: ActionMap,
    bindings: InputBindings,
}
```

### Input State Tracking
Maintain current and previous state for all input devices:

```rust
#[derive(Debug, Clone)]
pub struct InputState {
    pub pressed: bool,
    pub just_pressed: bool,
    pub just_released: bool,
    pub timestamp: f64,
}
```

## Input Devices

### Keyboard Input
```rust
#[derive(Debug, Clone)]
pub struct KeyboardState {
    keys: HashMap<KeyCode, InputState>,
    modifiers: ModifierState,
}

#[derive(Debug, Clone, Copy)]
pub struct ModifierState {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub super_key: bool,
}
```

### Mouse Input
```rust
#[derive(Debug, Clone)]
pub struct MouseState {
    pub position: Vec2,
    pub delta: Vec2,
    pub wheel_delta: f32,
    pub buttons: HashMap<MouseButton, InputState>,
}
```

### Gamepad Input
```rust
#[derive(Debug, Clone)]
pub struct GamepadState {
    pub connected: bool,
    pub buttons: HashMap<GamepadButton, InputState>,
    pub axes: HashMap<GamepadAxis, f32>,
    pub vibration: Option<VibrationState>,
}
```

### Touch Input
```rust
#[derive(Debug, Clone)]
pub struct TouchState {
    pub touches: HashMap<TouchId, TouchPoint>,
}

#[derive(Debug, Clone)]
pub struct TouchPoint {
    pub id: TouchId,
    pub position: Vec2,
    pub phase: TouchPhase,
}
```

## Action System

### Input Actions
Abstract game actions from specific input methods:

```rust
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Action {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Jump,
    Attack,
    Interact,
    Pause,
}
```

### Action Mapping
Map input events to game actions:

```rust
pub struct ActionMap {
    mappings: HashMap<Action, Vec<InputBinding>>,
}

impl ActionMap {
    pub fn bind(&mut self, action: Action, binding: InputBinding) {
        self.mappings.entry(action).or_insert(Vec::new()).push(binding);
    }

    pub fn get_action_state(&self, action: Action) -> ActionState {
        // Check all bindings for this action
    }
}
```

### Input Bindings
Flexible binding system supporting multiple input types:

```rust
pub enum InputBinding {
    Key(KeyCode),
    MouseButton(MouseButton),
    GamepadButton(GamepadButton),
    GamepadAxis(GamepadAxis, AxisDirection),
    MouseWheel(MouseWheelDirection),
    TouchGesture(TouchGesture),
}
```

## Advanced Features

### Input Buffering
Store input events for processing on the next frame:

```rust
pub struct InputBuffer {
    events: VecDeque<InputEvent>,
    max_size: usize,
}

impl InputBuffer {
    pub fn push_event(&mut self, event: InputEvent) {
        if self.events.len() >= self.max_size {
            self.events.pop_front();
        }
        self.events.push_back(event);
    }
}
```

### Input Sequences
Detect complex input combinations:

```rust
pub struct InputSequence {
    pub inputs: Vec<InputBinding>,
    pub time_window: f32,
    pub allow_repeats: bool,
}

impl InputSequence {
    pub fn check_sequence(&self, input_history: &[InputEvent]) -> bool {
        // Check if sequence was performed within time window
    }
}
```

### Dead Zones and Sensitivity
Handle analog input properly:

```rust
#[derive(Debug, Clone)]
pub struct AnalogConfig {
    pub deadzone: f32,
    pub sensitivity: f32,
    pub invert: bool,
}

impl AnalogConfig {
    pub fn process_value(&self, raw_value: f32) -> f32 {
        let mut value = raw_value;

        // Apply deadzone
        if value.abs() < self.deadzone {
            value = 0.0;
        } else {
            value = (value - self.deadzone.signum() * self.deadzone) / (1.0 - self.deadzone);
        }

        // Apply sensitivity
        value *= self.sensitivity;

        // Apply inversion
        if self.invert {
            value = -value;
        }

        value
    }
}
```

## Platform Abstraction

### Platform-specific Input
Handle different input APIs across platforms:

```rust
pub trait InputBackend {
    fn poll_events(&mut self) -> Vec<InputEvent>;
    fn get_keyboard_state(&self) -> KeyboardState;
    fn get_mouse_state(&self) -> MouseState;
    fn get_gamepad_state(&self, id: GamepadId) -> Option<GamepadState>;
}
```

### Cross-platform Support
- **Windows**: Win32 API or winit
- **macOS**: Cocoa or winit
- **Linux**: X11, Wayland, or winit
- **Web**: Web APIs (KeyboardEvent, MouseEvent, Gamepad API)
- **Mobile**: Touch events and accelerometer

## Integration with ECS

### Input Components
```rust
#[derive(Component, Debug, Clone)]
pub struct PlayerInput {
    pub move_direction: Vec2,
    pub look_direction: Vec2,
    pub actions: HashSet<Action>,
}

#[derive(Component, Debug, Clone)]
pub struct InputControlled {
    pub player_id: u32,
}
```

### Input System
```rust
impl<'a> System<'a> for InputSystem {
    type SystemData = (
        WriteStorage<'a, PlayerInput>,
        ReadStorage<'a, InputControlled>,
    );

    fn run(&mut self, (mut player_inputs, input_controlled): Self::SystemData) {
        // Update input state
        self.input_manager.update();

        // Process input for controlled entities
        for (player_input, _) in (&mut player_inputs, &input_controlled).join() {
            *player_input = self.input_manager.get_player_input();
        }
    }
}
```

## Configuration and Customization

### Input Profiles
Save and load different input configurations:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputProfile {
    pub name: String,
    pub bindings: HashMap<Action, Vec<InputBinding>>,
    pub analog_configs: HashMap<GamepadAxis, AnalogConfig>,
}
```

### Runtime Rebinding
Allow players to customize controls:

```rust
impl InputManager {
    pub fn start_rebind(&mut self, action: Action) -> RebindState {
        // Enter rebinding mode
    }

    pub fn complete_rebind(&mut self, binding: InputBinding) {
        // Apply new binding
    }
}
```

## Best Practices

### Performance
1. Poll input at consistent intervals
2. Cache input state to avoid redundant queries
3. Use event-driven input when possible
4. Minimize input processing per frame

### User Experience
1. Provide clear visual feedback for input
2. Support multiple input methods simultaneously
3. Handle input conflicts gracefully
4. Provide sensible default bindings

### Accessibility
1. Support keyboard-only navigation
2. Allow adjustable input sensitivity
3. Provide input remapping options
4. Support screen readers and other assistive technologies

### Debugging
1. Log input events for analysis
2. Visualize input state in debug mode
3. Provide input recording/playback for testing
4. Show active bindings and conflicts

## Integration Points

### Game Systems
- **Physics**: Apply movement forces based on input
- **UI**: Handle menu navigation and selection
- **Audio**: Play sound effects for input feedback
- **Camera**: Control camera movement

### Events
- **Input Events**: Published for each input change
- **Action Events**: Published when actions are triggered
- **Binding Events**: Published when bindings change

### Persistence
- **Save/Load**: Store input preferences
- **Profiles**: Support multiple input configurations
- **Cloud Sync**: Synchronize settings across devices