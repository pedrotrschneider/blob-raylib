use crate::bindings::{GetFPS, GetFrameTime, GetTime, SetTargetFPS};
use crate::{
    BlendMode, Camera2D, Camera3D, Color, ConfigFlag, CubemapLayout, Font, Gamepad, GamepadAxis, GamepadButton,
    GestureFlag, Image, KeyboardKey, MouseButton, MouseCursor, NPatchInfo, PixelFormat, Rectangle, RenderTexture2D,
    Shader, Texture2D, TextureCubemap, TextureFilter, TextureWrap, Vector2, VrDeviceInfo, VrStereoConfig, bindings,
};
use std::ffi::{CStr, CString};
use std::os::raw::{c_float, c_int, c_void};
use std::slice;

// Window related wrappers
/// Initialize window and OpenGL context
pub fn init_window(width: i32, height: i32, title: &str) {
    let c_text = CString::new(title).unwrap();
    unsafe {
        bindings::InitWindow(width, height, c_text.as_ptr());
    };
}

/// Close window and unload OpenGL context
pub fn window_should_close() -> bool {
    return unsafe { bindings::WindowShouldClose() };
}

/// Check if application should close (KEY_ESCAPE pressed or windows close icon clicked)
pub fn close_window() {
    unsafe { bindings::CloseWindow() };
}

/// Check if window has been initialized successfully
pub fn is_window_ready() -> bool {
    unsafe { bindings::IsWindowReady() }
}

/// Check if window is currently fullscreen
pub fn is_window_fullscreen() -> bool {
    unsafe { bindings::IsWindowFullscreen() }
}

/// Check if window is currently hidden
pub fn is_window_hidden() -> bool {
    unsafe { bindings::IsWindowHidden() }
}

/// Check if window is currently minimized
pub fn is_window_minimized() -> bool {
    unsafe { bindings::IsWindowMinimized() }
}

/// Check if window is currently maximized
pub fn is_window_maximized() -> bool {
    unsafe { bindings::IsWindowMaximized() }
}

/// Check if window is currently focused
pub fn is_window_focused() -> bool {
    unsafe { bindings::IsWindowFocused() }
}

/// Check if window has been resized last frame
pub fn is_window_resized() -> bool {
    unsafe { bindings::IsWindowResized() }
}

/// Check if one specific window flag is enabled
pub fn is_window_state(flag: ConfigFlag) -> bool {
    unsafe { bindings::IsWindowState(flag.value()) }
}

/// Set window configuration state using flags
pub fn set_window_state(flags: ConfigFlag) {
    unsafe { bindings::SetWindowState(flags.value()) }
}

/// Clear window configuration state flags
pub fn clear_window_state(flags: ConfigFlag) {
    unsafe { bindings::ClearWindowState(flags.value()) }
}

/// Toggle window state: fullscreen/windowed
pub fn toggle_fullscreen() {
    unsafe { bindings::ToggleFullscreen() }
}

/// Toggle window state: borderless windowed
pub fn toggle_borderless_windowed() {
    unsafe { bindings::ToggleBorderlessWindowed() }
}

/// Set window state: maximized, if resizable
pub fn maximize_window() {
    unsafe { bindings::MaximizeWindow() }
}

/// Set window state: minimized, if resizable
pub fn minimize_window() {
    unsafe { bindings::MinimizeWindow() }
}

/// Restore window from being minimized/maximized
pub fn restore_window() {
    unsafe { bindings::RestoreWindow() }
}

/// Set icon for window (single image, RGBA 32bit)
pub fn set_window_icon(image: Image) {
    unsafe { bindings::SetWindowIcon(image) }
}

/// Set icon for window (multiple images, RGBA 32bit)
pub fn set_window_icons(images: &mut [Image]) {
    unsafe { bindings::SetWindowIcons(images.as_mut_ptr(), images.len() as c_int) }
}

/// Set title for window
pub fn set_window_title(title: &str) {
    let c_title = CString::new(title).unwrap();
    unsafe { bindings::SetWindowTitle(c_title.as_ptr()) }
}

/// Set window position on screen
pub fn set_window_position(x: i32, y: i32) {
    unsafe { bindings::SetWindowPosition(x as c_int, y as c_int) }
}

/// Set monitor for the current window
pub fn set_window_monitor(monitor: i32) {
    unsafe { bindings::SetWindowMonitor(monitor as c_int) }
}

