use crate::types::Color;
use crate::{bindings, ConfigFlag, CubemapLayout, Font, Gamepad, GamepadAxis, GamepadButton, Image, KeyboardKey, MouseButton, NPatchInfo, PixelFormat, Rectangle, RenderTexture2D, Texture2D, TextureCubemap, TextureFilter, TextureWrap, Vector2};
use std::ffi::{CStr, CString};
use std::os::raw::c_void;
use std::slice;

pub const LIGHTGRAY: Color = Color {
    r: 200,
    g: 200,
    b: 200,
    a: 255,
};
pub const GRAY: Color = Color {
    r: 130,
    g: 130,
    b: 130,
    a: 255,
};
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

// ---------------------------------------------------------------------------------
// Image loading wrappers
// ---------------------------------------------------------------------------------

pub fn load_image(filename: &str) -> Image {
    let c_filename = CString::new(filename).expect("Failed to create CString");
    return unsafe { bindings::LoadImage(c_filename.as_ptr()) };
}

pub fn load_image_raw(filename: &str, width: i32, height: i32, format: PixelFormat, header_size: i32) -> Image {
    let c_filename = CString::new(filename).expect("Failed to create CString");
    return unsafe { bindings::LoadImageRaw(c_filename.as_ptr(), width, height, format as i32, header_size) };
}

pub fn load_image_anim(filename: &str, frames: &mut i32) -> Image {
    let c_filename = CString::new(filename).expect("Failed to create CString");
    return unsafe { bindings::LoadImageAnim(c_filename.as_ptr(), frames as *mut i32) };
}

pub fn load_image_anim_from_memory(file_type: &str, file_data: &[u8], frames: &mut i32) -> Image {
    let c_file_type = CString::new(file_type).expect("Failed to create CString");
    return unsafe { bindings::LoadImageAnimFromMemory(c_file_type.as_ptr(), file_data.as_ptr(), file_data.len() as i32, frames as *mut i32) };
}

pub fn load_image_from_memory(file_type: &str, file_data: &[u8]) -> Image {
    let c_file_type = CString::new(file_type).expect("Failed to create CString");
    return unsafe { bindings::LoadImageFromMemory(c_file_type.as_ptr(), file_data.as_ptr(), file_data.len() as i32) };
}

pub fn load_image_from_texture(texture: Texture2D) -> Image {
    return unsafe { bindings::LoadImageFromTexture(texture) };
}

pub fn load_image_from_screen() -> Image {
    return unsafe { bindings::LoadImageFromScreen() };
}

pub fn is_image_valid(image: Image) -> bool {
    return unsafe { bindings::IsImageValid(image) };
}

pub fn unload_image(image: Image) {
    unsafe { bindings::UnloadImage(image) };
}

pub fn export_image(image: Image, filename: &str) -> bool {
    let c_filename = CString::new(filename).expect("Failed to create CString");
    return unsafe { bindings::ExportImage(image, c_filename.as_ptr()) };
}

pub fn export_image_to_memory(image: Image, file_type: &str) -> Vec<u8> {
    let c_file_type = CString::new(file_type).expect("Failed to create CString");
    let mut file_size: i32 = 0;

    unsafe {
        let data_ptr = bindings::ExportImageToMemory(image, c_file_type.as_ptr(), &mut file_size as *mut i32);
        let data_slice = slice::from_raw_parts(data_ptr, file_size as usize);
        let result_vec = data_slice.to_vec();
        bindings::MemFree(data_ptr as *mut c_void);
        return result_vec;
    }
}

pub fn export_image_as_code(image: Image, filename: &str) -> bool {
    let c_filename = CString::new(filename).expect("Failed to create CString");
    return unsafe { bindings::ExportImageAsCode(image, c_filename.as_ptr()) };
}

// ---------------------------------------------------------------------------------
// Image generation wrappers
// ---------------------------------------------------------------------------------

pub fn gen_image_color(width: i32, height: i32, color: Color) -> Image {
    return unsafe { bindings::GenImageColor(width, height, color) };
}

pub fn gen_image_gradient_linear(width: i32, height: i32, direction: i32, start: Color, end: Color) -> Image {
    return unsafe { bindings::GenImageGradientLinear(width, height, direction, start, end) };
}

