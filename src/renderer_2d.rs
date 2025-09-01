//! 2D Rendering System
//!
//! Provides basic 2D rendering capabilities for games.
//! Supports shapes, text, and frame buffer management.

use crate::font::{FontSystem, TextBitmap};
use crate::window::WindowManager;
use std::path::Path;

/// Color representation (ARGB format)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color(pub u32);

impl Color {
    pub const WHITE: Color = Color(0xFFFFFFFF);
    pub const BLACK: Color = Color(0xFF000000);
    pub const RED: Color = Color(0xFFFF0000);
    pub const GREEN: Color = Color(0xFF00FF00);
    pub const BLUE: Color = Color(0xFF0000FF);
    pub const YELLOW: Color = Color(0xFFFFFF00);
    pub const CYAN: Color = Color(0xFF00FFFF);
    pub const MAGENTA: Color = Color(0xFFFF00FF);

    /// Create a color from RGB values
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color(0xFF000000 | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32))
    }

    /// Create a color from RGBA values
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color(((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32))
    }

    /// Get red component
    pub fn r(&self) -> u8 {
        ((self.0 >> 16) & 0xFF) as u8
    }

    /// Get green component
    pub fn g(&self) -> u8 {
        ((self.0 >> 8) & 0xFF) as u8
    }

    /// Get blue component
    pub fn b(&self) -> u8 {
        (self.0 & 0xFF) as u8
    }

    /// Get alpha component
    pub fn a(&self) -> u8 {
        ((self.0 >> 24) & 0xFF) as u8
    }
}

/// 2D Renderer for basic graphics operations
pub struct Renderer2D {
    buffer: Vec<u32>,
    width: usize,
    height: usize,
    font_system: FontSystem,
}

impl Renderer2D {
    /// Create a new renderer with the given dimensions
    pub fn new(width: usize, height: usize) -> Self {
        let mut font_system = FontSystem::new();

        // Try to load a default font, fallback to built-in if available
        if let Err(_) = font_system.load_builtin_font("default") {
            // If built-in font fails, we'll use a simple fallback
        }

        Self {
            buffer: vec![0; width * height],
            width,
            height,
            font_system,
        }
    }

    /// Create a renderer that matches a window's dimensions
    pub fn from_window(window: &WindowManager) -> Self {
        let (width, height) = window.dimensions();
        Self::new(width, height)
    }

    /// Clear the buffer with a specific color
    pub fn clear(&mut self, color: Color) {
        self.buffer.fill(color.0);
    }

    /// Draw a filled rectangle
    pub fn draw_rect(&mut self, x: i32, y: i32, width: i32, height: i32, color: Color) {
        for dy in 0..height {
            for dx in 0..width {
                let px = x + dx;
                let py = y + dy;
                self.set_pixel(px, py, color);
            }
        }
    }

    /// Draw a rectangle outline
    pub fn draw_rect_outline(&mut self, x: i32, y: i32, width: i32, height: i32, color: Color) {
        // Top and bottom lines
        for dx in 0..width {
            self.set_pixel(x + dx, y, color);
            self.set_pixel(x + dx, y + height - 1, color);
        }
        // Left and right lines
        for dy in 0..height {
            self.set_pixel(x, y + dy, color);
            self.set_pixel(x + width - 1, y + dy, color);
        }
    }