/// Set window minimum dimensions (for FLAG_WINDOW_RESIZABLE)
pub fn set_window_min_size(width: i32, height: i32) {
    unsafe { bindings::SetWindowMinSize(width as c_int, height as c_int) }
}

/// Set window maximum dimensions (for FLAG_WINDOW_RESIZABLE)
pub fn set_window_max_size(width: i32, height: i32) {
    unsafe { bindings::SetWindowMaxSize(width as c_int, height as c_int) }
}

/// Set window dimensions
pub fn set_window_size(width: i32, height: i32) {
    unsafe { bindings::SetWindowSize(width as c_int, height as c_int) }
}

/// Set window opacity [0.0f..1.0f]
pub fn set_window_opacity(opacity: f32) {
    unsafe { bindings::SetWindowOpacity(opacity as c_float) }
}

/// Set window focused
pub fn set_window_focused() {
    unsafe { bindings::SetWindowFocused() }
}

/// Get native window handle
pub fn get_window_handle() -> *mut c_void {
    unsafe { bindings::GetWindowHandle() }
}

/// Get current screen width
pub fn get_screen_width() -> i32 {
    unsafe { bindings::GetScreenWidth() }
}

/// Get current screen height
pub fn get_screen_height() -> i32 {
    unsafe { bindings::GetScreenHeight() }
}

/// Get current render width (it considers HiDPI)
pub fn get_render_width() -> i32 {
    unsafe { bindings::GetRenderWidth() }
}

/// Get current render height (it considers HiDPI)
pub fn get_render_height() -> i32 {
    unsafe { bindings::GetRenderHeight() }
}

/// Get number of connected monitors
pub fn get_monitor_count() -> i32 {
    unsafe { bindings::GetMonitorCount() }
}

/// Get current monitor where window is placed
pub fn get_current_monitor() -> i32 {
    unsafe { bindings::GetCurrentMonitor() }
}

/// Get specified monitor position
pub fn get_monitor_position(monitor: i32) -> Vector2 {
    unsafe { bindings::GetMonitorPosition(monitor as c_int) }
}

/// Get specified monitor width (current video mode used by monitor)
pub fn get_monitor_width(monitor: i32) -> i32 {
    unsafe { bindings::GetMonitorWidth(monitor as c_int) }
}

/// Get specified monitor height (current video mode used by monitor)
pub fn get_monitor_height(monitor: i32) -> i32 {
    unsafe { bindings::GetMonitorHeight(monitor as c_int) }
}

/// Get specified monitor physical width in millimetres
pub fn get_monitor_physical_width(monitor: i32) -> i32 {
    unsafe { bindings::GetMonitorPhysicalWidth(monitor as c_int) }
}

/// Get specified monitor physical height in millimetres
pub fn get_monitor_physical_height(monitor: i32) -> i32 {
    unsafe { bindings::GetMonitorPhysicalHeight(monitor as c_int) }
}

/// Get specified monitor refresh rate
pub fn get_monitor_refresh_rate(monitor: i32) -> i32 {
    unsafe { bindings::GetMonitorRefreshRate(monitor as c_int) }
}

/// Get window position XY on monitor
pub fn get_window_position() -> Vector2 {
    unsafe { bindings::GetWindowPosition() }
}

/// Get window scale DPI factor
pub fn get_window_scale_dpi() -> Vector2 {
    unsafe { bindings::GetWindowScaleDPI() }
}

/// Get the human-readable, UTF-8 encoded name of the specified monitor
pub fn get_monitor_name(monitor: i32) -> &'static str {
    unsafe {
        let c_str = bindings::GetMonitorName(monitor as c_int);
        if c_str.is_null() {
            ""
        } else {
            CStr::from_ptr(c_str).to_str().unwrap_or("")
        }
    }
}

/// Set clipboard text content
pub fn set_clipboard_text(text: &str) {
    let c_text = CString::new(text).unwrap();
    unsafe { bindings::SetClipboardText(c_text.as_ptr()) }
}

/// Get clipboard text content
/// NOTE: This wrapper allocates a new String.
pub fn get_clipboard_text() -> String {
    unsafe {
        let c_str = bindings::GetClipboardText();
        if c_str.is_null() {
            String::new()
        } else {
            CStr::from_ptr(c_str).to_string_lossy().into_owned()
        }
    }
}

/// Get clipboard image content
pub fn get_clipboard_image() -> Image {
    unsafe { bindings::GetClipboardImage() }
}