pub fn gen_image_gradient_radial(width: i32, height: i32, density: f32, inner: Color, outer: Color) -> Image {
    return unsafe { bindings::GenImageGradientRadial(width, height, density, inner, outer) };
}

pub fn gen_image_gradient_square(width: i32, height: i32, density: f32, inner: Color, outer: Color) -> Image {
    return unsafe { bindings::GenImageGradientSquare(width, height, density, inner, outer) };
}

pub fn gen_image_checked(width: i32, height: i32, checks_x: i32, checks_y: i32, col1: Color, col2: Color) -> Image {
    return unsafe { bindings::GenImageChecked(width, height, checks_x, checks_y, col1, col2) };
}

pub fn gen_image_white_noise(width: i32, height: i32, factor: f32) -> Image {
    return unsafe { bindings::GenImageWhiteNoise(width, height, factor) };
}

pub fn gen_image_perlin_noise(width: i32, height: i32, offset_x: i32, offset_y: i32, scale: f32) -> Image {
    return unsafe { bindings::GenImagePerlinNoise(width, height, offset_x, offset_y, scale) };
}

pub fn gen_image_cellular(width: i32, height: i32, tile_size: i32) -> Image {
    return unsafe { bindings::GenImageCellular(width, height, tile_size) };
}

pub fn gen_image_text(width: i32, height: i32, text: &str) -> Image {
    let c_text = CString::new(text).expect("Failed to create CString");
    return unsafe { bindings::GenImageText(width, height, c_text.as_ptr()) };
}

// ---------------------------------------------------------------------------------
// Image manipulation wrappers
// ---------------------------------------------------------------------------------

pub fn image_copy(image: Image) -> Image {
    return unsafe { bindings::ImageCopy(image) };
}

pub fn image_from_image(image: Image, rec: Rectangle) -> Image {
    return unsafe { bindings::ImageFromImage(image, rec) };
}

pub fn image_from_channel(image: Image, selected_channel: i32) -> Image {
    return unsafe { bindings::ImageFromChannel(image, selected_channel) };
}

pub fn image_text(text: &str, font_size: i32, color: Color) -> Image {
    let c_text = CString::new(text).expect("Failed to create CString");
    return unsafe { bindings::ImageText(c_text.as_ptr(), font_size, color) };
}

pub fn image_text_ex(font: Font, text: &str, font_size: f32, spacing: f32, tint: Color) -> Image {
    let c_text = CString::new(text).expect("Failed to create CString");
    return unsafe { bindings::ImageTextEx(font, c_text.as_ptr(), font_size, spacing, tint) };
}

pub fn image_format(image: &mut Image, new_format: PixelFormat) {
    unsafe { bindings::ImageFormat(image as *mut Image, new_format as i32) };
}

pub fn image_to_pot(image: &mut Image, fill: Color) {
    unsafe { bindings::ImageToPOT(image as *mut Image, fill) };
}

pub fn image_crop(image: &mut Image, crop: Rectangle) {
    unsafe { bindings::ImageCrop(image as *mut Image, crop) };
}

pub fn image_alpha_crop(image: &mut Image, threshold: f32) {
    unsafe { bindings::ImageAlphaCrop(image as *mut Image, threshold) };
}

pub fn image_alpha_clear(image: &mut Image, color: Color, threshold: f32) {
    unsafe { bindings::ImageAlphaClear(image as *mut Image, color, threshold) };
}

pub fn image_alpha_mask(image: &mut Image, alpha_mask: Image) {
    unsafe { bindings::ImageAlphaMask(image as *mut Image, alpha_mask) };
}

pub fn image_alpha_premultiply(image: &mut Image) {
    unsafe { bindings::ImageAlphaPremultiply(image as *mut Image) };
}

pub fn image_blur_gaussian(image: &mut Image, blur_size: i32) {
    unsafe { bindings::ImageBlurGaussian(image as *mut Image, blur_size) };
}

pub fn image_kernel_convolution(image: &mut Image, kernel: &[f32], kernel_size: i32) {
    unsafe { bindings::ImageKernelConvolution(image as *mut Image, kernel.as_ptr(), kernel_size) };
}

pub fn image_resize(image: &mut Image, new_width: i32, new_height: i32) {
    unsafe { bindings::ImageResize(image as *mut Image, new_width, new_height) };
}

pub fn image_resize_nn(image: &mut Image, new_width: i32, new_height: i32) {
    unsafe { bindings::ImageResizeNN(image as *mut Image, new_width, new_height) };
}

