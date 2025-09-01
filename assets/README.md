# Assets Directory

This directory contains assets for the modular game engine.

## Fonts

Place TTF font files in the `fonts/` subdirectory. The engine will automatically load and use TTF fonts for improved text rendering quality.

### How to Add Fonts

1. Copy TTF font files to `assets/fonts/`
2. Common system font locations on Linux:
   - `/usr/share/fonts/truetype/` (Ubuntu/Debian)
   - `/usr/share/fonts/` (general)
3. Popular fonts to try:
   - `DejaVuSans.ttf` - Clean, readable sans-serif
   - `LiberationSans-Regular.ttf` - Professional appearance
   - `FiraCode-Regular.ttf` - Monospace with good readability

### Usage in Code

To load a font in your game:

```rust
// Load a font (call this during initialization)
if let Ok(()) = renderer.load_font("game_font", "assets/fonts/DejaVuSans.ttf") {
    // Set it as default for all text rendering
    renderer.set_default_font("game_font");
    println!("Custom font loaded successfully!");
} else {
    println!("Using fallback bitmap font");
}

// Now all text rendering will use the loaded font
renderer.draw_text("Hello World!", 100, 100, Color::WHITE, 2);
renderer.draw_text_centered("Game Title", 400, 200, Color::GREEN, 3);
```

### Font Loading in Improved Pong

The improved pong demo is ready to use custom fonts. Simply add a TTF file to `assets/fonts/` and the game will automatically use it for all menu text, scores, and UI elements.

### Fallback System

If no fonts are loaded or font loading fails, the system automatically falls back to the built-in bitmap font for compatibility.