/// Enable waiting for events on EndDrawing(), no automatic event polling
pub fn enable_event_waiting() {
    unsafe { bindings::EnableEventWaiting() }
}

/// Disable waiting for events on EndDrawing(), automatic events polling
pub fn disable_event_waiting() {
    unsafe { bindings::DisableEventWaiting() }
}

// Cursor related wrappers
/// Shows cursor
pub fn show_cursor() {
    unsafe { bindings::ShowCursor() };
}

/// Hides cursor
pub fn hide_cursor() {
    unsafe { bindings::HideCursor() };
}

/// Check if cursor is not visible
pub fn is_cursor_hidden() -> bool {
    return unsafe { bindings::IsCursorHidden() };
}

/// Enables cursor (unlock cursor)
pub fn enable_cursor() {
    unsafe { bindings::EnableCursor() }
}

/// Disables cursor (lock cursor)
pub fn disable_cursor() {
    unsafe { bindings::DisableCursor() }
}

/// Check if cursor is on the screen
pub fn is_cursor_on_screen() -> bool {
    unsafe { bindings::IsCursorOnScreen() }
}

// Drawing related wrappers
/// Set background color (framebuffer clear color)
pub fn clear_background(color: Color) {
    unsafe { bindings::ClearBackground(color) };
}

/// Setup canvas (framebuffer) to start drawing
pub fn begin_drawing() {
    unsafe { bindings::BeginDrawing() };
}

/// End canvas drawing and swap buffers (double buffering)
pub fn end_drawing() {
    unsafe { bindings::EndDrawing() };
}

/// Begin 2D mode with custom camera (2D)
pub fn begin_mode_2d(camera: Camera2D) {
    unsafe { bindings::BeginMode2D(camera) }
}

/// Ends 2D mode with custom camera
pub fn end_mode_2d() {
    unsafe { bindings::EndMode2D() }
}

/// Begin 3D mode with custom camera (3D)
pub fn begin_mode_3d(camera: Camera3D) {
    unsafe { bindings::BeginMode3D(camera) }
}

/// Ends 3D mode and returns to default 2D orthographic mode
pub fn end_mode_3d() {
    unsafe { bindings::EndMode3D() }
}

/// Begin drawing to render texture
pub fn begin_texture_mode(target: RenderTexture2D) {
    unsafe { bindings::BeginTextureMode(target) }
}

/// Ends drawing to render texture
pub fn end_texture_mode() {
    unsafe { bindings::EndTextureMode() }
}

/// Begin custom shader drawing
pub fn begin_shader_mode(shader: Shader) {
    unsafe { bindings::BeginShaderMode(shader) }
}

/// End custom shader drawing (use default shader)
pub fn end_shader_mode() {
    unsafe { bindings::EndShaderMode() }
}

/// Begin blending mode
pub fn begin_blend_mode(mode: BlendMode) {
    unsafe { bindings::BeginBlendMode(mode as c_int) }
}

/// End blending mode (reset to default: alpha blending)
pub fn end_blend_mode() {
    unsafe { bindings::EndBlendMode() }
}

/// Begin scissor mode (define screen area for following drawing)
pub fn begin_scissor_mode(x: i32, y: i32, width: i32, height: i32) {
    unsafe { bindings::BeginScissorMode(x as c_int, y as c_int, width as c_int, height as c_int) }
}

/// End scissor mode
pub fn end_scissor_mode() {
    unsafe { bindings::EndScissorMode() }
}

/// Begin stereo rendering (requires VR simulator)
pub fn begin_vr_stereo_mode(config: VrStereoConfig) {
    unsafe { bindings::BeginVrStereoMode(config) }
}

/// End stereo rendering (requires VR simulator)
pub fn end_vr_stereo_mode() {
    unsafe { bindings::EndVrStereoMode() }
}

// VR stereo config functions for VR simulator
/// Load VR stereo config for VR simulator device parameters
pub fn load_vr_stereo_config(device: VrDeviceInfo) -> VrStereoConfig {
    unsafe { bindings::LoadVrStereoConfig(device) }
}

/// Unload VR stereo config
pub fn unload_vr_stereo_config(config: VrStereoConfig) {
    unsafe { bindings::UnloadVrStereoConfig(config) }
}

// Timing related wrappers
/// Set target FPS (maximum)
pub fn set_target_fps(fps: i32) {
    unsafe { SetTargetFPS(fps) };
}