pub fn image_resize_canvas(image: &mut Image, new_width: i32, new_height: i32, offset_x: i32, offset_y: i32, fill: Color) {
    unsafe { bindings::ImageResizeCanvas(image as *mut Image, new_width, new_height, offset_x, offset_y, fill) };
}

pub fn image_mipmaps(image: &mut Image) {
    unsafe { bindings::ImageMipmaps(image as *mut Image) };
}

pub fn image_dither(image: &mut Image, r_bpp: i32, g_bpp: i32, b_bpp: i32, a_bpp: i32) {
    unsafe { bindings::ImageDither(image as *mut Image, r_bpp, g_bpp, b_bpp, a_bpp) };
}

pub fn image_flip_vertical(image: &mut Image) {
    unsafe { bindings::ImageFlipVertical(image as *mut Image) };
}

pub fn image_flip_horizontal(image: &mut Image) {
    unsafe { bindings::ImageFlipHorizontal(image as *mut Image) };
}

pub fn image_rotate(image: &mut Image, degrees: i32) {
    unsafe { bindings::ImageRotate(image as *mut Image, degrees) };
}

pub fn image_rotate_cw(image: &mut Image) {
    unsafe { bindings::ImageRotateCW(image as *mut Image) };
}

pub fn image_rotate_ccw(image: &mut Image) {
    unsafe { bindings::ImageRotateCCW(image as *mut Image) };
}

pub fn image_color_tint(image: &mut Image, color: Color) {
    unsafe { bindings::ImageColorTint(image as *mut Image, color) };
}

pub fn image_color_invert(image: &mut Image) {
    unsafe { bindings::ImageColorInvert(image as *mut Image) };
}

pub fn image_color_grayscale(image: &mut Image) {
    unsafe { bindings::ImageColorGrayscale(image as *mut Image) };
}

pub fn image_color_contrast(image: &mut Image, contrast: f32) {
    unsafe { bindings::ImageColorContrast(image as *mut Image, contrast) };
}

pub fn image_color_brightness(image: &mut Image, brightness: i32) {
    unsafe { bindings::ImageColorBrightness(image as *mut Image, brightness) };
}

pub fn image_color_replace(image: &mut Image, color: Color, replace: Color) {
    unsafe { bindings::ImageColorReplace(image as *mut Image, color, replace) };
}

pub fn load_image_colors(image: Image) -> &'static mut [Color] {
    unsafe {
        let colors_ptr = bindings::LoadImageColors(image);
        let color_count = (image.width * image.height) as usize;
        return slice::from_raw_parts_mut(colors_ptr, color_count);
    }
}

pub fn load_image_palette(image: Image, max_palette_size: i32) -> &'static mut [Color] {
    let color_count: *mut i32 = std::ptr::null_mut();
    unsafe {
        let colors_ptr = bindings::LoadImagePalette(image, max_palette_size, color_count as *mut i32);
        return slice::from_raw_parts_mut(colors_ptr, *color_count as usize);
    }
}

pub fn unload_image_colors(colors: *mut Color) {
    unsafe { bindings::UnloadImageColors(colors) };
}

pub fn unload_image_palette(colors: *mut Color) {
    unsafe { bindings::UnloadImagePalette(colors) };
}

pub fn get_image_alpha_border(image: Image, threshold: f32) -> Rectangle {
    return unsafe { bindings::GetImageAlphaBorder(image, threshold) };
}

pub fn get_image_color(image: Image, x: i32, y: i32) -> Color {
    return unsafe { bindings::GetImageColor(image, x, y) };
}

// ---------------------------------------------------------------------------------
// Image drawing wrappers
// ---------------------------------------------------------------------------------

pub fn image_clear_background(dst: &mut Image, color: Color) {
    unsafe { bindings::ImageClearBackground(dst as *mut Image, color) };
}

pub fn image_draw_pixel(dst: &mut Image, pos_x: i32, pos_y: i32, color: Color) {
    unsafe { bindings::ImageDrawPixel(dst as *mut Image, pos_x, pos_y, color) };
}

pub fn image_draw_pixel_v(dst: &mut Image, position: Vector2, color: Color) {
    unsafe { bindings::ImageDrawPixelV(dst as *mut Image, position, color) };
}

