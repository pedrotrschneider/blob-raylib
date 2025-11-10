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
    /// Light Gray
    pub const LIGHTGRAY: Color = Color {
        r: 200,
        g: 200,
        b: 200,
        a: 255,
    };

    /// Gray
    pub const GRAY: Color = Color {
        r: 130,
        g: 130,
        b: 130,
        a: 255,
    };

    /// Dark Gray
    pub const DARKGRAY: Color = Color {
        r: 80,
        g: 80,
        b: 80,
        a: 255,
    };

    /// Yellow
    pub const YELLOW: Color = Color {
        r: 253,
        g: 249,
        b: 0,
        a: 255,
    };

    /// Gold
    pub const GOLD: Color = Color {
        r: 255,
        g: 203,
        b: 0,
        a: 255,
    };

    /// Orange
    pub const ORANGE: Color = Color {
        r: 255,
        g: 161,
        b: 0,
        a: 255,
    };

    /// Pink
    pub const PINK: Color = Color {
        r: 255,
        g: 109,
        b: 194,
        a: 255,
    };

    /// Red
    pub const RED: Color = Color {
        r: 230,
        g: 41,
        b: 55,
        a: 255,
    };

    /// Maroon
    pub const MAROON: Color = Color {
        r: 190,
        g: 33,
        b: 55,
        a: 255,
    };

    /// Green
    pub const GREEN: Color = Color {
        r: 0,
        g: 228,
        b: 48,
        a: 255,
    };

    /// Lime
    pub const LIME: Color = Color {
        r: 0,
        g: 158,
        b: 47,
        a: 255,
    };

    /// Dark Green
    pub const DARKGREEN: Color = Color {
        r: 0,
        g: 117,
        b: 44,
        a: 255,
    };

    /// Sky Blue
    pub const SKYBLUE: Color = Color {
        r: 102,
        g: 191,
        b: 255,
        a: 255,
    };

    /// Blue
    pub const BLUE: Color = Color {
        r: 0,
        g: 121,
        b: 241,
        a: 255,
    };

    /// Dark Blue
    pub const DARKBLUE: Color = Color {
        r: 0,
        g: 82,
        b: 172,
        a: 255,
    };

    /// Purple
    pub const PURPLE: Color = Color {
        r: 200,
        g: 122,
        b: 255,
        a: 255,
    };

    /// Violet
    pub const VIOLET: Color = Color {
        r: 135,
        g: 60,
        b: 190,
        a: 255,
    };

    /// Dark Purple
    pub const DARKPURPLE: Color = Color {
        r: 112,
        g: 31,
        b: 126,
        a: 255,
    };

    /// Beige
    pub const BEIGE: Color = Color {
        r: 211,
        g: 176,
        b: 131,
        a: 255,
    };

    /// Brown
    pub const BROWN: Color = Color {
        r: 127,
        g: 106,
        b: 79,
        a: 255,
    };

    /// Dark Brown
    pub const DARKBROWN: Color = Color {
        r: 76,
        g: 63,
        b: 47,
        a: 255,
    };

    /// White
    pub const WHITE: Color = Color {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    };

    /// Black
    pub const BLACK: Color = Color {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    };

    /// Blank
    pub const BLANK: Color = Color { r: 0, g: 0, b: 0, a: 0 };

    /// Transparent
    pub const TRANSPARENT: Color = Color { r: 0, g: 0, b: 0, a: 0 };

    /// Magenta
    pub const MAGENTA: Color = Color {
        r: 255,
        g: 0,
        b: 255,
        a: 255,
    };

    /// My own White (raylib logo)
    pub const RAYWHITE: Color = Color {
        r: 245,
        g: 245,
        b: 245,
        a: 255,
    };

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

    pub fn zero() -> Self {
        return Self { x: 0.0, y: 0.0 };
    }
}

impl From<(f32, f32)> for Vector2 {
    fn from((x, y): (f32, f32)) -> Self {
        return Vector2::new(x, y);
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        return Self { x, y, z };
    }

    pub fn zero() -> Self {
        return Self { x: 0.0, y: 0.0, z: 0.0 };
    }
}

impl From<(f32, f32, f32)> for Vector3 {
    fn from((x, y, z): (f32, f32, f32)) -> Self {
        return Vector3::new(x, y, z);
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vector4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        return Self { x, y, z, w };
    }

    pub fn zero() -> Self {
        return Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        };
    }
}

