use crate::{
    Color, Image, PixelFormat, Rectangle, Vector2, draw_texture, draw_texture_ex, draw_texture_n_patch,
    draw_texture_pro, draw_texture_rec, draw_texture_v, gen_texture_mipmaps, is_render_texture_valid, is_texture_valid,
    load_render_texture, load_texture, load_texture_cubemap, load_texture_from_image, set_texture_filter,
    set_texture_wrap, unload_render_texture, unload_texture, update_texture, update_texture_rec,
};

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureFilter {
    Point = 0,
    Bilinear,
    Trilinear,
    Anisotropic4x,
    Anisotropic8x,
    Anisotropic16x,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureWrap {
    Repeat = 0,
    Clamp,
    MirrorRepeat,
    MirrorClamp,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CubemapLayout {
    AutoDetect = 0,
    LineVertical,
    LineHorizontal,
    CrossThreeByFour,
    CrossFourByThree,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NPatchLayout {
    NinePatch = 0,        // Npatch layout: 3x3 tiles
    ThreePatchVertical,   // Npatch layout: 1x3 tiles
    ThreePatchHorizontal, // Npatch layout: 3x1 tiles
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NPatchInfo {
    pub source: Rectangle, // Texture source rectangle
    pub left: i32,         // Left border offset
    pub top: i32,          // Top border offset
    pub right: i32,        // Right border offset
    pub bottom: i32,       // Bottom border offset
    pub layout: i32,       // Layout of the n-patch (NPatchLayout)
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Texture {
    pub id: u32,             // OpenGL texture id
    pub width: i32,          // Texture base width
    pub height: i32,         // Texture base height
    pub mipmaps: i32,        // Mipmap levels, 1 by default
    pub format: PixelFormat, // Data format (PixelFormat type)
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Texture2D(Texture);

impl Texture2D {
    // Loading

    pub fn load(filename: &str) -> Texture2D {
        return load_texture(filename);
    }

    pub fn from_image(image: Image) -> Texture2D {
        return load_texture_from_image(image);
    }

    pub fn unload(&self) {
        unload_texture(*self);
    }

    pub fn is_valid(&self) -> bool {
        return is_texture_valid(*self);
    }

    pub fn update(&self, pixels: &[Color]) {
        update_texture(*self, pixels);
    }

    pub fn update_rec(&self, rec: Rectangle, pixels: &[Color]) {
        update_texture_rec(*self, rec, pixels);
    }

    // Configuration

    pub fn gen_mipmaps(texture: &mut Texture2D) {
        gen_texture_mipmaps(texture);
    }

    pub fn set_filter(&self, filter: TextureFilter) {
        set_texture_filter(*self, filter);
    }

    pub fn set_wrap(&self, wrap: TextureWrap) {
        set_texture_wrap(*self, wrap);
    }

    // Drawing

    pub fn draw(&self, pos_x: i32, pos_y: i32, tint: Color) {
        draw_texture(*self, pos_x, pos_y, tint);
    }

    pub fn draw_v(&self, position: Vector2, tint: Color) {
        draw_texture_v(*self, position, tint);
    }

    pub fn draw_ex(&self, position: Vector2, rotation: f32, scale: f32, tint: Color) {
        draw_texture_ex(*self, position, rotation, scale, tint);
    }

    pub fn draw_rec(&self, source: Rectangle, position: Vector2, tint: Color) {
        draw_texture_rec(*self, source, position, tint);
    }

    pub fn draw_pro(&self, source: Rectangle, dest: Rectangle, origin: Vector2, rotation: f32, tint: Color) {
        draw_texture_pro(*self, source, dest, origin, rotation, tint);
    }

    pub fn draw_n_patch(&self, n_patch_info: NPatchInfo, dest: Rectangle, origin: Vector2, rotation: f32, tint: Color) {
        draw_texture_n_patch(*self, n_patch_info, dest, origin, rotation, tint);
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct TextureCubemap(Texture);

impl TextureCubemap {
    pub fn load(image: Image, layout: CubemapLayout) -> TextureCubemap {
        return load_texture_cubemap(image, layout);
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RenderTexture {
    pub id: u32,          // OpenGL framebuffer object id
    pub texture: Texture, // Color buffer attachment texture
    pub depth: Texture,   // Depth buffer attachment texture
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RenderTexture2D(RenderTexture);

impl RenderTexture2D {
    pub fn load(width: i32, height: i32) -> RenderTexture2D {
        return load_render_texture(width, height);
    }

    pub fn valid(self) -> bool {
        return is_render_texture_valid(self);
    }

    pub fn unload(self) {
        unload_render_texture(self);
    }
}
