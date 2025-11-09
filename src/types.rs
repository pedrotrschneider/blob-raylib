use crate::{draw_texture, draw_texture_ex, draw_texture_n_patch, draw_texture_pro, draw_texture_rec, draw_texture_v, export_image, export_image_to_memory, gen_image_cellular, gen_image_checked, gen_image_color, gen_image_gradient_linear, gen_image_gradient_radial, gen_image_gradient_square, gen_image_perlin_noise, gen_image_text, gen_image_white_noise, gen_texture_mipmaps, get_gamepad_axis_count, get_gamepad_axis_movement, get_gamepad_name, get_image_alpha_border, get_image_color, image_alpha_clear, image_alpha_crop, image_alpha_mask, image_alpha_premultiply, image_blur_gaussian, image_clear_background, image_color_brightness, image_color_contrast, image_color_grayscale, image_color_invert, image_color_replace, image_color_tint, image_copy, image_crop, image_dither, image_draw, image_draw_circle, image_draw_circle_lines, image_draw_circle_lines_v, image_draw_circle_v, image_draw_line, image_draw_line_ex, image_draw_line_v, image_draw_pixel, image_draw_pixel_v, image_draw_rectangle, image_draw_rectangle_lines, image_draw_rectangle_rec, image_draw_rectangle_v, image_draw_text, image_draw_text_ex, image_draw_triangle, image_draw_triangle_ex, image_draw_triangle_fan, image_draw_triangle_lines, image_draw_triangle_strip, image_flip_horizontal, image_flip_vertical, image_format, image_from_channel, image_from_image, image_kernel_convolution, image_mipmaps, image_resize, image_resize_canvas, image_resize_nn, image_rotate, image_rotate_ccw, image_rotate_cw, image_text, image_text_ex, image_to_pot, is_gamepad_available, is_gamepad_button_down, is_gamepad_button_pressed, is_gamepad_button_released, is_gamepad_button_up, is_image_valid, is_render_texture_valid, is_texture_valid, load_image, load_image_anim, load_image_anim_from_memory, load_image_colors, load_image_from_memory, load_image_from_screen, load_image_from_texture, load_image_palette, load_image_raw, load_render_texture, load_texture, load_texture_cubemap, load_texture_from_image, set_gamepad_vibration, set_texture_filter, set_texture_wrap, unload_image, unload_render_texture, unload_texture, update_texture, update_texture_rec};
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};
use std::os::raw::c_void;

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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Image {
    pub data: *mut c_void,          // Image raw data
    pub width: i32,                 // Image base width
    pub height: i32,                // Image base height
    pub mipmaps: i32,               // Mipmap levels, 1 by default
    pub format: PixelFormat,        // Data format (PixelFormat type)
}

impl Image {
    // Loading

    pub fn load(filename: &str) -> Image {
        return load_image(filename);
    }

    pub fn load_raw(filename: &str, width: i32, height: i32, format: PixelFormat, header_size: i32) -> Image {
        return load_image_raw(filename, width, height, format, header_size);
    }

    pub fn load_anim(filename: &str, frames: &mut i32) -> Image {
        return load_image_anim(filename, frames);
    }

    pub fn load_anim_from_memory(file_type: &str, file_data: &[u8], frames: &mut i32) -> Image {
        return load_image_anim_from_memory(file_type, file_data, frames);
    }

    pub fn load_from_memory(file_type: &str, file_data: &[u8]) -> Image {
        return load_image_from_memory(file_type, file_data);
    }

    pub fn from_texture(texture: Texture2D) -> Image {
        return load_image_from_texture(texture);
    }

    pub fn from_screen() -> Image {
        return load_image_from_screen();
    }

    pub fn is_valid(&self) -> bool {
        return is_image_valid(*self);
    }

    pub fn unload(&self) {
        unload_image(*self);
    }

    pub fn export(&self, filename: &str) -> bool {
        return export_image(*self, filename);
    }