    /// Draw a line between two points
    pub fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, color: Color) {
        let dx = (x2 - x1).abs();
        let dy = (y2 - y1).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = dx - dy;

        let mut x = x1;
        let mut y = y1;

        loop {
            self.set_pixel(x, y, color);

            if x == x2 && y == y2 {
                break;
            }

            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x += sx;
            }
            if e2 < dx {
                err += dx;
                y += sy;
            }
        }
    }

    /// Draw a circle
    pub fn draw_circle(&mut self, center_x: i32, center_y: i32, radius: i32, color: Color) {
        let mut x = 0;
        let mut y = radius;
        let mut d = 3 - 2 * radius;

        while y >= x {
            // Draw all 8 octants
            self.set_pixel(center_x + x, center_y + y, color);
            self.set_pixel(center_x - x, center_y + y, color);
            self.set_pixel(center_x + x, center_y - y, color);
            self.set_pixel(center_x - x, center_y - y, color);
            self.set_pixel(center_x + y, center_y + x, color);
            self.set_pixel(center_x - y, center_y + x, color);
            self.set_pixel(center_x + y, center_y - x, color);
            self.set_pixel(center_x - y, center_y - x, color);

            x += 1;
            if d > 0 {
                y -= 1;
                d += 4 * (x - y) + 10;
            } else {
                d += 4 * x + 6;
            }
        }
    }

    /// Draw filled circle
    pub fn draw_circle_filled(&mut self, center_x: i32, center_y: i32, radius: i32, color: Color) {
        for y in -radius..=radius {
            for x in -radius..=radius {
                if x * x + y * y <= radius * radius {
                    self.set_pixel(center_x + x, center_y + y, color);
                }
            }
        }
    }

    /// Draw simple text using FreeType fonts
    pub fn draw_text(&mut self, text: &str, x: usize, y: usize, color: Color, scale: usize) {
        // Better font size calculation with minimum size for readability
        let base_size = 16.0; // Minimum readable font size
        let font_size = base_size + (scale as f32 - 1.0) * 8.0; // Scale up from base

        // Try FreeType first
        if let Ok(text_bitmap) = self.font_system.render_text(text, None, font_size, color) {
            if text_bitmap.width > 0 && text_bitmap.height > 0 {
                self.draw_text_bitmap(&text_bitmap, x, y);
                return; // Success, don't use fallback
            }
        }

        // Fallback to bitmap font if FreeType fails
        self.draw_text_fallback(text, x, y, color, scale);
    }

    /// Draw text centered at a position
    pub fn draw_text_centered(
        &mut self,
        text: &str,
        center_x: usize,
        y: usize,
        color: Color,
        scale: usize,
    ) {
        // Use same font size calculation as draw_text
        let base_size = 16.0;
        let font_size = base_size + (scale as f32 - 1.0) * 8.0;

        if let Ok(metrics) = self.font_system.get_text_metrics(text, None, font_size) {
            let text_width = metrics.width as usize;
            let x = center_x.saturating_sub(text_width / 2);
            self.draw_text(text, x, y, color, scale);
        } else {
            // Fallback calculation
            let text_width = text.chars().filter(|&c| c != ' ').count() * 8 * scale;
            let x = center_x.saturating_sub(text_width / 2);
            self.draw_text(text, x, y, color, scale);
        }
    }

    /// Draw rendered text bitmap to the screen
    fn draw_text_bitmap(&mut self, bitmap: &TextBitmap, x: usize, y: usize) {
        for by in 0..bitmap.height {
            for bx in 0..bitmap.width {
                let pixel_idx = (by * bitmap.width + bx) * 4;
                let alpha = bitmap.data[pixel_idx + 3];

                if alpha > 0 {
                    let r = bitmap.data[pixel_idx];
                    let g = bitmap.data[pixel_idx + 1];
                    let b = bitmap.data[pixel_idx + 2];
                    let color = Color::rgba(r, g, b, alpha);
                    self.set_pixel((x + bx) as i32, (y + by) as i32, color);
                }
            }
        }
    }

    /// Fallback text rendering using simple bitmap font
    fn draw_text_fallback(&mut self, text: &str, x: usize, y: usize, color: Color, scale: usize) {
        let mut current_x = x;
        for ch in text.chars() {
            if ch != ' ' {
                self.draw_char_fallback(ch, current_x, y, color, scale);
            }
            current_x += 8 * scale;
        }
    }

    /// Draw a single character (fallback bitmap font)
    pub fn draw_char_fallback(&mut self, ch: char, x: usize, y: usize, color: Color, scale: usize) {
        // Improved 7x9 font for better readability
        let font_data = match ch {
            '0' => [
                [false, true, true, true, true, true, false],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [false, true, true, true, true, true, false],
            ],
            '1' => [
                [false, false, false, true, false, false, false],
                [false, false, true, true, false, false, false],
                [false, true, false, true, false, false, false],
                [false, false, false, true, false, false, false],
                [false, false, false, true, false, false, false],
                [false, false, false, true, false, false, false],
                [false, false, false, true, false, false, false],
                [false, false, false, true, false, false, false],
                [true, true, true, true, true, true, true],
            ],
            '2' => [
                [false, true, true, true, true, true, false],
                [true, false, false, false, false, false, true],
                [false, false, false, false, false, false, true],
                [false, false, false, false, false, true, false],
                [false, false, false, false, true, false, false],
                [false, false, false, true, false, false, false],
                [false, false, true, false, false, false, false],
                [false, true, false, false, false, false, false],
                [true, true, true, true, true, true, true],
            ],
            '3' => [
                [false, true, true, true, true, true, false],
                [true, false, false, false, false, false, true],
                [false, false, false, false, false, false, true],
                [false, false, false, false, true, true, false],
                [false, false, false, true, true, false, false],
                [false, false, false, false, false, false, true],
                [false, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [false, true, true, true, true, true, false],
            ],
            '4' => [
                [false, false, false, false, true, false, false],
                [false, false, false, true, true, false, false],
                [false, false, true, false, true, false, false],
                [false, true, false, false, true, false, false],
                [true, false, false, false, true, false, false],
                [true, true, true, true, true, true, true],
                [false, false, false, false, true, false, false],
                [false, false, false, false, true, false, false],
                [false, false, false, false, true, false, false],
            ],
            '5' => [
                [true, true, true, true, true, true, true],
                [true, false, false, false, false, false, false],
                [true, false, false, false, false, false, false],
                [true, true, true, true, true, true, false],
                [false, false, false, false, false, false, true],
                [false, false, false, false, false, false, true],
                [false, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [false, true, true, true, true, true, false],
            ],
            '6' => [
                [false, true, true, true, true, true, false],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, false],
                [true, false, false, false, false, false, false],
                [true, true, true, true, true, true, false],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [false, true, true, true, true, true, false],
            ],
            '7' => [
                [true, true, true, true, true, true, true],
                [false, false, false, false, false, false, true],
                [false, false, false, false, false, true, false],
                [false, false, false, false, true, false, false],
                [false, false, false, true, false, false, false],
                [false, false, true, false, false, false, false],
                [false, true, false, false, false, false, false],
                [true, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
            ],
            '8' => [
                [false, true, true, true, true, true, false],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [false, true, true, true, true, true, false],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [false, true, true, true, true, true, false],
            ],
            '9' => [
                [false, true, true, true, true, true, false],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [true, true, true, true, true, true, false],
            ],
            // Lowercase letters
            'a' => [
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
                [false, false, true, true, true, false, false],
                [true, false, false, false, false, true, false],
                [true, false, false, false, false, true, false],
                [true, false, false, false, false, true, false],
                [true, false, false, false, false, true, false],
                [false, true, true, true, true, false, false],
                [false, false, false, false, false, false, false],
            ],
            'b' => [
                [true, false, false, false, false, false, false],
                [true, false, false, false, false, false, false],
                [true, false, false, false, false, false, false],
                [true, true, true, true, true, false, false],
                [true, false, false, false, false, true, false],
                [true, false, false, false, false, true, false],
                [true, false, false, false, false, true, false],
                [true, true, true, true, true, false, false],
                [false, false, false, false, false, false, false],
            ],
            'c' => [
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
                [false, true, true, true, true, false, false],
                [true, false, false, false, false, true, false],
                [true, false, false, false, false, false, false],
                [true, false, false, false, false, false, false],
                [true, false, false, false, false, true, false],
                [false, true, true, true, true, false, false],
                [false, false, false, false, false, false, false],
            ],
            'd' => [
                [false, false, false, false, false, true, false],
                [false, false, false, false, false, true, false],
                [false, false, false, false, false, true, false],
                [false, true, true, true, true, true, false],
                [true, false, false, false, false, true, false],
                [true, false, false, false, false, true, false],
                [true, false, false, false, false, true, false],
                [false, true, true, true, true, true, false],
                [false, false, false, false, false, false, false],
            ],
            'e' => [
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
                [false, true, true, true, true, false, false],
                [true, false, false, false, false, true, false],
                [true, true, true, true, true, true, false],
                [true, false, false, false, false, false, false],
                [true, false, false, false, false, true, false],
                [false, true, true, true, true, false, false],
                [false, false, false, false, false, false, false],
            ],
            'f' => [
                [false, false, true, true, true, false, false],
                [false, true, false, false, false, true, false],
                [false, true, false, false, false, false, false],
                [true, true, true, true, true, false, false],
                [false, true, false, false, false, false, false],
                [false, true, false, false, false, false, false],
                [false, true, false, false, false, false, false],
                [false, true, false, false, false, false, false],
                [false, false, false, false, false, false, false],
            ],
            'g' => [
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
                [false, true, true, true, true, true, false],
                [true, false, false, false, false, true, false],
                [true, false, false, false, false, true, false],
                [true, false, false, false, false, true, false],
                [false, true, true, true, true, true, false],
                [false, false, false, false, false, true, false],
                [false, true, true, true, true, false, false],
            ],
            'h' => [
                [true, false, false, false, false, false, false],
                [true, false, false, false, false, false, false],
                [true, false, false, false, false, false, false],
                [true, true, true, true, true, false, false],
                [true, false, false, false, false, true, false],
                [true, false, false, false, false, true, false],
                [true, false, false, false, false, true, false],
                [true, false, false, false, false, true, false],
                [false, false, false, false, false, false, false],
            ],
            'i' => [
                [false, false, true, false, false, false, false],
                [false, false, false, false, false, false, false],
                [false, true, true, true, false, false, false],
                [false, false, true, false, false, false, false],
                [false, false, true, false, false, false, false],
                [false, false, true, false, false, false, false],
                [false, false, true, false, false, false, false],
                [false, true, true, true, false, false, false],
                [false, false, false, false, false, false, false],
            ],
            'j' => [
                [false, false, false, true, false, false, false],
                [false, false, false, false, false, false, false],
                [false, false, true, true, true, false, false],
                [false, false, false, true, false, false, false],
                [false, false, false, true, false, false, false],
                [false, false, false, true, false, false, false],
                [false, false, false, true, false, false, false],
                [true, false, false, true, false, false, false],
                [false, true, true, false, false, false, false],
            ],
            'k' => [
                [true, false, false, false, false, false, false],
                [true, false, false, false, false, false, false],
                [true, false, false, false, true, false, false],
                [true, false, false, true, false, false, false],
                [true, true, true, false, false, false, false],
                [true, false, false, true, false, false, false],
                [true, false, false, false, true, false, false],
                [true, false, false, false, false, true, false],
                [false, false, false, false, false, false, false],
            ],
            'l' => [
                [false, true, true, true, false, false, false],
                [false, false, true, false, false, false, false],
                [false, false, true, false, false, false, false],
                [false, false, true, false, false, false, false],
                [false, false, true, false, false, false, false],
                [false, false, true, false, false, false, false],
                [false, false, true, false, false, false, false],
                [false, true, true, true, false, false, false],
                [false, false, false, false, false, false, false],
            ],
            'm' => [
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
                [true, true, false, true, true, false, false],
                [true, false, true, false, true, false, false],
                [true, false, true, false, true, false, false],
                [true, false, true, false, true, false, false],
                [true, false, true, false, true, false, false],
                [true, false, true, false, true, false, false],
                [false, false, false, false, false, false, false],
            ],
            'n' => [
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
                [true, true, true, true, true, false, false],
                [true, false, false, false, true, false, false],
                [true, false, false, false, true, false, false],
                [true, false, false, false, true, false, false],
                [true, false, false, false, true, false, false],
                [true, false, false, false, true, false, false],
                [false, false, false, false, false, false, false],
            ],
            'o' => [
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
                [false, true, true, true, true, false, false],
                [true, false, false, false, false, true, false],
                [true, false, false, false, false, true, false],
                [true, false, false, false, false, true, false],
                [true, false, false, false, false, true, false],
                [false, true, true, true, true, false, false],
                [false, false, false, false, false, false, false],
            ],
            'p' => [
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
                [true, true, true, true, true, false, false],
                [true, false, false, false, false, true, false],
                [true, false, false, false, false, true, false],
                [true, false, false, false, false, true, false],
                [true, true, true, true, true, false, false],
                [true, false, false, false, false, false, false],
                [true, false, false, false, false, false, false],
            ],
            'q' => [
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
                [false, true, true, true, true, true, false],
                [true, false, false, false, false, true, false],
                [true, false, false, false, false, true, false],
                [true, false, false, false, false, true, false],
                [false, true, true, true, true, true, false],
                [false, false, false, false, false, true, false],
                [false, false, false, false, false, true, false],
            ],
            'r' => [
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
                [true, true, true, true, true, false, false],
                [true, false, false, false, false, true, false],
                [true, false, false, false, false, false, false],
                [true, false, false, false, false, false, false],
                [true, false, false, false, false, false, false],
                [true, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
            ],
            's' => [
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
                [false, true, true, true, true, false, false],
                [true, false, false, false, false, true, false],
                [true, true, true, true, false, false, false],
                [false, false, false, false, true, false, false],
                [true, false, false, false, false, true, false],
                [false, true, true, true, true, false, false],
                [false, false, false, false, false, false, false],
            ],
            't' => [
                [false, true, false, false, false, false, false],
                [false, true, false, false, false, false, false],
                [true, true, true, true, true, false, false],
                [false, true, false, false, false, false, false],
                [false, true, false, false, false, false, false],
                [false, true, false, false, false, false, false],
                [false, true, false, false, false, false, false],
                [false, false, true, true, true, false, false],
                [false, false, false, false, false, false, false],
            ],
            'u' => [
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
                [true, false, false, false, false, true, false],
                [true, false, false, false, false, true, false],
                [true, false, false, false, false, true, false],
                [true, false, false, false, false, true, false],
                [true, false, false, false, false, true, false],
                [false, true, true, true, true, false, false],
                [false, false, false, false, false, false, false],
            ],
            'v' => [
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
                [true, false, false, false, false, true, false],
                [true, false, false, false, false, true, false],
                [true, false, false, false, false, true, false],
                [true, false, false, false, false, true, false],
                [false, true, false, false, true, false, false],
                [false, false, true, true, false, false, false],
                [false, false, false, false, false, false, false],
            ],
            'w' => [
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
                [true, false, false, false, false, true, false],
                [true, false, false, false, false, true, false],
                [true, false, false, false, false, true, false],
                [true, false, true, false, true, false, false],
                [true, false, true, false, true, false, false],
                [false, true, false, false, false, true, false],
                [false, false, false, false, false, false, false],
            ],
            'x' => [
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
                [true, false, false, false, false, true, false],
                [false, true, false, false, true, false, false],
                [false, false, true, true, false, false, false],
                [false, false, true, true, false, false, false],
                [false, true, false, false, true, false, false],
                [true, false, false, false, false, true, false],
                [false, false, false, false, false, false, false],
            ],
            'y' => [
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
                [true, false, false, false, false, true, false],
                [true, false, false, false, false, true, false],
                [true, false, false, false, false, true, false],
                [false, true, false, false, true, false, false],
                [false, false, true, true, false, false, false],
                [false, false, false, false, true, false, false],
                [false, false, false, true, false, false, false],
            ],
            'z' => [
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
                [true, true, true, true, true, true, false],
                [false, false, false, false, true, false, false],
                [false, false, false, true, false, false, false],
                [false, false, true, false, false, false, false],
                [false, true, false, false, false, false, false],
                [true, true, true, true, true, true, false],
                [false, false, false, false, false, false, false],
            ],
            'A' => [
                [false, true, true, true, true, true, false],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [true, true, true, true, true, true, true],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
            ],
            'B' => [
                [true, true, true, true, true, true, false],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [true, true, true, true, true, true, false],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [true, true, true, true, true, true, false],
            ],
            'C' => [
                [false, true, true, true, true, true, false],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, false],
                [true, false, false, false, false, false, false],
                [true, false, false, false, false, false, false],
                [true, false, false, false, false, false, false],
                [true, false, false, false, false, false, false],
                [true, false, false, false, false, false, true],
                [false, true, true, true, true, true, false],
            ],
            'D' => [
                [true, true, true, true, true, true, false],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [true, true, true, true, true, true, false],
            ],
            'E' => [
                [true, true, true, true, true, true, true],
                [true, false, false, false, false, false, false],
                [true, false, false, false, false, false, false],
                [true, false, false, false, false, false, false],
                [true, true, true, true, true, true, false],
                [true, false, false, false, false, false, false],
                [true, false, false, false, false, false, false],
                [true, false, false, false, false, false, false],
                [true, true, true, true, true, true, true],
            ],
            'F' => [
                [true, true, true, true, true, true, true],
                [true, false, false, false, false, false, false],
                [true, false, false, false, false, false, false],
                [true, false, false, false, false, false, false],
                [true, true, true, true, true, true, false],
                [true, false, false, false, false, false, false],
                [true, false, false, false, false, false, false],
                [true, false, false, false, false, false, false],
                [true, false, false, false, false, false, false],
            ],
            'G' => [
                [false, true, true, true, true, true, false],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, false],
                [true, false, false, false, false, false, false],
                [true, false, false, true, true, true, true],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [false, true, true, true, true, true, false],
            ],
            'H' => [
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [true, true, true, true, true, true, true],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
            ],
            'I' => [
                [true, true, true, true, true, true, true],
                [false, false, false, true, false, false, false],
                [false, false, false, true, false, false, false],
                [false, false, false, true, false, false, false],
                [false, false, false, true, false, false, false],
                [false, false, false, true, false, false, false],
                [false, false, false, true, false, false, false],
                [false, false, false, true, false, false, false],
                [true, true, true, true, true, true, true],
            ],
            'J' => [
                [true, true, true, true, true, true, true],
                [false, false, false, false, false, true, false],
                [false, false, false, false, false, true, false],
                [false, false, false, false, false, true, false],
                [false, false, false, false, false, true, false],
                [false, false, false, false, false, true, false],
                [true, false, false, false, false, true, false],
                [true, false, false, false, false, true, false],
                [false, true, true, true, true, false, false],
            ],
            'K' => [
                [true, false, false, false, false, true, false],
                [true, false, false, false, true, false, false],
                [true, false, false, true, false, false, false],
                [true, false, true, false, false, false, false],
                [true, true, false, false, false, false, false],
                [true, false, true, false, false, false, false],
                [true, false, false, true, false, false, false],
                [true, false, false, false, true, false, false],
                [true, false, false, false, false, true, false],
            ],
            'L' => [
                [true, false, false, false, false, false, false],
                [true, false, false, false, false, false, false],
                [true, false, false, false, false, false, false],
                [true, false, false, false, false, false, false],
                [true, false, false, false, false, false, false],
                [true, false, false, false, false, false, false],
                [true, false, false, false, false, false, false],
                [true, false, false, false, false, false, false],
                [true, true, true, true, true, true, true],
            ],
            'M' => [
                [true, false, false, false, false, false, true],
                [true, true, false, false, false, true, true],
                [true, false, true, false, true, false, true],
                [true, false, true, false, true, false, true],
                [true, false, false, true, false, false, true],
                [true, false, false, true, false, false, true],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
            ],
            'N' => [
                [true, false, false, false, false, false, true],
                [true, true, false, false, false, false, true],
                [true, false, true, false, false, false, true],
                [true, false, true, false, false, false, true],
                [true, false, false, true, false, false, true],
                [true, false, false, true, false, false, true],
                [true, false, false, false, true, false, true],
                [true, false, false, false, true, false, true],
                [true, false, false, false, false, true, true],
            ],
            'O' => [
                [false, true, true, true, true, true, false],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [false, true, true, true, true, true, false],
            ],
            'P' => [
                [true, true, true, true, true, true, false],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [true, true, true, true, true, true, false],
                [true, false, false, false, false, false, false],
                [true, false, false, false, false, false, false],
                [true, false, false, false, false, false, false],
                [true, false, false, false, false, false, false],
            ],
            'Q' => [
                [false, true, true, true, true, true, false],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [true, false, false, false, true, false, true],
                [true, false, false, false, false, true, true],
                [true, false, false, false, false, false, true],
                [false, true, true, true, true, true, true],
            ],
            'R' => [
                [true, true, true, true, true, true, false],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [true, true, true, true, true, true, false],
                [true, false, false, false, true, false, false],
                [true, false, false, false, false, true, false],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
            ],
            'S' => [
                [false, true, true, true, true, true, false],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, false],
                [true, false, false, false, false, false, false],
                [false, true, true, true, true, true, false],
                [false, false, false, false, false, false, true],
                [false, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [false, true, true, true, true, true, false],
            ],
            'T' => [
                [true, true, true, true, true, true, true],
                [false, false, false, true, false, false, false],
                [false, false, false, true, false, false, false],
                [false, false, false, true, false, false, false],
                [false, false, false, true, false, false, false],
                [false, false, false, true, false, false, false],
                [false, false, false, true, false, false, false],
                [false, false, false, true, false, false, false],
                [false, false, false, true, false, false, false],
            ],
            'U' => [
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [false, true, true, true, true, true, false],
            ],
            'V' => [
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [false, true, false, false, false, true, false],
                [false, true, false, false, false, true, false],
                [false, false, true, true, true, false, false],
            ],
            'W' => [
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [true, false, true, false, true, false, true],
                [true, false, true, false, true, false, true],
                [true, true, false, false, false, true, true],
                [true, false, false, false, false, false, true],
            ],
            'X' => [
                [true, false, false, false, false, false, true],
                [false, true, false, false, false, true, false],
                [false, true, false, false, false, true, false],
                [false, false, true, false, true, false, false],
                [false, false, true, false, true, false, false],
                [false, false, false, true, false, false, false],
                [false, false, true, false, true, false, false],
                [false, true, false, false, false, true, false],
                [true, false, false, false, false, false, true],
            ],
            'Y' => [
                [true, false, false, false, false, false, true],
                [false, true, false, false, false, true, false],
                [false, true, false, false, false, true, false],
                [false, false, true, false, true, false, false],
                [false, false, true, false, true, false, false],
                [false, false, false, true, false, false, false],
                [false, false, false, true, false, false, false],
                [false, false, false, true, false, false, false],
                [false, false, false, true, false, false, false],
            ],
            'Z' => [
                [true, true, true, true, true, true, true],
                [false, false, false, false, false, true, false],
                [false, false, false, false, true, false, false],
                [false, false, false, true, false, false, false],
                [false, false, true, false, false, false, false],
                [false, true, false, false, false, false, false],
                [true, false, false, false, false, false, false],
                [true, false, false, false, false, false, false],
                [true, true, true, true, true, true, true],
            ],
            // Lowercase letters
            'a' => [
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
                [false, false, true, true, true, false, false],
                [true, false, false, false, false, true, false],
                [true, false, false, false, false, true, false],
                [true, false, false, false, false, true, false],
                [true, false, false, false, false, true, false],
                [false, true, true, true, true, false, false],
                [false, false, false, false, false, false, false],
            ],
            'b' => [
                [true, false, false, false, false, false, false],
                [true, false, false, false, false, false, false],
                [true, false, false, false, false, false, false],
                [true, true, true, true, true, false, false],
                [true, false, false, false, false, true, false],
                [true, false, false, false, false, true, false],
                [true, false, false, false, false, true, false],
                [true, true, true, true, true, false, false],
                [false, false, false, false, false, false, false],
            ],
            'c' => [
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
                [false, true, true, true, true, false, false],
                [true, false, false, false, false, true, false],
                [true, false, false, false, false, false, false],
                [true, false, false, false, false, false, false],
                [true, false, false, false, false, true, false],
                [false, true, true, true, true, false, false],
                [false, false, false, false, false, false, false],
            ],
            'd' => [
                [false, false, false, false, false, true, false],
                [false, false, false, false, false, true, false],
                [false, false, false, false, false, true, false],
                [false, true, true, true, true, true, false],
                [true, false, false, false, false, true, false],
                [true, false, false, false, false, true, false],
                [true, false, false, false, false, true, false],
                [false, true, true, true, true, true, false],
                [false, false, false, false, false, false, false],
            ],
            'e' => [
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
                [false, true, true, true, true, false, false],
                [true, false, false, false, false, true, false],
                [true, true, true, true, true, true, false],
                [true, false, false, false, false, false, false],
                [true, false, false, false, false, true, false],
                [false, true, true, true, true, false, false],
                [false, false, false, false, false, false, false],
            ],
            'f' => [
                [false, false, true, true, true, false, false],
                [false, true, false, false, false, true, false],
                [false, true, false, false, false, false, false],
                [true, true, true, true, true, false, false],
                [false, true, false, false, false, false, false],
                [false, true, false, false, false, false, false],
                [false, true, false, false, false, false, false],
                [false, true, false, false, false, false, false],
                [false, false, false, false, false, false, false],
            ],
            'g' => [
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
                [false, true, true, true, true, true, false],
                [true, false, false, false, false, true, false],
                [true, false, false, false, false, true, false],
                [true, false, false, false, false, true, false],
                [false, true, true, true, true, true, false],
                [false, false, false, false, false, true, false],
                [false, true, true, true, true, false, false],
            ],
            'h' => [
                [true, false, false, false, false, false, false],
                [true, false, false, false, false, false, false],
                [true, false, false, false, false, false, false],
                [true, true, true, true, true, false, false],
                [true, false, false, false, false, true, false],
                [true, false, false, false, false, true, false],
                [true, false, false, false, false, true, false],
                [true, false, false, false, false, true, false],
                [false, false, false, false, false, false, false],
            ],
            'i' => [
                [false, false, true, false, false, false, false],
                [false, false, false, false, false, false, false],
                [false, true, true, true, false, false, false],
                [false, false, true, false, false, false, false],
                [false, false, true, false, false, false, false],
                [false, false, true, false, false, false, false],
                [false, false, true, false, false, false, false],
                [false, true, true, true, false, false, false],
                [false, false, false, false, false, false, false],
            ],
            'j' => [
                [false, false, false, true, false, false, false],
                [false, false, false, false, false, false, false],
                [false, false, true, true, true, false, false],
                [false, false, false, true, false, false, false],
                [false, false, false, true, false, false, false],
                [false, false, false, true, false, false, false],
                [false, false, false, true, false, false, false],
                [true, false, false, true, false, false, false],
                [false, true, true, false, false, false, false],
            ],
            'k' => [
                [true, false, false, false, false, false, false],
                [true, false, false, false, false, false, false],
                [true, false, false, false, true, false, false],
                [true, false, false, true, false, false, false],
                [true, true, true, false, false, false, false],
                [true, false, false, true, false, false, false],
                [true, false, false, false, true, false, false],
                [true, false, false, false, false, true, false],
                [false, false, false, false, false, false, false],
            ],
            'l' => [
                [false, true, true, true, false, false, false],
                [false, false, true, false, false, false, false],
                [false, false, true, false, false, false, false],
                [false, false, true, false, false, false, false],
                [false, false, true, false, false, false, false],
                [false, false, true, false, false, false, false],
                [false, false, true, false, false, false, false],
                [false, true, true, true, false, false, false],
                [false, false, false, false, false, false, false],
            ],
            'm' => [
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
                [true, true, false, true, true, false, false],
                [true, false, true, false, true, false, false],
                [true, false, true, false, true, false, false],
                [true, false, true, false, true, false, false],
                [true, false, true, false, true, false, false],
                [true, false, true, false, true, false, false],
                [false, false, false, false, false, false, false],
            ],
            'n' => [
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
                [true, true, true, true, true, false, false],
                [true, false, false, false, true, false, false],
                [true, false, false, false, true, false, false],
                [true, false, false, false, true, false, false],
                [true, false, false, false, true, false, false],
                [true, false, false, false, true, false, false],
                [false, false, false, false, false, false, false],
            ],
            'o' => [
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
                [false, true, true, true, true, false, false],
                [true, false, false, false, false, true, false],
                [true, false, false, false, false, true, false],
                [true, false, false, false, false, true, false],
                [true, false, false, false, false, true, false],
                [false, true, true, true, true, false, false],
                [false, false, false, false, false, false, false],
            ],
            'p' => [
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
                [true, true, true, true, true, false, false],
                [true, false, false, false, false, true, false],
                [true, false, false, false, false, true, false],
                [true, false, false, false, false, true, false],
                [true, true, true, true, true, false, false],
                [true, false, false, false, false, false, false],
                [true, false, false, false, false, false, false],
            ],
            'q' => [
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
                [false, true, true, true, true, true, false],
                [true, false, false, false, false, true, false],
                [true, false, false, false, false, true, false],
                [true, false, false, false, false, true, false],
                [false, true, true, true, true, true, false],
                [false, false, false, false, false, true, false],
                [false, false, false, false, false, true, false],
            ],
            'r' => [
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
                [true, true, true, true, true, false, false],
                [true, false, false, false, false, true, false],
                [true, false, false, false, false, false, false],
                [true, false, false, false, false, false, false],
                [true, false, false, false, false, false, false],
                [true, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
            ],
            's' => [
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
                [false, true, true, true, true, false, false],
                [true, false, false, false, false, true, false],
                [true, true, true, true, false, false, false],
                [false, false, false, false, true, false, false],
                [true, false, false, false, false, true, false],
                [false, true, true, true, true, false, false],
                [false, false, false, false, false, false, false],
            ],
            't' => [
                [false, true, false, false, false, false, false],
                [false, true, false, false, false, false, false],
                [true, true, true, true, true, false, false],
                [false, true, false, false, false, false, false],
                [false, true, false, false, false, false, false],
                [false, true, false, false, false, false, false],
                [false, true, false, false, false, false, false],
                [false, false, true, true, true, false, false],
                [false, false, false, false, false, false, false],
            ],
            'u' => [
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
                [true, false, false, false, false, true, false],
                [true, false, false, false, false, true, false],
                [true, false, false, false, false, true, false],
                [true, false, false, false, false, true, false],
                [true, false, false, false, false, true, false],
                [false, true, true, true, true, false, false],
                [false, false, false, false, false, false, false],
            ],
            'v' => [
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
                [true, false, false, false, false, true, false],
                [true, false, false, false, false, true, false],
                [true, false, false, false, false, true, false],
                [true, false, false, false, false, true, false],
                [false, true, false, false, true, false, false],
                [false, false, true, true, false, false, false],
                [false, false, false, false, false, false, false],
            ],
            'w' => [
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
                [true, false, false, false, false, true, false],
                [true, false, false, false, false, true, false],
                [true, false, false, false, false, true, false],
                [true, false, true, false, true, false, false],
                [true, false, true, false, true, false, false],
                [false, true, false, false, false, true, false],
                [false, false, false, false, false, false, false],
            ],
            'x' => [
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
                [true, false, false, false, false, true, false],
                [false, true, false, false, true, false, false],
                [false, false, true, true, false, false, false],
                [false, false, true, true, false, false, false],
                [false, true, false, false, true, false, false],
                [true, false, false, false, false, true, false],
                [false, false, false, false, false, false, false],
            ],
            'y' => [
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
                [true, false, false, false, false, true, false],
                [true, false, false, false, false, true, false],
                [true, false, false, false, false, true, false],
                [false, true, false, false, true, false, false],
                [false, false, true, true, false, false, false],
                [false, false, false, false, true, false, false],
                [false, false, false, true, false, false, false],
            ],
            'z' => [
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
                [true, true, true, true, true, true, false],
                [false, false, false, false, true, false, false],
                [false, false, false, true, false, false, false],
                [false, false, true, false, false, false, false],
                [false, true, false, false, false, false, false],
                [true, true, true, true, true, true, false],
                [false, false, false, false, false, false, false],
            ],
            ' ' => [
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
            ],

            '!' => [
                [false, false, true, false, false, false, false],
                [false, false, true, false, false, false, false],
                [false, false, true, false, false, false, false],
                [false, false, true, false, false, false, false],
                [false, false, true, false, false, false, false],
                [false, false, true, false, false, false, false],
                [false, false, false, false, false, false, false],
                [false, false, true, false, false, false, false],
                [false, false, false, false, false, false, false],
            ],
            '?' => [
                [false, true, true, true, true, false, false],
                [true, false, false, false, false, true, false],
                [false, false, false, false, true, false, false],
                [false, false, false, true, false, false, false],
                [false, false, true, false, false, false, false],
                [false, false, true, false, false, false, false],
                [false, false, false, false, false, false, false],
                [false, false, true, false, false, false, false],
                [false, false, false, false, false, false, false],
            ],
            ':' => [
                [false, false, false, false, false, false, false],
                [false, false, true, false, false, false, false],
                [false, false, true, false, false, false, false],
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
                [false, false, true, false, false, false, false],
                [false, false, true, false, false, false, false],
                [false, false, false, false, false, false, false],
            ],
            '-' => [
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
                [true, true, true, true, true, true, true],
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
            ],
            '_' => [
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
                [true, true, true, true, true, true, true],
            ],
            '.' => [
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
                [false, false, true, false, false, false, false],
                [false, false, true, false, false, false, false],
            ],
            ',' => [
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
                [false, false, true, false, false, false, false],
                [false, true, false, false, false, false, false],
            ],
            ':' => [
                [false, false, false, false, false, false, false],
                [false, false, true, false, false, false, false],
                [false, false, true, false, false, false, false],
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
                [false, false, true, false, false, false, false],
                [false, false, true, false, false, false, false],
                [false, false, false, false, false, false, false],
            ],
            '/' => [
                [false, false, false, false, false, true, false],
                [false, false, false, false, true, false, false],
                [false, false, false, true, false, false, false],
                [false, false, true, false, false, false, false],
                [false, true, false, false, false, false, false],
                [true, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
                [false, false, false, false, false, false, false],
            ],
            _ => [
                [true, true, true, true, true, true, true],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [true, false, false, false, false, false, true],
                [true, true, true, true, true, true, true],
            ], // Default box for unknown characters
        };

        for (row, pixels) in font_data.iter().enumerate() {
            for (col, pixel) in pixels.iter().enumerate() {
                if *pixel {
                    let px = x + col * scale;
                    let py = y + row * scale;
                    self.draw_rect(px as i32, py as i32, scale as i32, scale as i32, color);
                }
            }
        }
    }

    /// Set a single pixel
    pub fn set_pixel(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            let index = (y as usize) * self.width + (x as usize);
            if index < self.buffer.len() {
                self.buffer[index] = color.0;
            }
        }
    }

    /// Get the buffer for rendering
    pub fn buffer(&self) -> &[u32] {
        &self.buffer
    }

    /// Get mutable buffer access
    pub fn buffer_mut(&mut self) -> &mut [u32] {
        &mut self.buffer
    }

    /// Get buffer dimensions
    pub fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    /// Load a TTF font from file
    pub fn load_font<P: AsRef<Path>>(
        &mut self,
        name: &str,
        path: P,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.font_system.load_font_from_file(name, path)
    }

    /// Set the default font for text rendering
    pub fn set_default_font(&mut self, name: &str) {
        self.font_system.set_default_font(name);
    }
}

/// Rendering context that combines window and renderer
pub struct RenderContext {
    pub window: WindowManager,
    pub renderer: Renderer2D,
}

impl RenderContext {
    /// Create a new rendering context
    pub fn new(config: crate::window::WindowConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let window = WindowManager::new(config)?;
        let renderer = Renderer2D::from_window(&window);

        Ok(Self { window, renderer })
    }

    /// Update the rendering context
    pub fn update(&mut self) {
        self.window.update();
    }

    /// Present the current frame
    pub fn present(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.window.window().update_with_buffer(
            self.renderer.buffer(),
            self.renderer.dimensions().0,
            self.renderer.dimensions().1,
        )?;
        Ok(())
    }

    /// Check if the context should close
    pub fn should_close(&self) -> bool {
        self.window.should_close()
    }
}