/// Get time in seconds for last frame drawn (delta time)
pub fn get_frame_time() -> f32 {
    return unsafe { GetFrameTime() };
}

/// Get elapsed time in seconds since InitWindow()
pub fn get_time() -> f64 {
    return unsafe { GetTime() };
}

/// Get current FPS
pub fn get_fps() -> i32 {
    return unsafe { GetFPS() };
}

pub fn draw_text(text: &str, pos_x: i32, pos_y: i32, font_size: i32, color: Color) {
    let c_text = CString::new(text).expect("Failed to create CString");
    unsafe { bindings::DrawText(c_text.as_ptr(), pos_x, pos_y, font_size, color) };
}

// Shape drawing related wrappers

pub fn draw_circle(center_x: i32, center_y: i32, radius: f32, color: Color) {
    unsafe { bindings::DrawCircle(center_x, center_y, radius, color) };
}

pub fn draw_circle_v(center: Vector2, radius: f32, color: Color) {
    unsafe { bindings::DrawCircleV(center, radius, color) };
}

pub fn draw_rectangle(pos_x: i32, pos_y: i32, width: i32, height: i32, color: Color) {
    unsafe { bindings::DrawRectangle(pos_x, pos_y, width, height, color) };
}

pub fn draw_rectangle_rounded(rect: Rectangle, roundness: f32, segments: i32, color: Color) {
    unsafe { bindings::DrawRectangleRounded(rect, roundness, segments, color) };
}

pub fn draw_triangle(v1: Vector2, v2: Vector2, v3: Vector2, color: Color) {
    unsafe { bindings::DrawTriangle(v1, v2, v3, color) };
}

// ---------------------------------------------------------------------------------
// Input-related functions: keyboard
// ---------------------------------------------------------------------------------

/// Check if a key has been pressed once
pub fn is_key_pressed(key: KeyboardKey) -> bool {
    unsafe { bindings::IsKeyPressed(key as c_int) }
}

/// Check if a key has been pressed again (useful for continuous key presses)
pub fn is_key_pressed_repeat(key: KeyboardKey) -> bool {
    unsafe { bindings::IsKeyPressedRepeat(key as c_int) }
}

/// Check if a key is being pressed
pub fn is_key_down(key: KeyboardKey) -> bool {
    unsafe { bindings::IsKeyDown(key as c_int) }
}

/// Check if a key has been released once
pub fn is_key_released(key: KeyboardKey) -> bool {
    unsafe { bindings::IsKeyReleased(key as c_int) }
}

/// Check if a key is NOT being pressed
pub fn is_key_up(key: KeyboardKey) -> bool {
    unsafe { bindings::IsKeyUp(key as c_int) }
}

/// Get key pressed (keycode), call it multiple times for keys queued
/// Returns `None` when the queue is empty
pub fn get_key_pressed() -> Option<KeyboardKey> {
    unsafe { KeyboardKey::try_from(bindings::GetKeyPressed()).ok() }
}

/// Get char pressed (unicode), call it multiple times for chars queued
/// Returns `None` when the queue is empty
pub fn get_char_pressed() -> Option<char> {
    unsafe { std::char::from_u32(bindings::GetCharPressed() as u32) }
}

/// Get name of a QWERTY key on the current keyboard layout
pub fn get_key_name(key: KeyboardKey) -> &'static str {
    unsafe {
        let c_str = bindings::GetKeyName(key as c_int);
        CStr::from_ptr(c_str).to_str().unwrap_or("")
    }
}

/// Set a custom key to exit program (default is ESC)
/// Pass `None` to restore the default (ESC).
pub fn set_exit_key(key: Option<KeyboardKey>) {
    let key_to_set = key.unwrap_or(KeyboardKey::Escape);
    unsafe { bindings::SetExitKey(key_to_set as c_int) }
}

// ---------------------------------------------------------------------------------
// Input-related functions: gamepad
// ---------------------------------------------------------------------------------

/// Check if a gamepad is available
pub fn is_gamepad_available(gamepad: &Gamepad) -> bool {
    return unsafe { bindings::IsGamepadAvailable(gamepad.id) };
}

/// Get gamepad internal name id
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

/// Check if a gamepad button has been pressed once
pub fn is_gamepad_button_pressed(gamepad: &Gamepad, button: GamepadButton) -> bool {
    unsafe { bindings::IsGamepadButtonPressed(gamepad.id, button as i32) }
}