    pub fn export_to_memory(&self, file_type: &str) -> Vec<u8> {
        return export_image_to_memory(*self, file_type);
    }

    // Generation

    pub fn gen_color(width: i32, height: i32, color: Color) -> Image {
        return gen_image_color(width, height, color);
    }

    pub fn gen_linear_gradient(width: i32, height: i32, direction: i32, start: Color, end: Color) -> Image {
        return gen_image_gradient_linear(width, height, direction, start, end);
    }

    pub fn gen_radial_gradient(width: i32, height: i32, density: f32, inner: Color, outer: Color) -> Image {
        return gen_image_gradient_radial(width, height, density, inner, outer);
    }

    pub fn gen_square_gradient(width: i32, height: i32, density: f32, inner: Color, outer: Color) -> Image {
        return gen_image_gradient_square(width, height, density, inner, outer);
    }

    pub fn gen_checked(width: i32, height: i32, checks_x: i32, checks_y: i32, col1: Color, col2: Color) -> Image {
        return gen_image_checked(width, height, checks_x, checks_y, col1, col2);
    }

    pub fn gen_white_noise(width: i32, height: i32, factor: f32) -> Image {
        return gen_image_white_noise(width, height, factor);
    }

    pub fn gen_perlin_noise(width: i32, height: i32, offset_x: i32, offset_y: i32, scale: f32) -> Image {
        return gen_image_perlin_noise(width, height, offset_x, offset_y, scale);
    }

    pub fn gen_cellular(width: i32, height: i32, tile_size: i32) -> Image {
        return gen_image_cellular(width, height, tile_size);
    }

    pub fn gen_text(width: i32, height: i32, text: &str) -> Image {
        return gen_image_text(width, height, text);
    }

    // Manipulation

    pub fn copy(&self) -> Image {
        return image_copy(*self);
    }

    pub fn from_image(image: Image, rec: Rectangle) -> Image {
        return image_from_image(image, rec);
    }

    pub fn from_channel(image: Image, selected_channel: i32) -> Image {
        return image_from_channel(image, selected_channel);
    }

    pub fn from_text(text: &str, font_size: i32, color: Color) -> Image {
        return image_text(text, font_size, color);
    }

    pub fn from_text_ex(font: Font, text: &str, font_size: f32, spacing: f32, tint: Color) -> Image {
        return image_text_ex(font, text, font_size, spacing, tint);
    }

    pub fn change_format(&mut self, new_format: PixelFormat) {
        return image_format(self, new_format);
    }

    pub fn pad_to_power_of_two(&mut self, fill: Color) {
        image_to_pot(self, fill);
    }

    pub fn crop(&mut self, crop: Rectangle) {
        image_crop(self, crop);
    }

    pub fn alpha_crop(&mut self, threshold: f32) {
        image_alpha_crop(self, threshold);
    }

    pub fn alpha_clear(&mut self, color: Color, threshold: f32) {
        image_alpha_clear(self, color, threshold);
    }

    pub fn alpha_mask(&mut self, alpha_mask: Image) {
        image_alpha_mask(self, alpha_mask);
    }

    pub fn alpha_premultiply(&mut self) {
        image_alpha_premultiply(self);
    }

    pub fn blur_gaussian(&mut self, blur_size: i32) {
        image_blur_gaussian(self, blur_size);
    }

    pub fn kernel_convolution(&mut self, kernel: &[f32]) {
        image_kernel_convolution(self, kernel, kernel.len() as i32);
    }

    pub fn resize(&mut self, new_width: i32, new_height: i32) {
        image_resize(self, new_width, new_height);
    }

    pub fn resize_nearest_neighbor(&mut self, new_width: i32, new_height: i32) {
        image_resize_nn(self, new_width, new_height);
    }

    pub fn resize_canvas(&mut self, new_width: i32, new_height: i32, offset_x: i32, offset_y: i32, fill: Color) {
        image_resize_canvas(self, new_width, new_height, offset_x, offset_y, fill);
    }

