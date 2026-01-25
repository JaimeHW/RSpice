//! Text Rendering System
//!
//! Commercial-grade GPU text rendering for schematic labels using a glyph atlas.
//! Follows professional EDA patterns for efficient text display at any zoom level.
//!
//! # Architecture
//!
//! Uses a monospace glyph atlas approach:
//! - Pre-rasterized ASCII glyphs in a texture atlas
//! - Each character is a textured quad instance
//! - LOD support: hide text below minimum readable size
//! - Batch rendering: all text in single draw call

use std::collections::HashMap;

use super::vertex::Vertex;

// =============================================================================
// Constants
// =============================================================================

/// Number of ASCII characters in the atlas (printable range)
pub const GLYPH_COUNT: usize = 95; // ASCII 32-126

/// Glyph width in atlas texture pixels
pub const GLYPH_WIDTH: u32 = 8;

/// Glyph height in atlas texture pixels  
pub const GLYPH_HEIGHT: u32 = 16;

/// Atlas texture width (12 glyphs per row)
pub const ATLAS_WIDTH: u32 = 128;

/// Atlas texture height
pub const ATLAS_HEIGHT: u32 = 128;

/// Minimum screen-space glyph height for LOD culling
pub const MIN_VISIBLE_HEIGHT: f32 = 4.0;

// =============================================================================
// Glyph Info
// =============================================================================

/// Information about a single glyph in the atlas
#[derive(Debug, Clone, Copy, Default)]
pub struct GlyphInfo {
    /// UV coordinates in atlas (top-left corner)
    pub u: f32,
    pub v: f32,
    /// UV size
    pub u_size: f32,
    pub v_size: f32,
    /// Advance width (for positioning next glyph)
    pub advance: f32,
}

/// Glyph atlas containing UV mapping for all characters
#[derive(Debug, Clone)]
pub struct GlyphAtlas {
    /// Mapping from ASCII code to glyph info
    glyphs: HashMap<char, GlyphInfo>,
    /// Atlas texture dimensions
    pub width: u32,
    pub height: u32,
}

impl Default for GlyphAtlas {
    fn default() -> Self {
        Self::new()
    }
}

impl GlyphAtlas {
    /// Create a new glyph atlas with standard ASCII layout
    pub fn new() -> Self {
        let mut glyphs = HashMap::new();
        let glyphs_per_row = ATLAS_WIDTH / GLYPH_WIDTH;
        let u_size = GLYPH_WIDTH as f32 / ATLAS_WIDTH as f32;
        let v_size = GLYPH_HEIGHT as f32 / ATLAS_HEIGHT as f32;

        for i in 0..GLYPH_COUNT {
            let ascii = (i + 32) as u8 as char;
            let col = i as u32 % glyphs_per_row;
            let row = i as u32 / glyphs_per_row;

            let u = col as f32 * u_size;
            let v = row as f32 * v_size;

            glyphs.insert(ascii, GlyphInfo {
                u,
                v,
                u_size,
                v_size,
                advance: GLYPH_WIDTH as f32,
            });
        }

        Self {
            glyphs,
            width: ATLAS_WIDTH,
            height: ATLAS_HEIGHT,
        }
    }

    /// Get glyph info for a character
    pub fn get(&self, c: char) -> Option<&GlyphInfo> {
        self.glyphs.get(&c)
    }

    /// Get glyph info or space fallback
    pub fn get_or_default(&self, c: char) -> GlyphInfo {
        self.glyphs.get(&c).copied().unwrap_or_else(|| {
            self.glyphs.get(&' ').copied().unwrap_or_default()
        })
    }

    /// Check if character is in atlas
    pub fn contains(&self, c: char) -> bool {
        self.glyphs.contains_key(&c)
    }

    /// Get number of glyphs in atlas
    pub fn glyph_count(&self) -> usize {
        self.glyphs.len()
    }
}

// =============================================================================
// Text Instance
// =============================================================================

