mod config_flag;
mod image;
mod input;
mod texture;

pub use config_flag::*;
pub use image::*;
pub use input::*;
pub use texture::*;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        return Self { r, g, b, a };
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Rectangle {
    x: f32,      // Rectangle top-left corner position x
    y: f32,      // Rectangle top-left corner position y
    width: f32,  // Rectangle width
    height: f32, // Rectangle height
}

impl Rectangle {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        return Self { x, y, width, height };
    }
}

impl From<(f32, f32, f32, f32)> for Rectangle {
    fn from((x, y, width, height): (f32, f32, f32, f32)) -> Self {
        return Self::new(x, y, width, height);
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        return Self { x, y };
    }
}

impl From<(f32, f32)> for Vector2 {
    fn from((x, y): (f32, f32)) -> Self {
        return Vector2::new(x, y);
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GlyphInfo {
    pub value: i32,     // Character value (Unicode)
    pub offset_x: i32,  // Character offset X when drawing
    pub offset_y: i32,  // Character offset Y when drawing
    pub advance_x: i32, // Character advance position X
    pub image: Image,   // Character image data
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Font {
    pub base_size: i32,         // Base size (default chars height)
    pub glyph_count: i32,       // Number of glyph characters
    pub glyph_padding: i32,     // Padding around the glyph characters
    pub texture: Texture2D,     // Texture atlas containing the glyphs
    pub recs: *mut Rectangle,   // Rectangles in texture for the glyphs
    pub glyphs: *mut GlyphInfo, // Glyphs info data
}