    pub fn gen_mipmaps(&mut self) {
        image_mipmaps(self);
    }

    pub fn dither(&mut self, r_bpp: i32, g_bpp: i32, b_bpp: i32, a_bpp: i32) {
        image_dither(self, r_bpp, g_bpp, b_bpp, a_bpp);
    }

    pub fn flip_vertical(&mut self) {
        image_flip_vertical(self);
    }

    pub fn flip_horizontal(&mut self) {
        image_flip_horizontal(self);
    }

    pub fn rotate(&mut self, degrees: i32) {
        image_rotate(self, degrees);
    }

    pub fn rotate_clockwise(&mut self) {
        image_rotate_cw(self);
    }

    pub fn rotate_counter_clockwise(&mut self) {
        image_rotate_ccw(self);
    }

    pub fn color_tint(&mut self, color: Color) {
        image_color_tint(self, color);
    }

    pub fn color_invert(&mut self) {
        image_color_invert(self);
    }

    pub fn color_grayscale(&mut self) {
        image_color_grayscale(self);
    }

    pub fn color_contrast(&mut self, contrast: f32) {
        image_color_contrast(self, contrast);
    }

    pub fn color_brightness(&mut self, brightness: i32) {
        image_color_brightness(self, brightness);
    }

    pub fn color_replace(&mut self, color: Color, replace: Color) {
        image_color_replace(self, color, replace);
    }

