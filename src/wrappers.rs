use crate::types::Color;
use crate::{bindings, ConfigFlag, Gamepad, GamepadAxis, GamepadButton, KeyboardKey, MouseButton, Rectangle, Texture2D, Vector2};
use std::ffi::{CStr, CString};

pub const LIGHTGRAY: Color = Color {
    r: 200,
    g: 200,
    b: 200,
    a: 255,
}; // Light Gray
pub const GRAY: Color = Color {
    r: 130,
    g: 130,
    b: 130,
    a: 255,
}; // Gray
pub const DARKGRAY: Color = Color {
    r: 80,
    g: 80,
    b: 80,
    a: 255,
}; // Dark Gray
pub const YELLOW: Color = Color {
    r: 253,
    g: 249,
    b: 0,
    a: 255,
}; // Yellow
pub const GOLD: Color = Color {
    r: 255,
    g: 203,
    b: 0,
    a: 255,
}; // Gold
pub const ORANGE: Color = Color {
    r: 255,
    g: 161,
    b: 0,
    a: 255,
}; // Orange
pub const PINK: Color = Color {
    r: 255,
    g: 109,
    b: 194,
    a: 255,
}; // Pink
pub const RED: Color = Color {
    r: 230,
    g: 41,
    b: 55,
    a: 255,
}; // Red
pub const MAROON: Color = Color {
    r: 190,
    g: 33,
    b: 55,
    a: 255,
}; // Maroon
pub const GREEN: Color = Color {
    r: 0,
    g: 228,
    b: 48,
    a: 255,
}; // Green
pub const LIME: Color = Color {
    r: 0,
    g: 158,
    b: 47,
    a: 255,
}; // Lime
pub const DARKGREEN: Color = Color {
    r: 0,
    g: 117,
    b: 44,
    a: 255,
}; // Dark Green
pub const SKYBLUE: Color = Color {
    r: 102,
    g: 191,
    b: 255,
    a: 255,
}; // Sky Blue
pub const BLUE: Color = Color {
    r: 0,
    g: 121,
    b: 241,
    a: 255,
}; // Blue
pub const DARKBLUE: Color = Color {
    r: 0,
    g: 82,
    b: 172,
    a: 255,
}; // Dark Blue
pub const PURPLE: Color = Color {
    r: 200,
    g: 122,
    b: 255,
    a: 255,
}; // Purple
pub const VIOLET: Color = Color {
    r: 135,
    g: 60,
    b: 190,
    a: 255,
}; // Violet
pub const DARKPURPLE: Color = Color {
    r: 112,
    g: 31,
    b: 126,
    a: 255,
}; // Dark Purple
pub const BEIGE: Color = Color {
    r: 211,
    g: 176,
    b: 131,
    a: 255,
}; // Beige
pub const BROWN: Color = Color {
    r: 127,
    g: 106,
    b: 79,
    a: 255,
}; // Brown
pub const DARKBROWN: Color = Color {
    r: 76,
    g: 63,
    b: 47,
    a: 255,
}; // Dark Brown

pub const WHITE: Color = Color {
    r: 255,
    g: 255,
    b: 255,
    a: 255,
}; // White
pub const BLACK: Color = Color {
    r: 0,
    g: 0,
    b: 0,
    a: 255,
}; // Black
pub const BLANK: Color = Color { r: 0, g: 0, b: 0, a: 0 }; // Blank (Transparent)
pub const MAGENTA: Color = Color {
    r: 255,
    g: 0,
    b: 255,
    a: 255,
}; // Magenta
pub const RAYWHITE: Color = Color {
    r: 245,
    g: 245,
    b: 245,
    a: 255,
}; // My own White (raylib logo)

// Window related wrappers

pub fn init_window(width: i32, height: i32, title: &str) {
    let c_text = CString::new(title).unwrap();
    unsafe {
        bindings::InitWindow(width, height, c_text.as_ptr());
    };
}

pub fn window_should_close() -> bool {
    return unsafe { bindings::WindowShouldClose() };
}

pub fn close_window() {
    unsafe { bindings::CloseWindow() };
}

// Timing related wrappers

pub fn set_target_fps(fps: i32) {
    unsafe { bindings::SetTargetFPS(fps) };
}

pub fn get_fps() -> i32 {
    return unsafe { bindings::GetFPS() };
}

pub fn get_frame_time() -> f32 {
    return unsafe { bindings::GetFrameTime() };
}

// Drawing related wrappers

pub fn begin_drawing() {
    unsafe { bindings::BeginDrawing() };
}

pub fn end_drawing() {
    unsafe { bindings::EndDrawing() };
}

pub fn clear_background(color: Color) {
    unsafe { bindings::ClearBackground(color) };
}

pub fn draw_text(text: &str, pos_x: i32, pos_y: i32, font_size: i32, color: Color) {
    let c_text = CString::new(text).expect("Failed to create CString");
    unsafe { bindings::DrawText(c_text.as_ptr(), pos_x, pos_y, font_size, color) };
}

// Shape drawing related wrappers

pub fn draw_circle(center_x: i32, center_y: i32, radius: f32, color: Color) {
    unsafe {bindings::DrawCircle(center_x, center_y, radius, color) };
}

pub fn draw_circle_v(center: Vector2, radius: f32, color: Color) {
    unsafe { bindings::DrawCircleV(center, radius, color) };
}