/// Check if a gamepad button is being pressed
pub fn is_gamepad_button_down(gamepad: &Gamepad, button: GamepadButton) -> bool {
    unsafe { bindings::IsGamepadButtonDown(gamepad.id, button as i32) }
}

/// Check if a gamepad button has been released once
pub fn is_gamepad_button_released(gamepad: &Gamepad, button: GamepadButton) -> bool {
    unsafe { bindings::IsGamepadButtonReleased(gamepad.id, button as i32) }
}

/// Check if a gamepad button is NOT being pressed
pub fn is_gamepad_button_up(gamepad: &Gamepad, button: GamepadButton) -> bool {
    unsafe { bindings::IsGamepadButtonUp(gamepad.id, button as i32) }
}

/// Get the last gamepad button pressed
pub fn get_gamepad_button_pressed() -> Option<GamepadButton> {
    unsafe {
        let button = bindings::GetGamepadButtonPressed();
        if button >= 0 { button.try_into().ok() } else { None }
    }
}

/// Get axis count for a gamepad
pub fn get_gamepad_axis_count(gamepad: &Gamepad) -> i32 {
    unsafe { bindings::GetGamepadAxisCount(gamepad.id) }
}

/// Get movement value for a gamepad axis
pub fn get_gamepad_axis_movement(gamepad: &Gamepad, axis: GamepadAxis) -> f32 {
    unsafe { bindings::GetGamepadAxisMovement(gamepad.id, axis as i32) }
}

/// Set internal gamepad mappings (SDL_GameControllerDB)
pub fn set_gamepad_mappings(mappings: &str) -> i32 {
    let c_str = CString::new(mappings).unwrap();
    unsafe { bindings::SetGamepadMappings(c_str.as_ptr()) }
}

/// Set gamepad vibration for both motors (duration in seconds)
pub fn set_gamepad_vibration(gamepad: &Gamepad, left_motor: f32, right_motor: f32, seconds: f32) {
    unsafe { bindings::SetGamepadVibration(gamepad.id, left_motor, right_motor, seconds) }
}

// ---------------------------------------------------------------------------------
// Input-related functions: mouse
// ---------------------------------------------------------------------------------

/// Check if a mouse button has been pressed once
pub fn is_mouse_button_pressed(button: MouseButton) -> bool {
    unsafe { bindings::IsMouseButtonPressed(button as c_int) }
}

/// Check if a mouse button is being pressed
pub fn is_mouse_button_down(button: MouseButton) -> bool {
    unsafe { bindings::IsMouseButtonDown(button as c_int) }
}

/// Check if a mouse button has been released once
pub fn is_mouse_button_released(button: MouseButton) -> bool {
    unsafe { bindings::IsMouseButtonReleased(button as c_int) }
}

/// Check if a mouse button is NOT being pressed
pub fn is_mouse_button_up(button: MouseButton) -> bool {
    unsafe { bindings::IsMouseButtonUp(button as c_int) }
}

/// Get mouse position X
pub fn get_mouse_x() -> i32 {
    unsafe { bindings::GetMouseX() }
}

/// Get mouse position Y
pub fn get_mouse_y() -> i32 {
    unsafe { bindings::GetMouseY() }
}

/// Get mouse position XY
pub fn get_mouse_position() -> Vector2 {
    unsafe { bindings::GetMousePosition() }
}

/// Get mouse delta between frames
pub fn get_mouse_delta() -> Vector2 {
    unsafe { bindings::GetMouseDelta() }
}

/// Set mouse position XY
pub fn set_mouse_position(x: i32, y: i32) {
    unsafe { bindings::SetMousePosition(x as c_int, y as c_int) }
}

/// Set mouse offset
pub fn set_mouse_offset(x: i32, y: i32) {
    unsafe { bindings::SetMouseOffset(x as c_int, y as c_int) }
}

/// Set mouse scaling
pub fn set_mouse_scale(x: f32, y: f32) {
    unsafe { bindings::SetMouseScale(x as c_float, y as c_float) }
}

/// Get mouse wheel movement for X or Y, whichever is larger
pub fn get_mouse_wheel_move() -> f32 {
    unsafe { bindings::GetMouseWheelMove() }
}

/// Get mouse wheel movement for both X and Y
pub fn get_mouse_wheel_move_v() -> Vector2 {
    unsafe { bindings::GetMouseWheelMoveV() }
}

