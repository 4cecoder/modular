# UI System

## Overview
The UI System manages all user interface elements including menus, HUD, dialogs, and interactive components. It provides a flexible, component-based UI framework that integrates seamlessly with the ECS architecture.

## Core Architecture

### UI Manager
Central hub for UI management:

```rust
pub struct UIManager {
    root: UIElement,
    input_handler: UIInputHandler,
    renderer: UIRenderer,
    style_manager: StyleManager,
    layout_engine: LayoutEngine,
}
```

### UI Elements
Hierarchical UI component system:

```rust
pub struct UIElement {
    pub id: String,
    pub component: Box<dyn UIComponent>,
    pub children: Vec<UIElement>,
    pub transform: UITransform,
    pub style: UIStyle,
    pub state: UIState,
}
```

## UI Components

### Basic Components
```rust
pub trait UIComponent {
    fn render(&self, renderer: &mut UIRenderer, style: &UIStyle);
    fn handle_input(&mut self, input: &UIInput) -> UIEvent;
    fn layout(&mut self, constraints: LayoutConstraints) -> LayoutResult;
}
```

### Common UI Components
```rust
pub struct Button {
    pub text: String,
    pub on_click: Option<Box<dyn Fn()>>,
    pub disabled: bool,
}

pub struct TextField {
    pub text: String,
    pub placeholder: String,
    pub max_length: Option<usize>,
    pub on_change: Option<Box<dyn Fn(String)>>,
}

pub struct Slider {
    pub value: f32,
    pub min: f32,
    pub max: f32,
    pub step: f32,
    pub on_change: Option<Box<dyn Fn(f32)>>,
}

pub struct Panel {
    pub title: Option<String>,
    pub content: Vec<UIElement>,
    pub scrollable: bool,
}
```

### Layout Components
```rust
pub struct Container {
    pub direction: LayoutDirection,
    pub alignment: Alignment,
    pub spacing: f32,
    pub children: Vec<UIElement>,
}

pub enum LayoutDirection {
    Horizontal,
    Vertical,
}

pub struct Grid {
    pub columns: usize,
    pub rows: usize,
    pub cell_size: Vec2,
    pub children: Vec<UIElement>,
}
```

## Styling System

### Style Definition
```rust
#[derive(Debug, Clone)]
pub struct UIStyle {
    pub background: Option<Background>,
    pub border: Option<Border>,
    pub text: Option<TextStyle>,
    pub spacing: Spacing,
    pub size: Size,
    pub position: Position,
}

#[derive(Debug, Clone)]
pub struct Background {
    pub color: Color,
    pub image: Option<String>,
}

#[derive(Debug, Clone)]
pub struct TextStyle {
    pub font: String,
    pub size: f32,
    pub color: Color,
    pub alignment: TextAlignment,
}
```

### Style Sheets
Cascading style system:

```rust
pub struct StyleSheet {
    pub rules: Vec<StyleRule>,
}

pub struct StyleRule {
    pub selector: UISelector,
    pub style: UIStyle,
}

pub enum UISelector {
    Element(String),      // By element ID
    Class(String),         // By CSS class
    State(UIState),        // By element state
    Type(String),          // By component type
}
```

## Layout System

### Layout Engine
Flexible layout computation:

```rust
pub struct LayoutEngine {
    pub algorithm: Box<dyn LayoutAlgorithm>,
}

pub trait LayoutAlgorithm {
    fn compute_layout(&self, element: &mut UIElement, constraints: LayoutConstraints);
}

pub struct LayoutConstraints {
    pub min_width: f32,
    pub max_width: f32,
    pub min_height: f32,
    pub max_height: f32,
}
```

### Layout Types
```rust
pub enum LayoutType {
    Flow,           // Elements flow in direction
    Flex,           // Flexible box layout
    Grid,           // Grid-based layout
    Absolute,       // Absolute positioning
    Dock,           // Dock to container edges
}
```

## Input Handling

### UI Input System
```rust
pub struct UIInputHandler {
    pub focused_element: Option<String>,
    pub hovered_element: Option<String>,
    pub drag_state: Option<DragState>,
}

impl UIInputHandler {
    pub fn process_input(&mut self, input: &InputState, ui_tree: &mut UIElement) -> Vec<UIEvent> {
        // Handle mouse/keyboard input
        // Update focus and hover states
        // Generate UI events
    }
}
```

### UI Events
```rust
pub enum UIEvent {
    Click { element_id: String },
    Hover { element_id: String },
    Focus { element_id: String },
    Blur { element_id: String },
    Change { element_id: String, value: UIValue },
    Submit { element_id: String },
}
```

## Animation System

### UI Animations
```rust
pub struct UIAnimation {
    pub property: AnimatedProperty,
    pub from: f32,
    pub to: f32,
    pub duration: f32,
    pub easing: EasingFunction,
    pub delay: f32,
}

pub enum AnimatedProperty {
    Position,
    Size,
    Opacity,
    Scale,
    Rotation,
}

pub enum EasingFunction {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    Bounce,
    Elastic,
}
```

