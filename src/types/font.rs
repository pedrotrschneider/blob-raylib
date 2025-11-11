use crate::{Image, Rectangle, Texture2D};

/// Font type, defines generation method
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontType {
    /// Default font generation, anti-aliased
    FontDefault = 0,
    /// Bitmap font generation, no anti-aliasing
    FontBitmap,
    /// SDF font generation, requires external shader
    FontSdf,
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