/// A text instance for GPU rendering
///
/// Size: 64 bytes (16-byte aligned for GPU buffers)
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TextInstance {
    /// World position (x, y) - 8 bytes
    pub position: [f32; 2],
    /// Scale factor - 4 bytes
    pub scale: f32,
    /// Padding - 4 bytes  
    _padding1: f32,
    /// Glyph UV in atlas (u, v, u_size, v_size) - 16 bytes
    pub uv: [f32; 4],
    /// Text color RGBA - 16 bytes
    pub color: [f32; 4],
    /// Reserved for future use - 16 bytes
    _reserved: [f32; 4],
}

impl TextInstance {
    /// Create a new text instance
    pub fn new(x: f32, y: f32, glyph: &GlyphInfo, color: [f32; 4], scale: f32) -> Self {
        Self {
            position: [x, y],
            scale,
            _padding1: 0.0,
            uv: [glyph.u, glyph.v, glyph.u_size, glyph.v_size],
            color,
            _reserved: [0.0; 4],
        }
    }
}

// =============================================================================
// Text Layout
// =============================================================================

/// Text alignment options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextAlign {
    #[default]
    Left,
    Center,
    Right,
}

/// Layout configuration for text
#[derive(Debug, Clone, Copy)]
pub struct TextLayout {
    pub x: f32,
    pub y: f32,
    pub scale: f32,
    pub color: [f32; 4],
    pub align: TextAlign,
}

impl Default for TextLayout {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            scale: 1.0,
            color: [1.0, 1.0, 1.0, 1.0],
            align: TextAlign::Left,
        }
    }
}

/// Generate text instances for a string
pub fn layout_text(text: &str, layout: &TextLayout, atlas: &GlyphAtlas) -> Vec<TextInstance> {
    if text.is_empty() {
        return Vec::new();
    }

    let mut instances = Vec::with_capacity(text.len());
    let char_width = GLYPH_WIDTH as f32 * layout.scale;

    // Calculate starting X based on alignment
    let text_width = text.len() as f32 * char_width;
    let start_x = match layout.align {
        TextAlign::Left => layout.x,
        TextAlign::Center => layout.x - text_width / 2.0,
        TextAlign::Right => layout.x - text_width,
    };

    let mut x = start_x;
    for c in text.chars() {
        let glyph = atlas.get_or_default(c);
        instances.push(TextInstance::new(x, layout.y, &glyph, layout.color, layout.scale));
        x += glyph.advance * layout.scale;
    }

    instances
}

/// Calculate text width in world units
pub fn text_width(text: &str, scale: f32) -> f32 {
    text.len() as f32 * GLYPH_WIDTH as f32 * scale
}

/// Check if text should be visible at given zoom level
pub fn is_text_visible(scale: f32, zoom: f32) -> bool {
    let screen_height = GLYPH_HEIGHT as f32 * scale * zoom;
    screen_height >= MIN_VISIBLE_HEIGHT
}

// =============================================================================
// Label Data
// =============================================================================

/// A schematic label for rendering
#[derive(Debug, Clone)]
pub struct LabelData {
    /// Label text
    pub text: String,
    /// World position
    pub x: f32,
    pub y: f32,
    /// Scale factor
    pub scale: f32,
    /// Text color
    pub color: [f32; 4],
    /// Alignment
    pub align: TextAlign,
}

impl LabelData {
    /// Create a new label
    pub fn new(text: impl Into<String>, x: f32, y: f32) -> Self {
        Self {
            text: text.into(),
            x,
            y,
            scale: 0.1, // Default scale for schematic text
            color: [1.0, 1.0, 1.0, 1.0],
            align: TextAlign::Left,
        }
    }

    /// Set label color
    pub fn with_color(mut self, color: [f32; 4]) -> Self {
        self.color = color;
        self
    }

    /// Set text alignment
    pub fn with_align(mut self, align: TextAlign) -> Self {
        self.align = align;
        self
    }

    /// Set scale
    pub fn with_scale(mut self, scale: f32) -> Self {
        self.scale = scale;
        self
    }

    /// Generate text instances for this label
    pub fn to_instances(&self, atlas: &GlyphAtlas) -> Vec<TextInstance> {
        let layout = TextLayout {
            x: self.x,
            y: self.y,
            scale: self.scale,
            color: self.color,
            align: self.align,
        };
        layout_text(&self.text, &layout, atlas)
    }
}