pub fn draw_rectangle(pos_x: i32, pos_y: i32, width: i32, height: i32, color: Color) {
    unsafe { bindings::DrawRectangle(pos_x, pos_y, width, height, color) };
}

pub fn draw_rectangle_rounded(rect: Rectangle, roundness: f32, segments: i32, color: Color) {
    unsafe {bindings::DrawRectangleRounded(rect, roundness, segments, color) };
}

pub fn draw_triangle(v1: Vector2, v2: Vector2, v3: Vector2, color: Color) {
    unsafe {bindings::DrawTriangle(v1, v2, v3, color) };
}

// Input related wrappers: keyboard

pub fn is_key_pressed(key: KeyboardKey) -> bool {
    return unsafe { bindings::IsKeyPressed(key as i32) };
}

pub fn is_key_pressed_repeat(key: KeyboardKey) -> bool {
    return unsafe { bindings::IsKeyPressedRepeat(key as i32) };
}

pub fn is_key_down(key: KeyboardKey) -> bool {
    return unsafe { bindings::IsKeyDown(key as i32) };
}

// Input related wrappers: gamepad

pub fn is_gamepad_available(gamepad: &Gamepad) -> bool {
    return unsafe { bindings::IsGamepadAvailable(gamepad.id) };
}

pub fn get_gamepad_name(gamepad: &Gamepad) -> Option<&'static str> {
    return unsafe {
        let ptr = bindings::GetGamepadName(gamepad.id);
        if ptr.is_null() {
            return None;
        }

        // Convert C string to &str safely
        let c_str = CStr::from_ptr(ptr);
        c_str.to_str().ok()
    };
}

pub fn is_gamepad_button_pressed(gamepad: &Gamepad, button: GamepadButton) -> bool {
    unsafe { bindings::IsGamepadButtonPressed(gamepad.id, button as i32) }
}

pub fn is_gamepad_button_down(gamepad: &Gamepad, button: GamepadButton) -> bool {
    unsafe { bindings::IsGamepadButtonDown(gamepad.id, button as i32) }
}

pub fn is_gamepad_button_released(gamepad: &Gamepad, button: GamepadButton) -> bool {
    unsafe { bindings::IsGamepadButtonReleased(gamepad.id, button as i32) }
}

pub fn is_gamepad_button_up(gamepad: &Gamepad, button: GamepadButton) -> bool {
    unsafe { bindings::IsGamepadButtonUp(gamepad.id, button as i32) }
}

pub fn get_gamepad_button_pressed() -> Option<GamepadButton> {
    unsafe {
        let button = bindings::GetGamepadButtonPressed();
        if button >= 0 {
            button.try_into().ok()
        } else {
            None
        }
    }
}

pub fn get_gamepad_axis_count(gamepad: &Gamepad) -> i32 {
    unsafe { bindings::GetGamepadAxisCount(gamepad.id) }
}

pub fn get_gamepad_axis_movement(gamepad: &Gamepad, axis: GamepadAxis) -> f32 {
    unsafe { bindings::GetGamepadAxisMovement(gamepad.id, axis as i32) }
}

pub fn set_gamepad_mappings(mappings: &str) -> i32 {
    let c_str = CString::new(mappings).unwrap();
    unsafe { bindings::SetGamepadMappings(c_str.as_ptr()) }
}

pub fn set_gamepad_vibration(gamepad: &Gamepad, left_motor: f32, right_motor: f32, duration: f32) {
    unsafe { bindings::SetGamepadVibration(gamepad.id, left_motor, right_motor, duration) }
}


// Input related wrappers: mouse

pub fn is_mouse_button_pressed(button: MouseButton) -> bool {
    return unsafe { bindings::IsMouseButtonPressed(button as i32) };
}

pub fn is_mouse_button_down(button: MouseButton) -> bool {
    return unsafe { bindings::IsMouseButtonDown(button as i32) };
}

pub fn is_mouse_button_released(button: MouseButton) -> bool {
    return unsafe { bindings::IsMouseButtonReleased(button as i32) };
}

pub fn is_mouse_button_up(button: MouseButton) -> bool {
    return unsafe { bindings::IsMouseButtonUp(button as i32) };
}

pub fn get_mouse_x() -> i32 {
    return unsafe { bindings::GetMouseX() };
}

pub fn get_mouse_y() -> i32 {
    return unsafe { bindings::GetMouseY() };
}

pub fn get_mouse_position() -> Vector2 {
    return unsafe { bindings::GetMousePosition() };
}

pub fn get_mouse_wheel_move() -> f32 {
    return unsafe { bindings::GetMouseWheelMove() };
}

// Cursor related wrappers

pub fn show_cursor() {
    unsafe { bindings::ShowCursor() };
}

pub fn hide_cursor() {
    unsafe { bindings::HideCursor() };
}

pub fn is_cursor_hidden() -> bool {
    return unsafe { bindings::IsCursorHidden() };
}

// Misc. wrappers

pub fn set_config_flags(flag: ConfigFlag) {
    return unsafe { bindings::SetConfigFlags(flag.value()) };
}

// Texture loading wrappers

pub fn load_texture(filename: &str) -> Texture2D {
    let c_text = CString::new(filename).expect("Failed to create CString");
    return unsafe { bindings::LoadTexture(c_text.as_ptr()) };
}

// Texture drawing wrappers

pub fn draw_texture(texture: Texture2D, pos_x: i32, pos_y: i32, tint: Color) {
    unsafe { bindings::DrawTexture(texture, pos_x, pos_y, tint)};
}