    pub fn load_colors(&self) -> &'static mut [Color] {
        return load_image_colors(*self);
    }

    pub fn load_palette(&self, max_palette_size: i32) -> &'static mut [Color] {
        return load_image_palette(*self, max_palette_size);
    }

    pub fn get_alpha_border(&self, threshold: f32) -> Rectangle {
        return get_image_alpha_border(*self, threshold);
    }

    pub fn get_color(&self, x: i32, y: i32) -> Color {
        return get_image_color(*self, x, y);
    }

    // Drawing

    pub fn clear_background(&mut self, color: Color) {
        image_clear_background(self, color);
    }

    pub fn draw_pixel(&mut self, pos_x: i32, pos_y: i32, color: Color) {
        image_draw_pixel(self, pos_x, pos_y, color);
    }

    pub fn draw_pixel_v(&mut self, position: Vector2, color: Color) {
        image_draw_pixel_v(self, position, color);
    }

    pub fn draw_line(&mut self, start_pos_x: i32, start_pos_y: i32, end_pos_x: i32, end_pos_y: i32, color: Color) {
        image_draw_line(self, start_pos_x, start_pos_y, end_pos_x, end_pos_y, color);
    }

    pub fn draw_line_v(&mut self, start: Vector2, end: Vector2, color: Color) {
        image_draw_line_v(self, start, end, color);
    }

    pub fn draw_line_ex(dst: &mut Image, start: Vector2, end: Vector2, thick: i32, color: Color) {
        image_draw_line_ex(dst, start, end, thick, color);
    }

    pub fn draw_circle(&mut self, center_x: i32, center_y: i32, radius: i32, color: Color) {
        image_draw_circle(self, center_x, center_y, radius, color);
    }

    pub fn draw_circle_v(&mut self, center: Vector2, radius: i32, color: Color) {
        image_draw_circle_v(self, center, radius, color);
    }

    pub fn draw_circle_lines(&mut self, center_x: i32, center_y: i32, radius: i32, color: Color) {
        image_draw_circle_lines(self, center_x, center_y, radius, color);
    }

    pub fn draw_circle_lines_v(&mut self, center: Vector2, radius: i32, color: Color) {
        image_draw_circle_lines_v(self, center, radius, color);
    }

    pub fn draw_rectangle(&mut self, pos_x: i32, pos_y: i32, width: i32, height: i32, color: Color) {
        image_draw_rectangle(self, pos_x, pos_y, width, height, color);
    }

    pub fn draw_rectangle_v(&mut self, position: Vector2, size: Vector2, color: Color) {
        image_draw_rectangle_v(self, position, size, color);
    }

    pub fn draw_rectangle_rec(&mut self, rec: Rectangle, color: Color) {
        image_draw_rectangle_rec(self, rec, color);
    }

    pub fn draw_rectangle_lines(&mut self, rec: Rectangle, thick: i32, color: Color) {
        image_draw_rectangle_lines(self, rec, thick, color);
    }

    pub fn draw_triangle(&mut self, v1: Vector2, v2: Vector2, v3: Vector2, color: Color) {
        image_draw_triangle(self, v1, v2, v3, color);
    }

    pub fn draw_triangle_ex(&mut self, v1: Vector2, v2: Vector2, v3: Vector2, c1: Color, c2: Color, c3: Color) {
        image_draw_triangle_ex(self, v1, v2, v3, c1, c2, c3);
    }

    pub fn draw_triangle_lines(&mut self, v1: Vector2, v2: Vector2, v3: Vector2, color: Color) {
        image_draw_triangle_lines(self, v1, v2, v3, color);
    }

    pub fn draw_triangle_fan(&mut self, points: &[Vector2], color: Color) {
        image_draw_triangle_fan(self, points, color);
    }

    pub fn draw_triangle_strip(&mut self, points: &[Vector2], color: Color) {
        image_draw_triangle_strip(self, points, color);
    }

    pub fn draw(&mut self, src: Image, src_rec: Rectangle, dst_rec: Rectangle, tint: Color) {
        image_draw(self, src, src_rec, dst_rec, tint);
    }

    pub fn draw_text(&mut self, text: &str, pos_x: i32, pos_y: i32, font_size: i32, color: Color) {
        image_draw_text(self, text, pos_x, pos_y, font_size, color);
    }

    pub fn draw_text_ex(&mut self, font: Font, text: &str, position: Vector2, font_size: f32, spacing: f32, tint: Color) {
        image_draw_text_ex(self, font, text, position, font_size, spacing, tint);
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Texture {
    pub id: u32,                // OpenGL texture id
    pub width: i32,             // Texture base width
    pub height: i32,            // Texture base height
    pub mipmaps: i32,           // Mipmap levels, 1 by default
    pub format: PixelFormat,    // Data format (PixelFormat type)
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

    pub fn draw_rec(&self, source: Rectangle, position: Vector2, tint:Color) {
        draw_texture_rec(*self, source, position, tint);
    }

    pub fn draw_pro(&self, source: Rectangle, dest: Rectangle, origin: Vector2, rotation: f32, tint: Color) {
        draw_texture_pro(*self, source, dest, origin, rotation, tint);
    }

    pub fn draw_n_patch(&self, n_patch_info: NPatchInfo, dest: Rectangle, origin: Vector2, rotation: f32, tint: Color){
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

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyboardKey {
    Null = 0, // Key: NULL, used for no key pressed

    // Alphanumeric keys
    Apostrophe = 39,   // Key: '
    Comma = 44,        // Key: ,
    Minus = 45,        // Key: -
    Period = 46,       // Key: .
    Slash = 47,        // Key: /
    Zero = 48,         // Key: 0
    One = 49,          // Key: 1
    Two = 50,          // Key: 2
    Three = 51,        // Key: 3
    Four = 52,         // Key: 4
    Five = 53,         // Key: 5
    Six = 54,          // Key: 6
    Seven = 55,        // Key: 7
    Eight = 56,        // Key: 8
    Nine = 57,         // Key: 9
    Semicolon = 59,    // Key: ;
    Equal = 61,        // Key: =
    A = 65,            // Key: A | a
    B = 66,            // Key: B | b
    C = 67,            // Key: C | c
    D = 68,            // Key: D | d
    E = 69,            // Key: E | e
    F = 70,            // Key: F | f
    G = 71,            // Key: G | g
    H = 72,            // Key: H | h
    I = 73,            // Key: I | i
    J = 74,            // Key: J | j
    K = 75,            // Key: K | k
    L = 76,            // Key: L | l
    M = 77,            // Key: M | m
    N = 78,            // Key: N | n
    O = 79,            // Key: O | o
    P = 80,            // Key: P | p
    Q = 81,            // Key: Q | q
    R = 82,            // Key: R | r
    S = 83,            // Key: S | s
    T = 84,            // Key: T | t
    U = 85,            // Key: U | u
    V = 86,            // Key: V | v
    W = 87,            // Key: W | w
    X = 88,            // Key: X | x
    Y = 89,            // Key: Y | y
    Z = 90,            // Key: Z | z
    LeftBracket = 91,  // Key: [
    Backslash = 92,    // Key: '\'
    RightBracket = 93, // Key: ]
    Grave = 96,        // Key: `

    // Function keys
    Space = 32,         // Key: Space
    Escape = 256,       // Key: Esc
    Enter = 257,        // Key: Enter
    Tab = 258,          // Key: Tab
    Backspace = 259,    // Key: Backspace
    Insert = 260,       // Key: Ins
    Delete = 261,       // Key: Del
    Right = 262,        // Key: Cursor right
    Left = 263,         // Key: Cursor left
    Down = 264,         // Key: Cursor down
    Up = 265,           // Key: Cursor up
    PageUp = 266,       // Key: Page up
    PageDown = 267,     // Key: Page down
    Home = 268,         // Key: Home
    End = 269,          // Key: End
    CapsLock = 280,     // Key: Caps lock
    ScrollLock = 281,   // Key: Scroll down
    NumLock = 282,      // Key: Num lock
    PrintScreen = 283,  // Key: Print screen
    Pause = 284,        // Key: Pause
    F1 = 290,           // Key: F1
    F2 = 291,           // Key: F2
    F3 = 292,           // Key: F3
    F4 = 293,           // Key: F4
    F5 = 294,           // Key: F5
    F6 = 295,           // Key: F6
    F7 = 296,           // Key: F7
    F8 = 297,           // Key: F8
    F9 = 298,           // Key: F9
    F10 = 299,          // Key: F10
    F11 = 300,          // Key: F11
    F12 = 301,          // Key: F12
    LeftShift = 340,    // Key: Shift left
    LeftControl = 341,  // Key: Control left
    LeftAlt = 342,      // Key: Alt left
    LeftSuper = 343,    // Key: Super left
    RightShift = 344,   // Key: Shift right
    RightControl = 345, // Key: Control right
    RightAlt = 346,     // Key: Alt right
    RightSuper = 347,   // Key: Super right
    KbMenu = 348,       // Key: KB menu

    // Keypad keys
    Kp0 = 320,        // Key: Keypad 0
    Kp1 = 321,        // Key: Keypad 1
    Kp2 = 322,        // Key: Keypad 2
    Kp3 = 323,        // Key: Keypad 3
    Kp4 = 324,        // Key: Keypad 4
    Kp5 = 325,        // Key: Keypad 5
    Kp6 = 326,        // Key: Keypad 6
    Kp7 = 327,        // Key: Keypad 7
    Kp8 = 328,        // Key: Keypad 8
    Kp9 = 329,        // Key: Keypad 9
    KpDecimal = 330,  // Key: Keypad .
    KpDivide = 331,   // Key: Keypad /
    KpMultiply = 332, // Key: Keypad *
    KpSubtract = 333, // Key: Keypad -
    KpAdd = 334,      // Key: Keypad +
    KpEnter = 335,    // Key: Keypad Enter
    KpEqual = 336,    // Key: Keypad =

    // Android key buttons
    Back = 4,        // Key: Android back button
    Menu = 5,        // Key: Android menu button
    VolumeUp = 24,   // Key: Android volume up button
    VolumeDown = 25, // Key: Android volume down button
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
    Left = 0,    // Mouse button left
    Right = 1,   // Mouse button right
    Middle = 2,  // Mouse button middle (pressed wheel)
    Side = 3,    // Mouse button side (advanced mouse device)
    Extra = 4,   // Mouse button extra (advanced mouse device)
    Forward = 5, // Mouse button forward (advanced mouse device)
    Back = 6,    // Mouse button back (advanced mouse device)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Gamepad {
    pub(crate) id: i32,
}

impl Gamepad {
    pub fn new(id: i32) -> Self {
        return Self { id };
    }

    pub fn next(&mut self) {
        self.id += 1;
    }

    pub fn prev(&mut self) {
        if self.id == 0 { return; }
        self.id -= 1;
    }

    pub fn id(&self) -> i32 {
        return self.id;
    }

    pub fn is_available(&self) -> bool {
        return is_gamepad_available(self);
    }

    pub fn name(&self) -> Option<&'static str> {
        return get_gamepad_name(self);
    }

    pub fn is_button_pressed(&self, button: GamepadButton) -> bool {
        return is_gamepad_button_pressed(self, button);
    }

    pub fn is_button_down(&self, button: GamepadButton) -> bool {
        return is_gamepad_button_down(self, button);
    }

    pub fn is_button_released(&self, button: GamepadButton) -> bool {
        return is_gamepad_button_released(self, button);
    }

    pub fn is_button_up(&self, button: GamepadButton) -> bool {
        return is_gamepad_button_up(self, button);
    }

    pub fn get_axis_count(&self) -> i32 {
        return get_gamepad_axis_count(self);
    }

    pub fn get_axis_movement(&self, axis: GamepadAxis) -> f32 {
        return get_gamepad_axis_movement(self, axis);
    }

    pub fn set_vibration(&self, left_motor: f32, right_motor: f32, duration: f32) {
        return set_gamepad_vibration(self, left_motor, right_motor, duration);
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GamepadButton {
    Unknown = 0,         // Unknown button, just for error checking
    LeftFaceUp,        // Gamepad left DPAD up button
    LeftFaceRight,     // Gamepad left DPAD right button
    LeftFaceDown,      // Gamepad left DPAD down button
    LeftFaceLeft,      // Gamepad left DPAD left button
    RightFaceUp,       // Gamepad right button up (i.e. PS3: Triangle, Xbox: Y)
    RightFaceRight,    // Gamepad right button right (i.e. PS3: Circle, Xbox: B)
    RightFaceDown,     // Gamepad right button down (i.e. PS3: Cross, Xbox: A)
    RightFaceLeft,     // Gamepad right button left (i.e. PS3: Square, Xbox: X)
    LeftTrigger1,      // Gamepad top/back trigger left (first), it could be a trailing button
    LeftTrigger2,      // Gamepad top/back trigger left (second), it could be a trailing button
    RightTrigger1,     // Gamepad top/back trigger right (first), it could be a trailing button
    RightTrigger2,     // Gamepad top/back trigger right (second), it could be a trailing button
    MiddleLeft,         // Gamepad center buttons, left one (i.e. PS3: Select)
    Middle,              // Gamepad center buttons, middle one (i.e. PS3: PS, Xbox: XBOX)
    MiddleRight,        // Gamepad center buttons, right one (i.e. PS3: Start)
    LeftThumb,          // Gamepad joystick pressed button left
    RightThumb          // Gamepad joystick pressed button right
}

impl TryFrom<i32> for GamepadButton {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        return match value {
            0 => Ok(GamepadButton::Unknown),
            1 => Ok(GamepadButton::LeftFaceUp),
            2 => Ok(GamepadButton::LeftFaceRight),
            3 => Ok(GamepadButton::LeftFaceDown),
            4 => Ok(GamepadButton::LeftFaceLeft),
            5 => Ok(GamepadButton::RightFaceUp),
            6 => Ok(GamepadButton::RightFaceRight),
            7 => Ok(GamepadButton::RightFaceDown),
            8 => Ok(GamepadButton::RightFaceLeft),
            9 => Ok(GamepadButton::LeftTrigger1),
            10 => Ok(GamepadButton::LeftTrigger2),
            11 => Ok(GamepadButton::RightTrigger1),
            12 => Ok(GamepadButton::RightTrigger2),
            13 => Ok(GamepadButton::MiddleLeft),
            14 => Ok(GamepadButton::Middle),
            15 => Ok(GamepadButton::MiddleRight),
            16 => Ok(GamepadButton::LeftThumb),
            17 => Ok(GamepadButton::RightThumb),
            _ => Err(()),
        };
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GamepadAxis {
    AxisLeftX = 0,     // Gamepad left stick X axis
    AxisLeftY = 1,     // Gamepad left stick Y axis
    AxisRightX = 2,     // Gamepad right stick X axis
    AxisRightY = 3,     // Gamepad right stick Y axis
    AxisLeftTrigger = 4,     // Gamepad back trigger left, pressure level: [1..-1]
    AxisRightTrigger = 5      // Gamepad back trigger right, pressure level: [1..-1]
}

impl TryFrom<i32> for GamepadAxis {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        return match value {
            0 => Ok(GamepadAxis::AxisLeftX),
            1 => Ok(GamepadAxis::AxisLeftY),
            2 => Ok(GamepadAxis::AxisRightX),
            3 => Ok(GamepadAxis::AxisRightY),
            4 => Ok(GamepadAxis::AxisLeftTrigger),
            5 => Ok(GamepadAxis::AxisRightTrigger),
            _ => Err(()),
        }
    }
}

pub struct ConfigFlag {
    flag: u32,
}

impl ConfigFlag {
    pub fn value(&self) -> u32 {
        return self.flag;
    }
}

impl BitOr for ConfigFlag {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        let result = self.flag | rhs.flag;
        return Self { flag: result };
    }
}

impl BitOrAssign for ConfigFlag {
    fn bitor_assign(&mut self, rhs: Self) {
        self.flag |= rhs.flag;
    }
}

impl BitAnd for ConfigFlag {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        let result = self.flag & rhs.flag;
        return Self { flag: result };
    }
}

impl BitAndAssign for ConfigFlag {
    fn bitand_assign(&mut self, rhs: Self) {
        self.flag &= rhs.flag;
    }
}

impl BitXor for ConfigFlag {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        let result = self.flag ^ rhs.flag;
        return Self { flag: result };
    }
}

impl BitXorAssign for ConfigFlag {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.flag ^= rhs.flag;
    }
}