pub fn image_draw_line(dst: &mut Image, start_pos_x: i32, start_pos_y: i32, end_pos_x: i32, end_pos_y: i32, color: Color) {
    unsafe { bindings::ImageDrawLine(dst as *mut Image, start_pos_x, start_pos_y, end_pos_x, end_pos_y, color) };
}

pub fn image_draw_line_v(dst: &mut Image, start: Vector2, end: Vector2, color: Color) {
    unsafe { bindings::ImageDrawLineV(dst as *mut Image, start, end, color) };
}

pub fn image_draw_line_ex(dst: &mut Image, start: Vector2, end: Vector2, thick: i32, color: Color) {
    unsafe { bindings::ImageDrawLineEx(dst as *mut Image, start, end, thick, color) };
}

pub fn image_draw_circle(dst: &mut Image, center_x: i32, center_y: i32, radius: i32, color: Color) {
    unsafe { bindings::ImageDrawCircle(dst as *mut Image, center_x, center_y, radius, color) };
}

pub fn image_draw_circle_v(dst: &mut Image, center: Vector2, radius: i32, color: Color) {
    unsafe { bindings::ImageDrawCircleV(dst as *mut Image, center, radius, color) };
}

pub fn image_draw_circle_lines(dst: &mut Image, center_x: i32, center_y: i32, radius: i32, color: Color) {
    unsafe { bindings::ImageDrawCircleLines(dst as *mut Image, center_x, center_y, radius, color) };
}

pub fn image_draw_circle_lines_v(dst: &mut Image, center: Vector2, radius: i32, color: Color) {
    unsafe { bindings::ImageDrawCircleLinesV(dst as *mut Image, center, radius, color) };
}

pub fn image_draw_rectangle(dst: &mut Image, pos_x: i32, pos_y: i32, width: i32, height: i32, color: Color) {
    unsafe { bindings::ImageDrawRectangle(dst as *mut Image, pos_x, pos_y, width, height, color) };
}

pub fn image_draw_rectangle_v(dst: &mut Image, position: Vector2, size: Vector2, color: Color) {
    unsafe { bindings::ImageDrawRectangleV(dst as *mut Image, position, size, color) };
}

pub fn image_draw_rectangle_rec(dst: &mut Image, rec: Rectangle, color: Color) {
    unsafe { bindings::ImageDrawRectangleRec(dst as *mut Image, rec, color) };
}

pub fn image_draw_rectangle_lines(dst: &mut Image, rec: Rectangle, thick: i32, color: Color) {
    unsafe { bindings::ImageDrawRectangleLines(dst as *mut Image, rec, thick, color) };
}

pub fn image_draw_triangle(dst: &mut Image, v1: Vector2, v2: Vector2, v3: Vector2, color: Color) {
    unsafe { bindings::ImageDrawTriangle(dst as *mut Image, v1, v2, v3, color) };
}

pub fn image_draw_triangle_ex(dst: &mut Image, v1: Vector2, v2: Vector2, v3: Vector2, c1: Color, c2: Color, c3: Color) {
    unsafe { bindings::ImageDrawTriangleEx(dst as *mut Image, v1, v2, v3, c1, c2, c3) };
}

pub fn image_draw_triangle_lines(dst: &mut Image, v1: Vector2, v2: Vector2, v3: Vector2, color: Color) {
    unsafe { bindings::ImageDrawTriangleLines(dst as *mut Image, v1, v2, v3, color) };
}

pub fn image_draw_triangle_fan(dst: &mut Image, points: &[Vector2], color: Color) {
    unsafe { bindings::ImageDrawTriangleFan(dst as *mut Image, points.as_ptr(), points.len() as i32, color) };
}

pub fn image_draw_triangle_strip(dst: &mut Image, points: &[Vector2], color: Color) {
    unsafe { bindings::ImageDrawTriangleStrip(dst as *mut Image, points.as_ptr(), points.len() as i32, color) };
}

pub fn image_draw(dst: &mut Image, src: Image, src_rec: Rectangle, dst_rec: Rectangle, tint: Color) {
    unsafe { bindings::ImageDraw(dst as *mut Image, src, src_rec, dst_rec, tint) };
}

pub fn image_draw_text(dst: &mut Image, text: &str, pos_x: i32, pos_y: i32, font_size: i32, color: Color) {
    let c_text = CString::new(text).expect("Failed to create CString");
    unsafe { bindings::ImageDrawText(dst as *mut Image, c_text.as_ptr(), pos_x, pos_y, font_size, color) };
}