// =============================================================================
// Glyph Bitmap Generation (for texture atlas creation)
// =============================================================================

/// Generate a simple bitmap font atlas (8x16 monospace)
/// Returns raw pixel data (grayscale, 1 byte per pixel)
pub fn generate_bitmap_atlas() -> Vec<u8> {
    let mut data = vec![0u8; (ATLAS_WIDTH * ATLAS_HEIGHT) as usize];

    // Simple 5x7 bitmap font patterns for ASCII 32-126
    let font_data = get_font_data();

    for (i, pattern) in font_data.iter().enumerate() {
        if i >= GLYPH_COUNT {
            break;
        }

        let glyphs_per_row = ATLAS_WIDTH / GLYPH_WIDTH;
        let col = (i as u32 % glyphs_per_row) * GLYPH_WIDTH;
        let row = (i as u32 / glyphs_per_row) * GLYPH_HEIGHT;

        // Render 5x7 pattern centered in 8x16 cell
        let offset_x = 1;
        let offset_y = 4;

        for (py, &line) in pattern.iter().enumerate() {
            for px in 0..5 {
                if (line >> (4 - px)) & 1 == 1 {
                    let x = col + offset_x + px;
                    let y = row + offset_y + py as u32;
                    if x < ATLAS_WIDTH && y < ATLAS_HEIGHT {
                        data[(y * ATLAS_WIDTH + x) as usize] = 255;
                    }
                }
            }
        }
    }

    data
}