impl From<(f32, f32, f32, f32)> for Vector4 {
    fn from((x, y, z, w): (f32, f32, f32, f32)) -> Self {
        return Vector4::new(x, y, z, w);
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Quaternion(Vector4);

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Matrix {
    pub m0: f32,
    pub m4: f32,
    pub m8: f32,
    pub m12: f32,
    pub m1: f32,
    pub m5: f32,
    pub m9: f32,
    pub m13: f32,
    pub m2: f32,
    pub m6: f32,
    pub m10: f32,
    pub m14: f32,
    pub m3: f32,
    pub m7: f32,
    pub m11: f32,
    pub m15: f32,
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

/// Camera system modes
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CameraMode {
    /// Camera custom, controlled by user (UpdateCamera() does nothing)
    Custom,
    /// Camera free mode
    Free,
    /// Camera orbital, around target, zoom supported
    Orbital,
    /// Camera first person
    FirstPerson,
    /// Camera third person
    ThirdPerson,
}

/// Camera projection
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CameraProjection {
    /// Perspective projection
    Perspective = 0,
    /// Orthographic projection
    Orthographic = 1,
}

/// Camera, defines position/orientation in 3d space
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Camera3D {
    /// Camera position
    pub position: Vector3,
    /// Camera target it looks-at
    pub target: Vector3,
    /// Camera up vector (rotation over its axis)
    pub up: Vector3,
    /// Camera field-of-view aperture in Y (degrees) in perspective,
    /// used as near plane height in world units in orthographic
    pub fovy: f32,
    /// Camera projection: CAMERA_PERSPECTIVE or CAMERA_ORTHOGRAPHIC
    pub projection: CameraProjection,
}

/// Camera type fallback, defaults to Camera3D
pub type Camera = Camera3D;

/// Camera2D, defines position/orientation in 2d space
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Camera2D {
    /// Camera offset (displacement from target)
    pub offset: Vector2,
    /// Camera target (rotation and zoom origin)
    pub target: Vector2,
    /// Camera rotation in degrees
    pub rotation: f32,
    /// Camera zoom (scaling), should be 1.0f by default
    pub zoom: f32,
}

/// Shader
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Shader {
    /// Shader program id
    pub id: ::std::os::raw::c_uint,
    /// Shader locations array (RL_MAX_SHADER_LOCATIONS)
    pub locs: *mut ::std::os::raw::c_int,
}

/// Color blending modes (pre-defined)
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlendMode {
    /// Blend textures considering alpha (default)
    Alpha = 0,
    /// Blend textures adding colors
    Additive,
    /// Blend textures multiplying colors
    Multiplied,
    /// Blend textures adding colors (alternative)
    AddColors,
    /// Blend textures subtracting colors (alternative)
    SubtractColors,
    /// Blend premultiplied textures considering alpha
    AlphaPremultiply,
    /// Blend textures using custom src/dst factors (use rlSetBlendFactors())
    Custom,
    /// Blend textures using custom rgb/alpha separate src/dst factors (use rlSetBlendFactorsSeparate())
    CustomSeparate,
}

/// VrDeviceInfo, Head-Mounted-Display device parameters
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VrDeviceInfo {
    /// Horizontal resolution in pixels
    pub h_resolution: ::std::os::raw::c_int,
    /// Vertical resolution in pixels
    pub v_resolution: ::std::os::raw::c_int,
    /// Horizontal size in meters
    pub h_screen_size: f32,
    /// Vertical size in meters
    pub v_screen_size: f32,
    /// Distance between eye and display in meters
    pub eye_to_screen_distance: f32,
    /// Lens separation distance in meters
    pub lens_separation_distance: f32,
    /// IPD (distance between pupils) in meters
    pub interpupillary_distance: f32,
    /// Lens distortion constant parameters
    pub lens_distortion_values: [f32; 4],
    /// Chromatic aberration correction parameters
    pub chroma_ab_correction: [f32; 4],
}

/// VrStereoConfig, VR stereo rendering configuration for simulator
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VrStereoConfig {
    /// VR projection matrices (per eye)
    pub projection: [Matrix; 2],
    /// VR view offset matrices (per eye)
    pub view_offset: [Matrix; 2],
    /// VR left lens center
    pub left_lens_center: [f32; 2],
    /// VR right lens center
    pub right_lens_center: [f32; 2],
    /// VR left screen center
    pub left_screen_center: [f32; 2],
    /// VR right screen center
    pub right_screen_center: [f32; 2],
    /// VR distortion scale
    pub scale: [f32; 2],
    /// VR distortion scale in
    pub scale_in: [f32; 2],
}