pub fn image_draw_text_ex(dst: &mut Image, font: Font, text: &str, position: Vector2, font_size: f32, spacing: f32, tint: Color) {
    let c_text = CString::new(text).expect("Failed to create CString");
    unsafe { bindings::ImageDrawTextEx(dst as *mut Image, font, c_text.as_ptr(), position, font_size, spacing, tint) };
}

// ---------------------------------------------------------------------------------
// Texture loading wrappers
// ---------------------------------------------------------------------------------

pub fn load_texture(filename: &str) -> Texture2D { 
    let c_text = CString::new(filename).expect("Failed to create CString");
    return unsafe { bindings::LoadTexture(c_text.as_ptr()) };
}

pub fn load_texture_from_image(image: Image) -> Texture2D {
    return unsafe { bindings::LoadTextureFromImage(image) };
}

pub fn load_texture_cubemap(image: Image, layout: CubemapLayout) -> TextureCubemap {
    return unsafe { bindings::LoadTextureCubemap(image, layout as i32) };
}

pub fn load_render_texture(width: i32, height: i32) -> RenderTexture2D {
    return unsafe { bindings::LoadRenderTexture(width, height) };
}

pub fn is_texture_valid(texture: Texture2D) -> bool {
    return unsafe { bindings::IsTextureValid(texture) };
}

pub fn unload_texture(texture: Texture2D) {
    unsafe { bindings::UnloadTexture(texture) };
}

pub fn is_render_texture_valid(target: RenderTexture2D) -> bool {
    return unsafe { bindings::IsRenderTextureValid(target) };
}

pub fn unload_render_texture(target: RenderTexture2D) {
    unsafe { bindings::UnloadRenderTexture(target) };
}

pub fn update_texture(texture: Texture2D, pixels: &[Color]) {
    unsafe { bindings::UpdateTexture(texture, pixels.as_ptr() as *const c_void) };
}

pub fn update_texture_rec(texture: Texture2D, rec: Rectangle, pixels: &[Color]) {
    unsafe { bindings::UpdateTextureRec(texture, rec, pixels.as_ptr() as *const c_void) };
}

// ---------------------------------------------------------------------------------
// Texture configuration wrappers
// ---------------------------------------------------------------------------------

pub fn gen_texture_mipmaps(texture: &mut Texture2D) {
    unsafe { bindings::GenTextureMipmaps(texture as *mut Texture2D) };
}

pub fn set_texture_filter(texture: Texture2D, filter: TextureFilter) {
    unsafe { bindings::SetTextureFilter(texture, filter as i32) };
}

pub fn set_texture_wrap(texture: Texture2D, wrap: TextureWrap) {
    unsafe { bindings::SetTextureWrap(texture, wrap as i32) };
}

// ---------------------------------------------------------------------------------
// Texture drawing wrappers
// ---------------------------------------------------------------------------------

pub fn draw_texture(texture: Texture2D, pos_x: i32, pos_y: i32, tint: Color) {
    unsafe { bindings::DrawTexture(texture, pos_x, pos_y, tint)};
}

pub fn draw_texture_v(texture: Texture2D, position: Vector2, tint: Color) {
    unsafe { bindings::DrawTextureV(texture, position, tint) };
}

pub fn draw_texture_ex(texture: Texture2D, position: Vector2, rotation: f32, scale: f32, tint: Color) {
    unsafe { bindings::DrawTextureEx(texture, position, rotation, scale, tint) };
}

pub fn draw_texture_rec(texture: Texture2D, source: Rectangle, position: Vector2, tint: Color) {
    unsafe { bindings::DrawTextureRec(texture, source, position, tint) };
}

pub fn draw_texture_pro(texture: Texture2D, source: Rectangle, dest: Rectangle, origin: Vector2, rotation: f32, tint: Color) {
    unsafe { bindings::DrawTexturePro(texture, source, dest, origin, rotation, tint) };
}

pub fn draw_texture_n_patch(texture: Texture2D, n_patch_info: NPatchInfo, dest: Rectangle, origin: Vector2, rotation: f32, tint: Color) {
    unsafe { bindings::DrawTextureNPatch(texture, n_patch_info, dest, origin, rotation, tint) };
}