/// Set mouse cursor
pub fn set_mouse_cursor(cursor: MouseCursor) {
    unsafe { bindings::SetMouseCursor(cursor as c_int) }
}

// ---------------------------------------------------------------------------------
// Input-related functions: touch
// ---------------------------------------------------------------------------------

/// Get touch position X for touch point 0
pub fn get_touch_x() -> i32 {
    unsafe { bindings::GetTouchX() }
}

/// Get touch position Y for touch point 0
pub fn get_touch_y() -> i32 {
    unsafe { bindings::GetTouchY() }
}

/// Get touch position XY for a touch point index
pub fn get_touch_position(index: i32) -> Vector2 {
    unsafe { bindings::GetTouchPosition(index as c_int) }
}

/// Get touch point identifier for given index
pub fn get_touch_point_id(index: i32) -> i32 {
    unsafe { bindings::GetTouchPointId(index as c_int) }
}

/// Get number of touch points
pub fn get_touch_point_count() -> i32 {
    unsafe { bindings::GetTouchPointCount() }
}

// ---------------------------------------------------------------------------------
// Gestures and Touch Handling Functions
// ---------------------------------------------------------------------------------

/// Enable a set of gestures using flags
pub fn set_gestures_enabled(flags: GestureFlag) {
    unsafe { bindings::SetGesturesEnabled(flags.bits()) }
}

/// Check if a gesture have been detected
pub fn is_gesture_detected(gesture: GestureFlag) -> bool {
    unsafe { bindings::IsGestureDetected(gesture.bits()) }
}

/// Get latest detected gesture
pub fn get_gesture_detected() -> Option<GestureFlag> {
    unsafe { GestureFlag::try_from(bindings::GetGestureDetected()).ok() }
}

/// Get gesture hold time in seconds
pub fn get_gesture_hold_duration() -> f32 {
    unsafe { bindings::GetGestureHoldDuration() }
}

/// Get gesture drag vector
pub fn get_gesture_drag_vector() -> Vector2 {
    unsafe { bindings::GetGestureDragVector() }
}

/// Get gesture drag angle
pub fn get_gesture_drag_angle() -> f32 {
    unsafe { bindings::GetGestureDragAngle() }
}

/// Get gesture pinch delta
pub fn get_gesture_pinch_vector() -> Vector2 {
    unsafe { bindings::GetGesturePinchVector() }
}

/// Get gesture pinch angle
pub fn get_gesture_pinch_angle() -> f32 {
    unsafe { bindings::GetGesturePinchAngle() }
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
    return unsafe {
        bindings::LoadImageAnimFromMemory(
            c_file_type.as_ptr(),
            file_data.as_ptr(),
            file_data.len() as i32,
            frames as *mut i32,
        )
    };
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

pub fn image_resize_canvas(
    image: &mut Image,
    new_width: i32,
    new_height: i32,
    offset_x: i32,
    offset_y: i32,
    fill: Color,
) {
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
        let colors_ptr = bindings::LoadImagePalette(image, max_palette_size, color_count);
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

pub fn image_draw_line(
    dst: &mut Image,
    start_pos_x: i32,
    start_pos_y: i32,
    end_pos_x: i32,
    end_pos_y: i32,
    color: Color,
) {
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

pub fn image_draw_text_ex(
    dst: &mut Image,
    font: Font,
    text: &str,
    position: Vector2,
    font_size: f32,
    spacing: f32,
    tint: Color,
) {
    let c_text = CString::new(text).expect("Failed to create CString");
    unsafe {
        bindings::ImageDrawTextEx(
            dst as *mut Image,
            font,
            c_text.as_ptr(),
            position,
            font_size,
            spacing,
            tint,
        )
    };
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
    unsafe { bindings::DrawTexture(texture, pos_x, pos_y, tint) };
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

pub fn draw_texture_pro(
    texture: Texture2D,
    source: Rectangle,
    dest: Rectangle,
    origin: Vector2,
    rotation: f32,
    tint: Color,
) {
    unsafe { bindings::DrawTexturePro(texture, source, dest, origin, rotation, tint) };
}

pub fn draw_texture_n_patch(
    texture: Texture2D,
    n_patch_info: NPatchInfo,
    dest: Rectangle,
    origin: Vector2,
    rotation: f32,
    tint: Color,
) {
    unsafe { bindings::DrawTextureNPatch(texture, n_patch_info, dest, origin, rotation, tint) };
}