### Animation Controller
```rust
pub struct AnimationController {
    pub animations: HashMap<String, Vec<UIAnimation>>,
    pub active_animations: Vec<ActiveAnimation>,
}

impl AnimationController {
    pub fn start_animation(&mut self, element_id: &str, animation_name: &str) {
        // Start animation for element
    }

    pub fn update(&mut self, delta_time: f32) {
        // Update active animations
    }
}
```

## UI States

### Element States
```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UIState {
    Normal,
    Hovered,
    Pressed,
    Disabled,
    Focused,
    Selected,
}
```

### State Management
```rust
pub struct StateManager {
    pub element_states: HashMap<String, UIState>,
}

impl StateManager {
    pub fn set_state(&mut self, element_id: &str, state: UIState) {
        self.element_states.insert(element_id.to_string(), state);
    }

    pub fn get_state(&self, element_id: &str) -> UIState {
        self.element_states.get(element_id).copied().unwrap_or(UIState::Normal)
    }
}
```

## Integration with ECS

### UI Components
```rust
#[derive(Component, Debug, Clone)]
pub struct UIElementComponent {
    pub element_id: String,
    pub visible: bool,
    pub interactive: bool,
}

#[derive(Component, Debug, Clone)]
pub struct UICanvas {
    pub size: Vec2,
    pub camera: Entity,
}
```

### UI System
```rust
impl<'a> System<'a> for UISystem {
    type SystemData = (
        ReadStorage<'a, UIElementComponent>,
        WriteStorage<'a, UICanvas>,
        Read<'a, InputState>,
    );

    fn run(&mut self, (ui_elements, mut canvases, input): Self::SystemData) {
        // Update UI based on game state
        // Handle input
        // Render UI
    }
}
```

## Advanced Features

### Data Binding
Connect UI to game data:

```rust
pub struct DataBinding {
    pub source: DataSource,
    pub property: String,
    pub converter: Option<Box<dyn DataConverter>>,
}

pub enum DataSource {
    Entity(Entity),
    Resource(String),
    Global(String),
}
```

### Localization
Multi-language support:

```rust
pub struct LocalizationManager {
    pub current_language: String,
    pub translations: HashMap<String, HashMap<String, String>>,
}

impl LocalizationManager {
    pub fn get_text(&self, key: &str) -> String {
        self.translations
            .get(&self.current_language)
            .and_then(|lang| lang.get(key))
            .cloned()
            .unwrap_or_else(|| format!("[{}]", key))
    }
}
```

### Accessibility
Screen reader and keyboard navigation support:

```rust
pub struct AccessibilityInfo {
    pub label: String,
    pub description: String,
    pub role: AccessibilityRole,
    pub keyboard_shortcut: Option<String>,
}

pub enum AccessibilityRole {
    Button,
    TextField,
    Slider,
    Panel,
    Menu,
}
```

## Performance Optimizations

### UI Culling
Only render visible elements:

```rust
impl UIManager {
    pub fn cull_elements(&mut self, viewport: Rect) {
        self.root.cull_recursive(viewport);
    }
}

impl UIElement {
    pub fn cull_recursive(&mut self, viewport: Rect) {
        self.visible = self.bounds.intersects(&viewport);
        for child in &mut self.children {
            child.cull_recursive(viewport);
        }
    }
}
```

### Batching
Group similar UI elements:

```rust
pub struct UIBatch {
    pub texture_id: String,
    pub vertices: Vec<UIVertex>,
    pub indices: Vec<u32>,
}

impl UIRenderer {
    pub fn batch_elements(&self, elements: &[UIElement]) -> Vec<UIBatch> {
        // Group elements by texture/material
    }
}
```

## Best Practices

### Design
1. Keep UI hierarchy shallow
2. Use consistent styling
3. Design for multiple resolutions
4. Consider accessibility from start

### Performance
1. Implement UI culling
2. Use object pooling for dynamic elements
3. Minimize layout recalculations
4. Profile UI rendering performance

### User Experience
1. Provide clear visual feedback
2. Support keyboard navigation
3. Handle edge cases gracefully
4. Test on target devices

### Debugging
1. Visualize UI bounds and layout
2. Log UI events and state changes
3. Implement UI debugging tools
4. Test with various screen sizes

## Integration Points

### Game Systems
- **Input**: Handle UI interactions
- **Rendering**: Draw UI elements
- **Audio**: Play UI sound effects
- **Localization**: Support multiple languages

### Events
- **UI Events**: Published for user interactions
- **Game Events**: Update UI based on game state
- **System Events**: Handle window resize, etc.

### Persistence
- **Settings**: Store UI preferences
- **State**: Save UI state
- **Layout**: Remember window positions