//! Font system using FreeType for high-quality text rendering
//!
//! This module provides TTF font loading, glyph caching, and rendering capabilities
//! for improved text quality in the game engine.

use crate::renderer_2d::Color;
use rusttype::{point, Font, PositionedGlyph, Scale};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Font system for loading and rendering TTF fonts
pub struct FontSystem {
    fonts: HashMap<String, Font<'static>>,
    glyph_cache: HashMap<(String, char, u32), Vec<u8>>,
    default_font: Option<String>,
}

impl FontSystem {
    /// Create a new font system
    pub fn new() -> Self {
        Self {
            fonts: HashMap::new(),
            glyph_cache: HashMap::new(),
            default_font: None,
        }
    }

    /// Load a TTF font from file
    pub fn load_font<P: AsRef<Path>>(
        &mut self,
        name: &str,
        path: P,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let font_data = fs::read(path)?;
        let font = Font::try_from_vec(font_data).ok_or("Failed to parse font data")?;

        self.fonts.insert(name.to_string(), font);

        // Set as default if this is the first font loaded
        if self.default_font.is_none() {
            self.default_font = Some(name.to_string());
        }

        Ok(())
    }

    /// Load a TTF font from file
    pub fn load_font_from_file<P: AsRef<Path>>(
        &mut self,
        name: &str,
        path: P,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.load_font(name, path)
    }

    /// Load a built-in font (fallback)
    pub fn load_builtin_font(&mut self, _name: &str) -> Result<(), Box<dyn std::error::Error>> {
        // For now, we'll skip built-in font loading since we don't have embedded fonts
        // This will cause the system to fall back to bitmap rendering
        Ok(())
    }

    /// Get a font by name, or the default font
    pub fn get_font(&self, name: Option<&str>) -> Option<&Font<'static>> {
        let font_name = name.unwrap_or(self.default_font.as_ref()?);
        self.fonts.get(font_name)
    }

    /// Render text to a pixel buffer
    pub fn render_text(
        &mut self,
        text: &str,
        font_name: Option<&str>,
        font_size: f32,
        color: Color,
    ) -> Result<TextBitmap, Box<dyn std::error::Error>> {
        let font = self.get_font(font_name).ok_or("Font not found")?;

        let scale = Scale::uniform(font_size);
        let v_metrics = font.v_metrics(scale);

        // Layout glyphs
        let glyphs: Vec<PositionedGlyph> = font
            .layout(text, scale, point(0.0, v_metrics.ascent))
            .collect();

        // Calculate text bounds
        let width = glyphs
            .iter()
            .rev()
            .map(|g| g.position().x as f32 + g.unpositioned().h_metrics().advance_width)
            .next()
            .unwrap_or(0.0) as usize;

        let height = (v_metrics.ascent - v_metrics.descent).ceil() as usize;

        if width == 0 || height == 0 {
            return Ok(TextBitmap {
                width: 0,
                height: 0,
                data: Vec::new(),
            });
        }

        // Create pixel buffer
        let mut pixel_data = vec![0u8; width * height * 4]; // RGBA

        // Render each glyph
        for glyph in glyphs {
            if let Some(bb) = glyph.pixel_bounding_box() {
                glyph.draw(|x, y, v| {
                    let x = x as i32 + bb.min.x;
                    let y = y as i32 + bb.min.y;

                    if x >= 0 && x < width as i32 && y >= 0 && y < height as i32 {
                        let idx = ((y as usize * width) + x as usize) * 4;
                        let alpha = (v * 255.0) as u8;

                        pixel_data[idx] = color.r(); // R
                        pixel_data[idx + 1] = color.g(); // G
                        pixel_data[idx + 2] = color.b(); // B
                        pixel_data[idx + 3] = alpha; // A
                    }
                });
            }
        }

        Ok(TextBitmap {
            width,
            height,
            data: pixel_data,
        })
    }

    /// Get text metrics without rendering
    pub fn get_text_metrics(
        &self,
        text: &str,
        font_name: Option<&str>,
        font_size: f32,
    ) -> Result<TextMetrics, Box<dyn std::error::Error>> {
        let font = self.get_font(font_name).ok_or("Font not found")?;

        let scale = Scale::uniform(font_size);
        let v_metrics = font.v_metrics(scale);

        let glyphs: Vec<PositionedGlyph> = font
            .layout(text, scale, point(0.0, v_metrics.ascent))
            .collect();

        let width = glyphs
            .iter()
            .rev()
            .map(|g| g.position().x as f32 + g.unpositioned().h_metrics().advance_width)
            .next()
            .unwrap_or(0.0);

        let height = v_metrics.ascent - v_metrics.descent;

        Ok(TextMetrics {
            width,
            height,
            ascent: v_metrics.ascent,
            descent: v_metrics.descent,
        })
    }

    /// Set the default font
    pub fn set_default_font(&mut self, name: &str) {
        self.default_font = Some(name.to_string());
    }

    /// Get the default font name
    pub fn get_default_font(&self) -> Option<&str> {
        self.default_font.as_deref()
    }
}

/// Rendered text bitmap data
pub struct TextBitmap {
    pub width: usize,
    pub height: usize,
    pub data: Vec<u8>, // RGBA pixel data
}

/// Text layout metrics
pub struct TextMetrics {
    pub width: f32,
    pub height: f32,
    pub ascent: f32,
    pub descent: f32,
}

impl Default for FontSystem {
    fn default() -> Self {
        Self::new()
    }
}