impl Not for ConfigFlag {
    type Output = Self;

    fn not(self) -> Self::Output {
        return Self { flag: !self.flag };
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigFlags {
    VsyncHint = 0x00000040,              // Set to try enabling V-Sync on GPU
    FullscreenMode = 0x00000002,         // Set to run program in fullscreen
    WindowResizable = 0x00000004,        // Set to allow resizable window
    WindowUndecorated = 0x00000008,      // Set to disable window decoration (frame and buttons)
    WindowHidden = 0x00000080,           // Set to hide window
    WindowMinimized = 0x00000200,        // Set to minimize window (iconify)
    WindowMaximized = 0x00000400,        // Set to maximize window (expanded to monitor)
    WindowUnfocused = 0x00000800,        // Set to window non focused
    WindowTopmost = 0x00001000,          // Set to window always on top
    WindowAlwaysRun = 0x00000100,        // Set to allow windows running while minimized
    WindowTransparent = 0x00000010,      // Set to allow transparent framebuffer
    WindowHighDPI = 0x00002000,          // Set to support HighDPI
    WindowMousePassthrough = 0x00004000, // Set to support mouse passthrough, only supported when WindowUndecorated
    BorderlessWindowedMode = 0x00008000, // Set to run program in borderless windowed mode
    Msaa4xHint = 0x00000020,             // Set to try enabling MSAA 4X
    InterlacedHint = 0x00010000,         // Set to try enabling interlaced video format (for V3D)
}

impl BitOr for ConfigFlags {
    type Output = ConfigFlag;

    fn bitor(self, rhs: Self) -> Self::Output {
        let result = self as u32 | rhs as u32;
        return Self::Output { flag: result };
    }
}

impl BitAnd for ConfigFlags {
    type Output = ConfigFlag;

    fn bitand(self, rhs: Self) -> Self::Output {
        let result = self as u32 & rhs as u32;
        return Self::Output { flag: result };
    }
}

impl BitXor for ConfigFlags {
    type Output = ConfigFlag;

    fn bitxor(self, rhs: Self) -> Self::Output {
        let result = self as u32 ^ rhs as u32;
        return Self::Output { flag: result };
    }
}

impl Not for ConfigFlags {
    type Output = ConfigFlag;

    fn not(self) -> Self::Output {
        return Self::Output { flag: !(self as u32) };
    }
}

impl Into<ConfigFlag> for ConfigFlags {
    fn into(self) -> ConfigFlag {
        return ConfigFlag { flag: self as u32 };
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GlyphInfo {
    pub value: i32,      // Character value (Unicode)
    pub offset_x: i32,   // Character offset X when drawing
    pub offset_y: i32,   // Character offset Y when drawing
    pub advance_x: i32,  // Character advance position X
    pub image: Image,      // Character image data
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Font {
    pub base_size: i32,        // Base size (default chars height)
    pub glyph_count: i32,      // Number of glyph characters
    pub glyph_padding: i32,    // Padding around the glyph characters
    pub texture: Texture2D,      // Texture atlas containing the glyphs
    pub recs: *mut Rectangle,    // Rectangles in texture for the glyphs
    pub glyphs: *mut GlyphInfo,  // Glyphs info data
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RenderTexture {
    pub id: u32,             // OpenGL framebuffer object id
    pub texture: Texture,    // Color buffer attachment texture
    pub depth: Texture,      // Depth buffer attachment texture
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

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NPatchInfo {
    pub source: Rectangle,       // Texture source rectangle
    pub left: i32,             // Left border offset
    pub top: i32,              // Top border offset
    pub right: i32,            // Right border offset
    pub bottom: i32,           // Bottom border offset
    pub layout: i32,           // Layout of the n-patch (NPatchLayout)
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PixelFormat {
    UncompressedGrayscale = 1,
    UncompressedGrayAlpha,
    UncompressedR5g6b5,
    UncompressedR8g8b8,
    UncompressedR5g5b5a1,
    UncompressedR4g4b4a4,
    UncompressedR8g8b8a8,
    UncompressedR32,
    UncompressedR32g32b32,
    UncompressedR32g32b32a32,
    UncompressedR16,
    UncompressedR16g16b16,
    UncompressedR16g16b16a16,
    CompressedDxt1Rgb,
    CompressedDxt1Rgba,
    CompressedDxt3Rgba,
    CompressedDxt5Rgba,
    CompressedEtc1Rgb,
    CompressedEtc2Rgb,
    CompressedEtc2EacRgba,
    CompressedPvrtRgb,
    CompressedPvrtRgba,
    CompressedAstc4x4Rgba,
    CompressedAstc8x8Rgba
}

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
    MirrorClamp
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CubemapLayout {
    AutoDetect = 0,
    LineVertical,
    LineHorizontal,
    CrossThreeByFour,
    CrossFourByThree
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NPatchLayout {
    NinePatch = 0,          // Npatch layout: 3x3 tiles
    ThreePatchVertical,    // Npatch layout: 1x3 tiles
    ThreePatchHorizontal   // Npatch layout: 3x1 tiles
}