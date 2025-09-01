# Rendering System

## Overview
The Rendering System is responsible for drawing all visual elements in the game world. It processes renderable entities, manages graphics resources, and handles various rendering techniques for optimal performance and visual quality.

## Core Architecture

### Render Pipeline
The rendering process follows a structured pipeline:

1. **Culling**: Filter out off-screen entities
2. **Sorting**: Order entities by render layer/depth
3. **Batching**: Group similar render operations
4. **Drawing**: Execute render commands
5. **Post-processing**: Apply effects and filters

### Render Commands
```rust
pub enum RenderCommand {
    DrawSprite {
        texture_id: String,
        position: Vec2,
        size: Vec2,
        rotation: f32,
        color: Color,
        layer: i32,
    },
    DrawText {
        text: String,
        font_id: String,
        position: Vec2,
        color: Color,
        size: f32,
    },
    DrawShape {
        shape: Shape,
        position: Vec2,
        color: Color,
        filled: bool,
    },
}
```

## Rendering Components

### Basic Rendering
```rust
#[derive(Component, Debug, Clone)]
pub struct Renderable {
    pub visible: bool,
    pub layer: i32,
    pub order: i32,
}
```

### Sprite Rendering
```rust
#[derive(Component, Debug, Clone)]
pub struct Sprite {
    pub texture_id: String,
    pub region: Option<Rect>, // For sprite sheets
    pub flip_horizontal: bool,
    pub flip_vertical: bool,
}
```

### Animated Sprites
```rust
#[derive(Component, Debug, Clone)]
pub struct AnimatedSprite {
    pub animation_id: String,
    pub current_frame: usize,
    pub frame_time: f32,
    pub playing: bool,
    pub loop_animation: bool,
}
```

### Particle Systems
```rust
#[derive(Component, Debug, Clone)]
pub struct ParticleEmitter {
    pub particle_type: String,
    pub emission_rate: f32,
    pub lifetime: f32,
    pub active: bool,
}
```

## Camera System

### Camera Component
```rust
#[derive(Component, Debug, Clone)]
pub struct Camera {
    pub position: Vec2,
    pub zoom: f32,
    pub rotation: f32,
    pub viewport: Rect,
    pub active: bool,
}
```

### Camera Controller
```rust
pub struct CameraController {
    pub follow_target: Option<Entity>,
    pub smoothing: f32,
    pub bounds: Option<Rect>,
}
```

### Multiple Cameras
Support for multiple cameras with different viewports:
- Main gameplay camera
- UI camera (screen space)
- Mini-map camera
- Split-screen cameras

## Rendering Techniques

### Sprite Batching
Group sprites with the same texture for efficient rendering:

```rust
pub struct SpriteBatch {
    texture_id: String,
    vertices: Vec<SpriteVertex>,
    indices: Vec<u32>,
}

impl SpriteBatch {
    pub fn add_sprite(&mut self, sprite: &Sprite, transform: &Transform) {
        // Add sprite to batch
    }

    pub fn flush(&self, renderer: &mut Renderer) {
        // Render entire batch
    }
}
```

### Texture Atlasing
Combine multiple textures into single atlas for reduced texture switches:

```rust
pub struct TextureAtlas {
    texture: Texture,
    regions: HashMap<String, Rect>,
}

impl TextureAtlas {
    pub fn get_region(&self, name: &str) -> Option<&Rect> {
        self.regions.get(name)
    }
}
```

### Layer System
Organize rendering by depth layers:

```rust
#[derive(Debug, Clone, Copy, PartialOrd, Ord, Eq, PartialEq)]
pub enum RenderLayer {
    Background = 0,
    Terrain = 1,
    Objects = 2,
    Characters = 3,
    Effects = 4,
    UI = 5,
}
```

## Advanced Features

### Lighting System
Dynamic lighting with multiple light types:

```rust
pub enum LightType {
    Point { radius: f32, intensity: f32 },
    Directional { direction: Vec2, intensity: f32 },
    Ambient { intensity: f32 },
}

#[derive(Component, Debug, Clone)]
pub struct Light {
    pub light_type: LightType,
    pub color: Color,
    pub cast_shadows: bool,
}
```

### Post-Processing Effects
Apply visual effects to the entire scene:

```rust
pub enum PostProcessEffect {
    Bloom { intensity: f32 },
    Blur { radius: f32 },
    ColorGrading { lookup_table: Texture },
    Vignette { intensity: f32 },
}
```

### Render Targets
Support for off-screen rendering:

```rust
pub struct RenderTarget {
    texture: Texture,
    framebuffer: Framebuffer,
    size: Vec2,
}

impl RenderTarget {
    pub fn begin(&self) {
        // Set as current render target
    }

    pub fn end(&self) {
        // Restore default render target
    }
}
```

## Performance Optimizations

### Culling Strategies
- **Frustum Culling**: Only render visible objects
- **Occlusion Culling**: Hide objects behind others
- **Distance Culling**: Fade out distant objects

### Level of Detail (LOD)
```rust
#[derive(Component, Debug, Clone)]
pub struct LOD {
    pub levels: Vec<LODLevel>,
    pub current_level: usize,
}

pub struct LODLevel {
    pub distance: f32,
    pub mesh: String,
    pub quality: f32,
}
```

### Instancing
Render multiple identical objects efficiently:

```rust
pub struct InstanceBuffer {
    transforms: Vec<Mat4>,
    colors: Vec<Color>,
}

impl InstanceBuffer {
    pub fn render_instanced(&self, renderer: &mut Renderer, mesh: &Mesh) {
        // Render all instances in one draw call
    }
}
```

## Integration Points

### ECS Integration
```rust
impl<'a> System<'a> for RenderingSystem {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, Renderable>,
        ReadStorage<'a, Sprite>,
        ReadStorage<'a, Camera>,
    );

    fn run(&mut self, (positions, renderables, sprites, cameras): Self::SystemData) {
        // Collect render commands
        // Sort by layer
        // Batch by texture
        // Submit to renderer
    }
}
```

### Resource Management
- **Texture Loading**: Load textures on demand
- **Font Management**: Cache font atlases
- **Shader Programs**: Precompile and cache shaders

### Event System
- **Render Events**: Notify when rendering completes
- **Texture Events**: Handle texture loading/unloading
- **Camera Events**: Respond to camera changes

## Best Practices

### Performance
1. Minimize texture switches by sorting draw calls
2. Use texture atlases for related sprites
3. Implement proper culling to reduce draw calls
4. Profile rendering performance regularly

### Visual Quality
1. Use appropriate texture filtering
2. Implement anti-aliasing when needed
3. Handle aspect ratio changes gracefully
4. Support multiple resolutions

### Debugging
1. Visualize render batches and draw calls
2. Show rendering statistics (FPS, draw calls, etc.)
3. Implement wireframe rendering mode
4. Add visual debugging for culling and LOD