/// Get 5x7 bitmap font patterns for ASCII 32-126
fn get_font_data() -> Vec<[u8; 7]> {
    vec![
        // Space (32)
        [0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000],
        // ! (33)
        [0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00000, 0b00100],
        // " (34)
        [0b01010, 0b01010, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000],
        // # (35)
        [0b01010, 0b11111, 0b01010, 0b01010, 0b11111, 0b01010, 0b00000],
        // $ (36)
        [0b00100, 0b01111, 0b10100, 0b01110, 0b00101, 0b11110, 0b00100],
        // % (37)
        [0b11001, 0b11010, 0b00100, 0b01000, 0b01011, 0b10011, 0b00000],
        // & (38)
        [0b01100, 0b10010, 0b01100, 0b10101, 0b10010, 0b01101, 0b00000],
        // ' (39)
        [0b00100, 0b00100, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000],
        // ( (40)
        [0b00010, 0b00100, 0b01000, 0b01000, 0b01000, 0b00100, 0b00010],
        // ) (41)
        [0b01000, 0b00100, 0b00010, 0b00010, 0b00010, 0b00100, 0b01000],
        // * (42)
        [0b00000, 0b00100, 0b10101, 0b01110, 0b10101, 0b00100, 0b00000],
        // + (43)
        [0b00000, 0b00100, 0b00100, 0b11111, 0b00100, 0b00100, 0b00000],
        // , (44)
        [0b00000, 0b00000, 0b00000, 0b00000, 0b00100, 0b00100, 0b01000],
        // - (45)
        [0b00000, 0b00000, 0b00000, 0b11111, 0b00000, 0b00000, 0b00000],
        // . (46)
        [0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00100, 0b00000],
        // / (47)
        [0b00001, 0b00010, 0b00100, 0b01000, 0b10000, 0b00000, 0b00000],
        // 0-9 (48-57)
        [0b01110, 0b10001, 0b10011, 0b10101, 0b11001, 0b10001, 0b01110], // 0
        [0b00100, 0b01100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110], // 1
        [0b01110, 0b10001, 0b00001, 0b00110, 0b01000, 0b10000, 0b11111], // 2
        [0b01110, 0b10001, 0b00001, 0b00110, 0b00001, 0b10001, 0b01110], // 3
        [0b00010, 0b00110, 0b01010, 0b10010, 0b11111, 0b00010, 0b00010], // 4
        [0b11111, 0b10000, 0b11110, 0b00001, 0b00001, 0b10001, 0b01110], // 5
        [0b00110, 0b01000, 0b10000, 0b11110, 0b10001, 0b10001, 0b01110], // 6
        [0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b01000, 0b01000], // 7
        [0b01110, 0b10001, 0b10001, 0b01110, 0b10001, 0b10001, 0b01110], // 8
        [0b01110, 0b10001, 0b10001, 0b01111, 0b00001, 0b00010, 0b01100], // 9
        // : (58)
        [0b00000, 0b00100, 0b00000, 0b00000, 0b00100, 0b00000, 0b00000],
        // ; (59)
        [0b00000, 0b00100, 0b00000, 0b00000, 0b00100, 0b00100, 0b01000],
        // < (60)
        [0b00010, 0b00100, 0b01000, 0b10000, 0b01000, 0b00100, 0b00010],
        // = (61)
        [0b00000, 0b00000, 0b11111, 0b00000, 0b11111, 0b00000, 0b00000],
        // > (62)
        [0b01000, 0b00100, 0b00010, 0b00001, 0b00010, 0b00100, 0b01000],
        // ? (63)
        [0b01110, 0b10001, 0b00010, 0b00100, 0b00100, 0b00000, 0b00100],
        // @ (64)
        [0b01110, 0b10001, 0b10111, 0b10101, 0b10111, 0b10000, 0b01110],
        // A-Z (65-90)
        [0b01110, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001], // A
        [0b11110, 0b10001, 0b10001, 0b11110, 0b10001, 0b10001, 0b11110], // B
        [0b01110, 0b10001, 0b10000, 0b10000, 0b10000, 0b10001, 0b01110], // C
        [0b11110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b11110], // D
        [0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b11111], // E
        [0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b10000], // F
        [0b01110, 0b10001, 0b10000, 0b10111, 0b10001, 0b10001, 0b01111], // G
        [0b10001, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001], // H
        [0b01110, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110], // I
        [0b00111, 0b00010, 0b00010, 0b00010, 0b00010, 0b10010, 0b01100], // J
        [0b10001, 0b10010, 0b10100, 0b11000, 0b10100, 0b10010, 0b10001], // K
        [0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b11111], // L
        [0b10001, 0b11011, 0b10101, 0b10101, 0b10001, 0b10001, 0b10001], // M
        [0b10001, 0b11001, 0b10101, 0b10011, 0b10001, 0b10001, 0b10001], // N
        [0b01110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110], // O
        [0b11110, 0b10001, 0b10001, 0b11110, 0b10000, 0b10000, 0b10000], // P
        [0b01110, 0b10001, 0b10001, 0b10001, 0b10101, 0b10010, 0b01101], // Q
        [0b11110, 0b10001, 0b10001, 0b11110, 0b10100, 0b10010, 0b10001], // R
        [0b01110, 0b10001, 0b10000, 0b01110, 0b00001, 0b10001, 0b01110], // S
        [0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100], // T
        [0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110], // U
        [0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01010, 0b00100], // V
        [0b10001, 0b10001, 0b10001, 0b10101, 0b10101, 0b11011, 0b10001], // W
        [0b10001, 0b10001, 0b01010, 0b00100, 0b01010, 0b10001, 0b10001], // X
        [0b10001, 0b10001, 0b01010, 0b00100, 0b00100, 0b00100, 0b00100], // Y
        [0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b10000, 0b11111], // Z
        // [ (91)
        [0b01110, 0b01000, 0b01000, 0b01000, 0b01000, 0b01000, 0b01110],
        // \ (92)
        [0b10000, 0b01000, 0b00100, 0b00010, 0b00001, 0b00000, 0b00000],
        // ] (93)
        [0b01110, 0b00010, 0b00010, 0b00010, 0b00010, 0b00010, 0b01110],
        // ^ (94)
        [0b00100, 0b01010, 0b10001, 0b00000, 0b00000, 0b00000, 0b00000],
        // _ (95)
        [0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b11111],
        // ` (96)
        [0b01000, 0b00100, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000],
        // a-z (97-122)
        [0b00000, 0b00000, 0b01110, 0b00001, 0b01111, 0b10001, 0b01111], // a
        [0b10000, 0b10000, 0b11110, 0b10001, 0b10001, 0b10001, 0b11110], // b
        [0b00000, 0b00000, 0b01110, 0b10000, 0b10000, 0b10001, 0b01110], // c
        [0b00001, 0b00001, 0b01111, 0b10001, 0b10001, 0b10001, 0b01111], // d
        [0b00000, 0b00000, 0b01110, 0b10001, 0b11111, 0b10000, 0b01110], // e
        [0b00110, 0b01001, 0b01000, 0b11110, 0b01000, 0b01000, 0b01000], // f
        [0b00000, 0b01111, 0b10001, 0b10001, 0b01111, 0b00001, 0b01110], // g
        [0b10000, 0b10000, 0b10110, 0b11001, 0b10001, 0b10001, 0b10001], // h
        [0b00100, 0b00000, 0b01100, 0b00100, 0b00100, 0b00100, 0b01110], // i
        [0b00010, 0b00000, 0b00110, 0b00010, 0b00010, 0b10010, 0b01100], // j
        [0b10000, 0b10000, 0b10010, 0b10100, 0b11000, 0b10100, 0b10010], // k
        [0b01100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110], // l
        [0b00000, 0b00000, 0b11010, 0b10101, 0b10101, 0b10101, 0b10001], // m
        [0b00000, 0b00000, 0b10110, 0b11001, 0b10001, 0b10001, 0b10001], // n
        [0b00000, 0b00000, 0b01110, 0b10001, 0b10001, 0b10001, 0b01110], // o
        [0b00000, 0b11110, 0b10001, 0b10001, 0b11110, 0b10000, 0b10000], // p
        [0b00000, 0b01111, 0b10001, 0b10001, 0b01111, 0b00001, 0b00001], // q
        [0b00000, 0b00000, 0b10110, 0b11001, 0b10000, 0b10000, 0b10000], // r
        [0b00000, 0b00000, 0b01111, 0b10000, 0b01110, 0b00001, 0b11110], // s
        [0b01000, 0b01000, 0b11110, 0b01000, 0b01000, 0b01001, 0b00110], // t
        [0b00000, 0b00000, 0b10001, 0b10001, 0b10001, 0b10011, 0b01101], // u
        [0b00000, 0b00000, 0b10001, 0b10001, 0b10001, 0b01010, 0b00100], // v
        [0b00000, 0b00000, 0b10001, 0b10001, 0b10101, 0b10101, 0b01010], // w
        [0b00000, 0b00000, 0b10001, 0b01010, 0b00100, 0b01010, 0b10001], // x
        [0b00000, 0b10001, 0b10001, 0b10001, 0b01111, 0b00001, 0b01110], // y
        [0b00000, 0b00000, 0b11111, 0b00010, 0b00100, 0b01000, 0b11111], // z
        // { (123)
        [0b00110, 0b01000, 0b01000, 0b10000, 0b01000, 0b01000, 0b00110],
        // | (124)
        [0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100],
        // } (125)
        [0b01100, 0b00010, 0b00010, 0b00001, 0b00010, 0b00010, 0b01100],
        // ~ (126)
        [0b00000, 0b01000, 0b10101, 0b00010, 0b00000, 0b00000, 0b00000],
    ]
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // Glyph Atlas Tests
    // =========================================================================

    #[test]
    fn test_glyph_atlas_new() {
        let atlas = GlyphAtlas::new();
        assert_eq!(atlas.glyph_count(), GLYPH_COUNT);
    }

    #[test]
    fn test_glyph_atlas_contains_ascii() {
        let atlas = GlyphAtlas::new();
        assert!(atlas.contains('A'));
        assert!(atlas.contains('z'));
        assert!(atlas.contains('0'));
        assert!(atlas.contains(' '));
        assert!(atlas.contains('~'));
    }

    #[test]
    fn test_glyph_atlas_get() {
        let atlas = GlyphAtlas::new();
        let glyph = atlas.get('A');
        assert!(glyph.is_some());
        let g = glyph.unwrap();
        assert!(g.u >= 0.0 && g.u <= 1.0);
        assert!(g.v >= 0.0 && g.v <= 1.0);
    }

    #[test]
    fn test_glyph_atlas_get_or_default() {
        let atlas = GlyphAtlas::new();
        let glyph = atlas.get_or_default('A');
        assert!(glyph.advance > 0.0);
    }

    #[test]
    fn test_glyph_atlas_non_ascii_fallback() {
        let atlas = GlyphAtlas::new();
        let glyph = atlas.get_or_default('😀'); // Emoji not in atlas
        // Should return space glyph
        assert!(glyph.advance > 0.0);
    }

    #[test]
    fn test_glyph_uv_bounds() {
        let atlas = GlyphAtlas::new();
        for c in ' '..='~' {
            if let Some(g) = atlas.get(c) {
                assert!(g.u >= 0.0 && g.u <= 1.0, "u out of bounds for '{}'", c);
                assert!(g.v >= 0.0 && g.v <= 1.0, "v out of bounds for '{}'", c);
                assert!(g.u + g.u_size <= 1.0, "u+u_size out of bounds for '{}'", c);
                assert!(g.v + g.v_size <= 1.0, "v+v_size out of bounds for '{}'", c);
            }
        }
    }

    // =========================================================================
    // Text Instance Tests
    // =========================================================================

    #[test]
    fn test_text_instance_size() {
        // Ensure proper alignment for GPU
        assert_eq!(std::mem::size_of::<TextInstance>() % 16, 0);
    }

    #[test]
    fn test_text_instance_new() {
        let glyph = GlyphInfo {
            u: 0.1, v: 0.2, u_size: 0.05, v_size: 0.1, advance: 8.0
        };
        let instance = TextInstance::new(10.0, 20.0, &glyph, [1.0, 1.0, 1.0, 1.0], 1.0);
        assert_eq!(instance.position, [10.0, 20.0]);
        assert_eq!(instance.uv[0], 0.1);
        assert_eq!(instance.scale, 1.0);
    }

    // =========================================================================
    // Text Layout Tests
    // =========================================================================

    #[test]
    fn test_layout_text_empty() {
        let atlas = GlyphAtlas::new();
        let layout = TextLayout::default();
        let instances = layout_text("", &layout, &atlas);
        assert!(instances.is_empty());
    }

    #[test]
    fn test_layout_text_single_char() {
        let atlas = GlyphAtlas::new();
        let layout = TextLayout::default();
        let instances = layout_text("A", &layout, &atlas);
        assert_eq!(instances.len(), 1);
    }

    #[test]
    fn test_layout_text_multiple_chars() {
        let atlas = GlyphAtlas::new();
        let layout = TextLayout::default();
        let instances = layout_text("Hello", &layout, &atlas);
        assert_eq!(instances.len(), 5);
    }

    #[test]
    fn test_layout_text_left_alignment() {
        let atlas = GlyphAtlas::new();
        let layout = TextLayout {
            x: 100.0,
            y: 50.0,
            align: TextAlign::Left,
            ..Default::default()
        };
        let instances = layout_text("AB", &layout, &atlas);
        assert_eq!(instances[0].position[0], 100.0);
    }

    #[test]
    fn test_layout_text_center_alignment() {
        let atlas = GlyphAtlas::new();
        let layout = TextLayout {
            x: 100.0,
            y: 50.0,
            scale: 1.0,
            align: TextAlign::Center,
            ..Default::default()
        };
        let instances = layout_text("AB", &layout, &atlas);
        // Centered around 100.0
        let text_width = 2.0 * GLYPH_WIDTH as f32;
        assert_eq!(instances[0].position[0], 100.0 - text_width / 2.0);
    }

    #[test]
    fn test_layout_text_right_alignment() {
        let atlas = GlyphAtlas::new();
        let layout = TextLayout {
            x: 100.0,
            y: 50.0,
            scale: 1.0,
            align: TextAlign::Right,
            ..Default::default()
        };
        let instances = layout_text("AB", &layout, &atlas);
        let text_width = 2.0 * GLYPH_WIDTH as f32;
        assert_eq!(instances[0].position[0], 100.0 - text_width);
    }

    #[test]
    fn test_layout_text_scale() {
        let atlas = GlyphAtlas::new();
        let layout = TextLayout {
            scale: 2.0,
            ..Default::default()
        };
        let instances = layout_text("AB", &layout, &atlas);
        // Second char should be spaced by scaled advance
        let expected_x = GLYPH_WIDTH as f32 * 2.0;
        assert!((instances[1].position[0] - expected_x).abs() < 0.01);
    }

    // =========================================================================
    // Utility Function Tests
    // =========================================================================

    #[test]
    fn test_text_width() {
        let width = text_width("Hello", 1.0);
        assert_eq!(width, 5.0 * GLYPH_WIDTH as f32);
    }

    #[test]
    fn test_text_width_scaled() {
        let width = text_width("Hi", 2.0);
        assert_eq!(width, 2.0 * GLYPH_WIDTH as f32 * 2.0);
    }

    #[test]
    fn test_is_text_visible() {
        // At normal scale and zoom, text should be visible
        assert!(is_text_visible(1.0, 1.0));
        // At very small scale, text should be hidden
        assert!(!is_text_visible(0.01, 1.0));
        // Small scale + high zoom = visible
        assert!(is_text_visible(0.1, 10.0));
    }

    // =========================================================================
    // LabelData Tests
    // =========================================================================

    #[test]
    fn test_label_data_new() {
        let label = LabelData::new("R1", 10.0, 20.0);
        assert_eq!(label.text, "R1");
        assert_eq!(label.x, 10.0);
        assert_eq!(label.y, 20.0);
    }

    #[test]
    fn test_label_data_builder() {
        let label = LabelData::new("C1", 0.0, 0.0)
            .with_color([1.0, 0.0, 0.0, 1.0])
            .with_align(TextAlign::Center)
            .with_scale(0.5);

        assert_eq!(label.color, [1.0, 0.0, 0.0, 1.0]);
        assert_eq!(label.align, TextAlign::Center);
        assert_eq!(label.scale, 0.5);
    }

    #[test]
    fn test_label_data_to_instances() {
        let atlas = GlyphAtlas::new();
        let label = LabelData::new("Test", 10.0, 20.0);
        let instances = label.to_instances(&atlas);
        assert_eq!(instances.len(), 4);
    }

    // =========================================================================
    // Bitmap Atlas Tests
    // =========================================================================

    #[test]
    fn test_generate_bitmap_atlas_size() {
        let data = generate_bitmap_atlas();
        assert_eq!(data.len(), (ATLAS_WIDTH * ATLAS_HEIGHT) as usize);
    }

    #[test]
    fn test_generate_bitmap_atlas_has_content() {
        let data = generate_bitmap_atlas();
        // Should have some non-zero pixels (glyphs)
        let non_zero_count = data.iter().filter(|&&x| x > 0).count();
        assert!(non_zero_count > 0, "Atlas should contain rendered glyphs");
    }

    #[test]
    fn test_font_data_count() {
        let font = get_font_data();
        assert_eq!(font.len(), GLYPH_COUNT);
    }

    #[test]
    fn test_font_data_pattern_size() {
        let font = get_font_data();
        for (i, pattern) in font.iter().enumerate() {
            assert_eq!(pattern.len(), 7, "Glyph {} should have 7 rows", i);
        }
    }

    // =========================================================================
    // Constants Tests
    // =========================================================================

    #[test]
    fn test_atlas_dimensions_power_of_two() {
        // GPU textures typically prefer power-of-two dimensions
        assert!(ATLAS_WIDTH.is_power_of_two());
        assert!(ATLAS_HEIGHT.is_power_of_two());
    }

    #[test]
    fn test_glyph_fits_in_atlas() {
        let glyphs_per_row = ATLAS_WIDTH / GLYPH_WIDTH;
        let rows_needed = (GLYPH_COUNT as u32 + glyphs_per_row - 1) / glyphs_per_row;
        let height_needed = rows_needed * GLYPH_HEIGHT;
        assert!(height_needed <= ATLAS_HEIGHT);
    }
}
