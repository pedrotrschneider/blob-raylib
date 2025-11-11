use crate::bindings::{AudioCallback, GetFPS, GetFrameTime, GetTime, LoadFileDataCallback, LoadFileTextCallback, SaveFileDataCallback, SaveFileTextCallback, SetTargetFPS, TraceLogCallback};
use crate::{bindings, AudioStream, AutomationEvent, AutomationEventList, BlendMode, BoundingBox, Camera, Camera2D, Camera3D, CameraMode, Color, ConfigFlag, CubemapLayout, FilePathList, Font, FontType, Gamepad, GamepadAxis, GamepadButton, GestureFlag, GlyphInfo, Image, KeyboardKey, Material, MaterialMapIndex, Matrix, Mesh, Model, ModelAnimation, Monitor, MouseButton, MouseCursor, Music, NPatchInfo, PixelFormat, Ray, RayCollision, Rectangle, RenderTexture2D, Shader, ShaderLocation, ShaderUniformDataType, Sound, Texture2D, TextureCubemap, TextureFilter, TextureWrap, TraceLogLevel, Vector2, Vector3, Vector4, VrDeviceInfo, VrStereoConfig, Wave};
use std::ffi::{c_char, c_double, c_uchar, c_uint, CStr, CString};
use std::os::raw::{c_float, c_int, c_void};
use std::slice;

// ---------------------------------------------------------------------------------
// Window related wrappers
// ---------------------------------------------------------------------------------

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
    return unsafe { bindings::GetWindowHandle() }
}

/// Get current screen width
pub fn get_screen_width() -> i32 {
    return unsafe { bindings::GetScreenWidth() }
}

/// Get current screen height
pub fn get_screen_height() -> i32 {
    return unsafe { bindings::GetScreenHeight() }
}

/// Get current render width (it considers HiDPI)
pub fn get_render_width() -> i32 {
    return unsafe { bindings::GetRenderWidth() }
}

/// Get current render height (it considers HiDPI)
pub fn get_render_height() -> i32 {
    return unsafe { bindings::GetRenderHeight() }
}

/// Get number of connected monitors
pub fn get_monitor_count() -> i32 {
    return unsafe { bindings::GetMonitorCount() }
}

/// Get current monitor where window is placed
pub fn get_current_monitor() -> Monitor {
    return unsafe { bindings::GetCurrentMonitor().into() };
}

/// Get specified monitor position
pub fn get_monitor_position(monitor: &Monitor) -> Vector2 {
    return unsafe { bindings::GetMonitorPosition(monitor.id as c_int) }
}

/// Get specified monitor width (current video mode used by monitor)
pub fn get_monitor_width(monitor: &Monitor) -> i32 {
    return unsafe { bindings::GetMonitorWidth(monitor.id as c_int) }
}

/// Get specified monitor height (current video mode used by monitor)
pub fn get_monitor_height(monitor: &Monitor) -> i32 {
    return unsafe { bindings::GetMonitorHeight(monitor.id as c_int) }
}

/// Get specified monitor physical width in millimetres
pub fn get_monitor_physical_width(monitor: &Monitor) -> i32 {
    return unsafe { bindings::GetMonitorPhysicalWidth(monitor.id as c_int) }
}

/// Get specified monitor physical height in millimetres
pub fn get_monitor_physical_height(monitor: &Monitor) -> i32 {
    return unsafe { bindings::GetMonitorPhysicalHeight(monitor.id as c_int) }
}

/// Get specified monitor refresh rate
pub fn get_monitor_refresh_rate(monitor: &Monitor) -> i32 {
    return unsafe { bindings::GetMonitorRefreshRate(monitor.id as c_int) }
}

/// Get window position XY on monitor
pub fn get_window_position() -> Vector2 {
    return unsafe { bindings::GetWindowPosition() }
}

/// Get window scale DPI factor
pub fn get_window_scale_dpi() -> Vector2 {
    return unsafe { bindings::GetWindowScaleDPI() }
}

/// Get the human-readable, UTF-8 encoded name of the specified monitor
pub fn get_monitor_name(monitor: &Monitor) -> &'static str {
    return unsafe {
        let c_str = bindings::GetMonitorName(monitor.id as c_int);
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

// ---------------------------------------------------------------------------------
// Cursor related wrappers
// ---------------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------------
// Drawing related wrappers
// ---------------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------------
// VR stereo config functions for VR simulator
// ---------------------------------------------------------------------------------

/// Load VR stereo config for VR simulator device parameters
pub fn load_vr_stereo_config(device: VrDeviceInfo) -> VrStereoConfig {
    unsafe { bindings::LoadVrStereoConfig(device) }
}

/// Unload VR stereo config
pub fn unload_vr_stereo_config(config: VrStereoConfig) {
    unsafe { bindings::UnloadVrStereoConfig(config) }
}

// ---------------------------------------------------------------------------------
// Shader management wrappers
// ---------------------------------------------------------------------------------

/// Load shader from files and bind default locations
pub fn load_shader(vs_filename: &str, fs_filename: &str) -> Shader {
    let c_vs = CString::new(vs_filename).unwrap();
    let c_fs = CString::new(fs_filename).unwrap();
    unsafe { bindings::LoadShader(c_vs.as_ptr(), c_fs.as_ptr()) }
}

/// Load shader from code strings and bind default locations
pub fn load_shader_from_memory(vs_code: &str, fs_code: &str) -> Shader {
    let c_vs = CString::new(vs_code).unwrap();
    let c_fs = CString::new(fs_code).unwrap();
    unsafe { bindings::LoadShaderFromMemory(c_vs.as_ptr(), c_fs.as_ptr()) }
}

/// Check if a shader is valid (loaded on GPU)
pub fn is_shader_valid(shader: Shader) -> bool {
    unsafe { bindings::IsShaderValid(shader) }
}

/// Get shader uniform location
pub fn get_shader_location(shader: Shader, uniform_name: &str) -> ShaderLocation {
    let c_name = CString::new(uniform_name).unwrap();
    return unsafe { bindings::GetShaderLocation(shader, c_name.as_ptr()) }.into();
}

/// Get shader attribute location
pub fn get_shader_location_attrib(shader: Shader, attrib_name: &str) -> ShaderLocation {
    let c_name = CString::new(attrib_name).unwrap();
    return unsafe { bindings::GetShaderLocationAttrib(shader, c_name.as_ptr()) }.into();
}

/// Set shader uniform value
pub fn set_shader_value<T>(shader: Shader, location: ShaderLocation, value: &T, uniform_type: ShaderUniformDataType) {
    unsafe {
        bindings::SetShaderValue(
            shader,
            location.into(),
            value as *const T as *const c_void,
            uniform_type as c_int,
        )
    }
}

/// Set shader uniform value vector
pub fn set_shader_value_v<T>(shader: Shader, location: ShaderLocation, value: &[T], uniform_type: ShaderUniformDataType) {
    unsafe {
        bindings::SetShaderValueV(
            shader,
            location.into(),
            value.as_ptr() as *const c_void,
            uniform_type as c_int,
            value.len() as c_int,
        )
    }
}

/// Set shader uniform value (matrix 4x4)
pub fn set_shader_value_matrix(shader: Shader, location: ShaderLocation, mat: Matrix) {
    unsafe { bindings::SetShaderValueMatrix(shader, location.into(), mat) }
}

/// Set shader uniform value and bind the texture (sampler2d)
pub fn set_shader_value_texture(shader: Shader, location: ShaderLocation, texture: Texture2D) {
    unsafe { bindings::SetShaderValueTexture(shader, location.into(), texture) }
}

/// Unload shader from GPU memory (VRAM)
pub fn unload_shader(shader: Shader) {
    unsafe { bindings::UnloadShader(shader) }
}

// ---------------------------------------------------------------------------------
// Screen-space-related wrappers
// ---------------------------------------------------------------------------------

/// Get a ray trace from screen position (i.e. mouse)
pub fn get_screen_to_world_ray(position: Vector2, camera: Camera) -> Ray {
    unsafe { bindings::GetScreenToWorldRay(position, camera) }
}

/// Get a ray trace from screen position (i.e. mouse)
/// ALIAS for get_screen_to_world_ray
pub fn get_mouse_ray(position: Vector2, camera: Camera) -> Ray {
    get_screen_to_world_ray(position, camera)
}

/// Get a ray trace from screen position (i.e. mouse) in a viewport
pub fn get_screen_to_world_ray_ex(position: Vector2, camera: Camera, width: i32, height: i32) -> Ray {
    unsafe { bindings::GetScreenToWorldRayEx(position, camera, width as c_int, height as c_int) }
}

/// Get the screen space position for a 3d world space position
pub fn get_world_to_screen(position: Vector3, camera: Camera) -> Vector2 {
    unsafe { bindings::GetWorldToScreen(position, camera) }
}

/// Get size position for a 3d world space position
pub fn get_world_to_screen_ex(position: Vector3, camera: Camera, width: i32, height: i32) -> Vector2 {
    unsafe { bindings::GetWorldToScreenEx(position, camera, width as c_int, height as c_int) }
}

/// Get the screen space position for a 2d camera world space position
pub fn get_world_to_screen_2d(position: Vector2, camera: Camera2D) -> Vector2 {
    unsafe { bindings::GetWorldToScreen2D(position, camera) }
}

/// Get the world space position for a 2d camera screen space position
pub fn get_screen_to_world_2d(position: Vector2, camera: Camera2D) -> Vector2 {
    unsafe { bindings::GetScreenToWorld2D(position, camera) }
}

/// Get camera transform matrix (view matrix)
pub fn get_camera_matrix(camera: Camera) -> Matrix {
    unsafe { bindings::GetCameraMatrix(camera) }
}

/// Get camera 2d transform matrix
pub fn get_camera_matrix_2d(camera: Camera2D) -> Matrix {
    unsafe { bindings::GetCameraMatrix2D(camera) }
}

// ---------------------------------------------------------------------------------
// Timing related wrappers
// ---------------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------------
// Custom frame control wrappers
// ---------------------------------------------------------------------------------

/// Swap back buffer with front buffer (screen drawing)
pub fn swap_screen_buffer() {
    unsafe { bindings::SwapScreenBuffer() }
}

/// Register all input events
pub fn poll_input_events() {
    unsafe { bindings::PollInputEvents() }
}

/// Wait for some time (halt program execution)
pub fn wait_time(seconds: f64) {
    unsafe { bindings::WaitTime(seconds as c_double) }
}

// ---------------------------------------------------------------------------------
// Random values generation wrappers
// ---------------------------------------------------------------------------------

/// Set the seed for the random number generator
pub fn set_random_seed(seed: u32) {
    unsafe { bindings::SetRandomSeed(seed as c_uint) }
}

/// Get a random value between min and max (both included)
pub fn get_random_value(min: i32, max: i32) -> i32 {
    unsafe { bindings::GetRandomValue(min as c_int, max as c_int) }
}

/// Load random values sequence, no values repeated.
/// NOTE: This wrapper allocates a `Vec<i32>` and frees the C-pointer.
pub fn load_random_sequence(count: u32, min: i32, max: i32) -> Vec<i32> {
    unsafe {
        let ptr = bindings::LoadRandomSequence(count as c_uint, min as c_int, max as c_int);
        if ptr.is_null() {
            return Vec::new();
        }
        let slice = slice::from_raw_parts(ptr, count as usize);
        let vec = slice.to_vec();
        bindings::UnloadRandomSequence(ptr);
        vec
    }
}

// ---------------------------------------------------------------------------------
// Misc. wrappers
// ---------------------------------------------------------------------------------

/// Takes a screenshot of current screen (filename extension defines format)
pub fn take_screenshot(filename: &str) {
    let c_filename = CString::new(filename).unwrap();
    unsafe { bindings::TakeScreenshot(c_filename.as_ptr()) }
}

/// Setup init configuration flags
pub fn set_config_flags(flag: ConfigFlag) {
    return unsafe { bindings::SetConfigFlags(flag.value()) };
}

/// Open URL with default system browser (if available)
pub fn open_url(url: &str) {
    let c_url = CString::new(url).unwrap();
    unsafe { bindings::OpenURL(c_url.as_ptr()) }
}

// ---------------------------------------------------------------------------------
// Utils wrappers
// ---------------------------------------------------------------------------------

/// Show trace log messages (LOG_DEBUG, LOG_INFO, LOG_WARNING, LOG_ERROR...)
pub fn trace_log(level: TraceLogLevel, message: &str) {
    let c_message = CString::new(message).unwrap();
    unsafe {
        bindings::TraceLog(level as c_int, c_message.as_ptr());
    }
}

/// Set the current threshold (minimum) log level
pub fn set_trace_log_level(log_level: TraceLogLevel) {
    unsafe { bindings::SetTraceLogLevel(log_level as c_int) }
}

/// Internal memory allocator
/// # Safety
/// This is a direct binding to C's `MemAlloc`. Memory must be freed with `mem_free`.
pub unsafe fn mem_alloc(size: u32) -> *mut c_void {
    return unsafe { bindings::MemAlloc(size as c_uint) };
}

/// Internal memory reallocator
/// # Safety
/// This is a direct binding to C's `MemRealloc`.
pub unsafe fn mem_realloc(ptr: *mut c_void, size: u32) -> *mut c_void {
    return unsafe { bindings::MemRealloc(ptr, size as c_uint) };
}

/// Internal memory free
/// # Safety
/// This is a direct binding to C's `MemFree`.
pub unsafe fn mem_free(ptr: *mut c_void) {
    return unsafe { bindings::MemFree(ptr) };
}

// ---------------------------------------------------------------------------------
// Custom callbacks wrappers
// ---------------------------------------------------------------------------------

/// Set custom trace log
pub fn set_trace_log_callback(callback: TraceLogCallback) {
    unsafe { bindings::SetTraceLogCallback(callback) }
}

/// Set custom file binary data loader
pub fn set_load_file_data_callback(callback: LoadFileDataCallback) {
    unsafe { bindings::SetLoadFileDataCallback(callback) }
}

/// Set custom file binary data saver
pub fn set_save_file_data_callback(callback: SaveFileDataCallback) {
    unsafe { bindings::SetSaveFileDataCallback(callback) }
}

/// Set custom file text data loader
pub fn set_load_file_text_callback(callback: LoadFileTextCallback) {
    unsafe { bindings::SetLoadFileTextCallback(callback) }
}

/// Set custom file text data saver
pub fn set_save_file_text_callback(callback: SaveFileTextCallback) {
    unsafe { bindings::SetSaveFileTextCallback(callback) }
}

// ---------------------------------------------------------------------------------
// Files management wrappers
// ---------------------------------------------------------------------------------

/// Load file data as byte array (read)
/// NOTE: This wrapper allocates a `Vec<u8>` and frees the C-pointer.
pub fn load_file_data(filename: &str) -> Result<Vec<u8>, String> {
    let c_filename = CString::new(filename).unwrap();
    let mut data_size: c_int = 0;
    unsafe {
        let data_ptr = bindings::LoadFileData(c_filename.as_ptr(), &mut data_size as *mut c_int);
        if data_ptr.is_null() {
            return Err(format!("Failed to load file data for: {}", filename));
        }
        let slice = slice::from_raw_parts(data_ptr, data_size as usize);
        let vec = slice.to_vec();
        bindings::UnloadFileData(data_ptr);
        Ok(vec)
    }
}

/// Save data to file from byte array (write), returns true on success
pub fn save_file_data(filename: &str, data: &[u8]) -> bool {
    let c_filename = CString::new(filename).unwrap();
    unsafe { bindings::SaveFileData(c_filename.as_ptr(), data.as_ptr() as *mut c_void, data.len() as c_int) }
}

/// Export data to code (.h), returns true on success
pub fn export_data_as_code(data: &[u8], filename: &str) -> bool {
    let c_filename = CString::new(filename).unwrap();
    unsafe {
        bindings::ExportDataAsCode(
            data.as_ptr() as *const c_uchar,
            data.len() as c_int,
            c_filename.as_ptr(),
        )
    }
}

/// Load text data from file (read), returns a `String`
/// NOTE: This wrapper allocates a `String` and frees the C-pointer.
pub fn load_file_text(filename: &str) -> Result<String, String> {
    let c_filename = CString::new(filename).unwrap();
    unsafe {
        let c_str_ptr = bindings::LoadFileText(c_filename.as_ptr());
        if c_str_ptr.is_null() {
            return Err(format!("Failed to load text file: {}", filename));
        }
        let c_str = CStr::from_ptr(c_str_ptr);
        let rust_str = c_str.to_string_lossy().into_owned();
        bindings::UnloadFileText(c_str_ptr);
        Ok(rust_str)
    }
}

/// Save text data to file (write), string must be '\0' terminated, returns true on success
pub fn save_file_text(filename: &str, text: &str) -> bool {
    let c_filename = CString::new(filename).unwrap();
    let c_text = CString::new(text).unwrap();
    unsafe { bindings::SaveFileText(c_filename.as_ptr(), c_text.as_ptr()) }
}

// ---------------------------------------------------------------------------------
// File system wrappers
// ---------------------------------------------------------------------------------

/// Rename file (if exists)
pub fn file_rename(filename: &str, new_filename: &str) -> bool {
    let c_filename = CString::new(filename).unwrap();
    let c_new_filename = CString::new(new_filename).unwrap();
    unsafe { bindings::FileRename(c_filename.as_ptr(), c_new_filename.as_ptr()) != 0 }
}

/// Remove file (if exists)
pub fn file_remove(filename: &str) -> bool {
    let c_filename = CString::new(filename).unwrap();
    unsafe { bindings::FileRemove(c_filename.as_ptr()) != 0 }
}

/// Copy file from one path to another, dstPath created if it doesn't exist
pub fn file_copy(src_path: &str, dst_path: &str) -> bool {
    let c_src = CString::new(src_path).unwrap();
    let c_dst = CString::new(dst_path).unwrap();
    unsafe { bindings::FileCopy(c_src.as_ptr(), c_dst.as_ptr()) != 0 }
}

/// Move file from one directory to another, dstPath created if it doesn't exist
pub fn file_move(src_path: &str, dst_path: &str) -> bool {
    let c_src = CString::new(src_path).unwrap();
    let c_dst = CString::new(dst_path).unwrap();
    unsafe { bindings::FileMove(c_src.as_ptr(), c_dst.as_ptr()) != 0 }
}

/// Replace text in an existing file
pub fn file_text_replace(filename: &str, search: &str, replacement: &str) -> bool {
    let c_filename = CString::new(filename).unwrap();
    let c_search = CString::new(search).unwrap();
    let c_replacement = CString::new(replacement).unwrap();
    unsafe { bindings::FileTextReplace(c_filename.as_ptr(), c_search.as_ptr(), c_replacement.as_ptr()) != 0 }
}

/// Find text in existing file, returns line index or -1
pub fn file_text_find_index(filename: &str, search: &str) -> i32 {
    let c_filename = CString::new(filename).unwrap();
    let c_search = CString::new(search).unwrap();
    unsafe { bindings::FileTextFindIndex(c_filename.as_ptr(), c_search.as_ptr()) }
}

/// Check if file exists
pub fn file_exists(filename: &str) -> bool {
    let c_filename = CString::new(filename).unwrap();
    unsafe { bindings::FileExists(c_filename.as_ptr()) }
}

/// Check if a directory path exists
pub fn directory_exists(dir_path: &str) -> bool {
    let c_dir_path = CString::new(dir_path).unwrap();
    unsafe { bindings::DirectoryExists(c_dir_path.as_ptr()) }
}

/// Check file extension (recommended include point: .png, .wav)
pub fn is_file_extension(filename: &str, ext: &str) -> bool {
    let c_filename = CString::new(filename).unwrap();
    let c_ext = CString::new(ext).unwrap();
    unsafe { bindings::IsFileExtension(c_filename.as_ptr(), c_ext.as_ptr()) }
}

/// Get file length in bytes
pub fn get_file_length(filename: &str) -> i32 {
    let c_filename = CString::new(filename).unwrap();
    unsafe { bindings::GetFileLength(c_filename.as_ptr()) }
}

/// Get file modification time (last write time)
pub fn get_file_mod_time(filename: &str) -> i64 {
    let c_filename = CString::new(filename).unwrap();
    unsafe { bindings::GetFileModTime(c_filename.as_ptr()) as i64 }
}

/// Get pointer to extension for a filename string (includes dot: '.png')
pub fn get_file_extension(filename: &str) -> &'static str {
    let c_filename = CString::new(filename).unwrap();
    unsafe {
        let c_str = bindings::GetFileExtension(c_filename.as_ptr());
        if c_str.is_null() {
            ""
        } else {
            CStr::from_ptr(c_str).to_str().unwrap_or("")
        }
    }
}

/// Get pointer to filename for a path string
pub fn get_file_name(filepath: &str) -> &'static str {
    let c_filepath = CString::new(filepath).unwrap();
    unsafe {
        let c_str = bindings::GetFileName(c_filepath.as_ptr());
        if c_str.is_null() {
            ""
        } else {
            CStr::from_ptr(c_str).to_str().unwrap_or("")
        }
    }
}

/// Get filename string without extension (uses static string)
pub fn get_file_name_without_ext(filepath: &str) -> &'static str {
    let c_filepath = CString::new(filepath).unwrap();
    unsafe {
        let c_str = bindings::GetFileNameWithoutExt(c_filepath.as_ptr());
        if c_str.is_null() {
            ""
        } else {
            CStr::from_ptr(c_str).to_str().unwrap_or("")
        }
    }
}

/// Get full path for a given fileName with path (uses static string)
pub fn get_directory_path(filepath: &str) -> &'static str {
    let c_filepath = CString::new(filepath).unwrap();
    unsafe {
        let c_str = bindings::GetDirectoryPath(c_filepath.as_ptr());
        if c_str.is_null() {
            ""
        } else {
            CStr::from_ptr(c_str).to_str().unwrap_or("")
        }
    }
}

/// Get previous directory path for a given path (uses static string)
pub fn get_prev_directory_path(dir_path: &str) -> &'static str {
    let c_dir_path = CString::new(dir_path).unwrap();
    unsafe {
        let c_str = bindings::GetPrevDirectoryPath(c_dir_path.as_ptr());
        if c_str.is_null() {
            ""
        } else {
            CStr::from_ptr(c_str).to_str().unwrap_or("")
        }
    }
}

/// Get current working directory (uses static string)
pub fn get_working_directory() -> &'static str {
    unsafe {
        let c_str = bindings::GetWorkingDirectory();
        if c_str.is_null() {
            ""
        } else {
            CStr::from_ptr(c_str).to_str().unwrap_or("")
        }
    }
}

/// Get the directory of the running application (uses static string)
pub fn get_application_directory() -> &'static str {
    unsafe {
        let c_str = bindings::GetApplicationDirectory();
        if c_str.is_null() {
            ""
        } else {
            CStr::from_ptr(c_str).to_str().unwrap_or("")
        }
    }
}

/// Create directories (including full path requested), returns 0 on success
pub fn make_directory(dir_path: &str) -> bool {
    let c_dir_path = CString::new(dir_path).unwrap();
    unsafe { bindings::MakeDirectory(c_dir_path.as_ptr()) == 0 }
}

/// Change working directory, return true on success
pub fn change_directory(dir: &str) -> bool {
    let c_dir = CString::new(dir).unwrap();
    unsafe { bindings::ChangeDirectory(c_dir.as_ptr()) }
}

/// Check if a given path is a file or a directory
pub fn is_path_file(path: &str) -> bool {
    let c_path = CString::new(path).unwrap();
    unsafe { bindings::IsPathFile(c_path.as_ptr()) }
}

/// Check if fileName is valid for the platform/OS
pub fn is_file_name_valid(filename: &str) -> bool {
    let c_filename = CString::new(filename).unwrap();
    unsafe { bindings::IsFileNameValid(c_filename.as_ptr()) }
}

/// Load directory file paths
pub fn load_directory_files(dir_path: &str) -> FilePathList {
    let c_dir_path = CString::new(dir_path).unwrap();
    return unsafe { bindings::LoadDirectoryFiles(c_dir_path.as_ptr()) };
}

/// Load directory file paths with extension filtering and recursive directory scan
pub fn load_directory_files_ex(base_path: &str, filter: &str, scan_subdirs: bool) -> FilePathList {
    let c_base_path = CString::new(base_path).unwrap();
    let c_filter = CString::new(filter).unwrap();
    return unsafe { bindings::LoadDirectoryFilesEx(c_base_path.as_ptr(), c_filter.as_ptr(), scan_subdirs) };
}

/// Unload file paths
pub fn unload_directory_files(files: FilePathList) {
    unsafe {
        bindings::UnloadDirectoryFiles(files);
    }
}

/// Check if a file has been dropped into window
pub fn is_file_dropped() -> bool {
    unsafe { bindings::IsFileDropped() }
}

/// Load dropped file paths
pub fn load_dropped_files() -> FilePathList {
    return unsafe { bindings::LoadDroppedFiles() };
}

/// Unload dropped file paths
pub fn unload_dropped_files(files: FilePathList) {
    unsafe { bindings::UnloadDroppedFiles(files) };
}

// ---------------------------------------------------------------------------------
// Compression/Encoding wrappers
// ---------------------------------------------------------------------------------

/// Compress data (DEFLATE algorithm)
pub fn compress_data(data: &[u8]) -> Vec<u8> {
    let mut comp_data_size: c_int = 0;
    unsafe {
        let ptr = bindings::CompressData(
            data.as_ptr() as *const c_uchar,
            data.len() as c_int,
            &mut comp_data_size as *mut c_int,
        );
        if ptr.is_null() {
            return Vec::new();
        }
        let slice = slice::from_raw_parts(ptr, comp_data_size as usize);
        let vec = slice.to_vec();
        bindings::MemFree(ptr as *mut c_void);
        vec
    }
}

/// Decompress data (DEFLATE algorithm)
pub fn decompress_data(comp_data: &[u8]) -> Vec<u8> {
    let mut data_size: c_int = 0;
    unsafe {
        let ptr = bindings::DecompressData(
            comp_data.as_ptr() as *const c_uchar,
            comp_data.len() as c_int,
            &mut data_size as *mut c_int,
        );
        if ptr.is_null() {
            return Vec::new();
        }
        let slice = slice::from_raw_parts(ptr, data_size as usize);
        let vec = slice.to_vec();
        bindings::MemFree(ptr as *mut c_void);
        vec
    }
}

/// Encode data to Base64 string
pub fn encode_data_base64(data: &[u8]) -> String {
    let mut output_size: c_int = 0;
    unsafe {
        let c_str_ptr = bindings::EncodeDataBase64(
            data.as_ptr() as *const c_uchar,
            data.len() as c_int,
            &mut output_size as *mut c_int,
        );
        if c_str_ptr.is_null() {
            return String::new();
        }
        // outputSize includes the null terminator, so slice length is output_size - 1
        let slice = slice::from_raw_parts(c_str_ptr as *const u8, (output_size - 1) as usize);
        let s = String::from_utf8_lossy(slice).into_owned();
        bindings::MemFree(c_str_ptr as *mut c_void);
        s
    }
}

/// Decode Base64 string
pub fn decode_data_base64(text: &str) -> Vec<u8> {
    let c_text = CString::new(text).unwrap();
    let mut output_size: c_int = 0;
    unsafe {
        let ptr = bindings::DecodeDataBase64(c_text.as_ptr(), &mut output_size as *mut c_int);
        if ptr.is_null() {
            return Vec::new();
        }
        let slice = slice::from_raw_parts(ptr, output_size as usize);
        let vec = slice.to_vec();
        bindings::MemFree(ptr as *mut c_void);
        vec
    }
}

/// Compute CRC32 hash code
pub fn compute_crc32(data: &mut [u8]) -> u32 {
    unsafe { bindings::ComputeCRC32(data.as_mut_ptr(), data.len() as c_int) }
}

/// Compute MD5 hash code, returns static [u32; 4] (16 bytes)
pub fn compute_md5(data: &mut [u8]) -> &'static [u32; 4] {
    unsafe { &*(bindings::ComputeMD5(data.as_mut_ptr(), data.len() as c_int) as *const [u32; 4]) }
}

/// Compute SHA1 hash code, returns static [u32; 5] (20 bytes)
pub fn compute_sha1(data: &mut [u8]) -> &'static [u32; 5] {
    unsafe { &*(bindings::ComputeSHA1(data.as_mut_ptr(), data.len() as c_int) as *const [u32; 5]) }
}

/// Compute SHA256 hash code, returns static [u32; 8] (32 bytes)
pub fn compute_sha256(data: &mut [u8]) -> &'static [u32; 8] {
    unsafe { &*(bindings::ComputeSHA256(data.as_mut_ptr(), data.len() as c_int) as *const [u32; 8]) }
}

// ---------------------------------------------------------------------------------
// Automation events wrappers
// ---------------------------------------------------------------------------------

/// Load automation events list from file
pub fn load_automation_event_list(filename: &str) -> AutomationEventList {
    let c_filename = CString::new(filename).unwrap();
    return unsafe {
        bindings::LoadAutomationEventList(c_filename.as_ptr())
    };
}

pub fn unload_automation_event_list(list: AutomationEventList) {
    unsafe {bindings::UnloadAutomationEventList(list)}
}

/// Export automation events list as text file
pub fn export_automation_event_list(list: AutomationEventList, filename: &str) -> bool {
    let c_filename = CString::new(filename).unwrap();
    unsafe { bindings::ExportAutomationEventList(list, c_filename.as_ptr()) }
}

/// Set automation event list to record to
pub fn set_automation_event_list(list: *mut AutomationEventList) {
    unsafe { bindings::SetAutomationEventList(list) }
}

/// Set automation event internal base frame to start recording
pub fn set_automation_event_base_frame(frame: i32) {
    unsafe { bindings::SetAutomationEventBaseFrame(frame as c_int) }
}

/// Start recording automation events
pub fn start_automation_event_recording() {
    unsafe { bindings::StartAutomationEventRecording() }
}

/// Stop recording automation events
pub fn stop_automation_event_recording() {
    unsafe { bindings::StopAutomationEventRecording() }
}

/// Play a recorded automation event
pub fn play_automation_event(event: AutomationEvent) {
    unsafe { bindings::PlayAutomationEvent(event) }
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

/// Get char pressed (Unicode), call it multiple times for chars queued
/// Returns `None` when the queue is empty
pub fn get_char_pressed() -> Option<char> {
    unsafe { char::from_u32(bindings::GetCharPressed() as u32) }
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

// ---------------------------------------------------------------------------------
// Camera System wrappers
// ---------------------------------------------------------------------------------

/// Update camera position for selected mode
pub fn update_camera(camera: &mut Camera, mode: CameraMode) {
    unsafe { bindings::UpdateCamera(camera as *mut Camera, mode as c_int) }
}

/// Update camera movement/rotation
pub fn update_camera_pro(camera: &mut Camera, movement: Vector3, rotation: Vector3, zoom: f32) {
    unsafe {
        bindings::UpdateCameraPro(
            camera as *mut Camera,
            movement,
            rotation,
            zoom as c_float,
        )
    }
}

// ---------------------------------------------------------------------------------
// Basic Shapes Drawing wrappers
// ---------------------------------------------------------------------------------

/// Set texture and rectangle to be used on shapes drawing
pub fn set_shapes_texture(texture: Texture2D, source: Rectangle) {
    unsafe { bindings::SetShapesTexture(texture, source) }
}

/// Get texture that is used for shapes drawing
pub fn get_shapes_texture() -> Texture2D {
    unsafe { bindings::GetShapesTexture() }
}

/// Get texture source rectangle that is used for shapes drawing
pub fn get_shapes_texture_rectangle() -> Rectangle {
    unsafe { bindings::GetShapesTextureRectangle() }
}

// Basic shapes drawing functions

/// Draw a pixel using geometry
pub fn draw_pixel(pos_x: i32, pos_y: i32, color: Color) {
    unsafe { bindings::DrawPixel(pos_x as c_int, pos_y as c_int, color) }
}

/// Draw a pixel using geometry (Vector version)
pub fn draw_pixel_v(position: Vector2, color: Color) {
    unsafe { bindings::DrawPixelV(position, color) }
}

/// Draw a line
pub fn draw_line(start_pos_x: i32, start_pos_y: i32, end_pos_x: i32, end_pos_y: i32, color: Color) {
    unsafe {
        bindings::DrawLine(
            start_pos_x as c_int,
            start_pos_y as c_int,
            end_pos_x as c_int,
            end_pos_y as c_int,
            color,
        )
    }
}

/// Draw a line (using gl lines)
pub fn draw_line_v(start_pos: Vector2, end_pos: Vector2, color: Color) {
    unsafe { bindings::DrawLineV(start_pos, end_pos, color) }
}

/// Draw a line (using triangles/quads)
pub fn draw_line_ex(start_pos: Vector2, end_pos: Vector2, thick: f32, color: Color) {
    unsafe { bindings::DrawLineEx(start_pos, end_pos, thick as c_float, color) }
}

/// Draw lines sequence (using gl lines)
pub fn draw_line_strip(points: &[Vector2], color: Color) {
    unsafe { bindings::DrawLineStrip(points.as_ptr(), points.len() as c_int, color) }
}

/// Draw line segment cubic-bezier in-out interpolation
pub fn draw_line_bezier(start_pos: Vector2, end_pos: Vector2, thick: f32, color: Color) {
    unsafe { bindings::DrawLineBezier(start_pos, end_pos, thick as c_float, color) }
}

/// Draw a dashed line
pub fn draw_line_dashed(
    start_pos: Vector2,
    end_pos: Vector2,
    dash_size: i32,
    space_size: i32,
    color: Color,
) {
    unsafe {
        bindings::DrawLineDashed(
            start_pos,
            end_pos,
            dash_size as c_int,
            space_size as c_int,
            color,
        )
    }
}

/// Draw a color-filled circle
pub fn draw_circle(center_x: i32, center_y: i32, radius: f32, color: Color) {
    unsafe { bindings::DrawCircle(center_x as c_int, center_y as c_int, radius as c_float, color) }
}

/// Draw a piece of a circle
pub fn draw_circle_sector(
    center: Vector2,
    radius: f32,
    start_angle: f32,
    end_angle: f32,
    segments: i32,
    color: Color,
) {
    unsafe {
        bindings::DrawCircleSector(
            center,
            radius as c_float,
            start_angle as c_float,
            end_angle as c_float,
            segments as c_int,
            color,
        )
    }
}

/// Draw circle sector outline
pub fn draw_circle_sector_lines(
    center: Vector2,
    radius: f32,
    start_angle: f32,
    end_angle: f32,
    segments: i32,
    color: Color,
) {
    unsafe {
        bindings::DrawCircleSectorLines(
            center,
            radius as c_float,
            start_angle as c_float,
            end_angle as c_float,
            segments as c_int,
            color,
        )
    }
}

/// Draw a gradient-filled circle
pub fn draw_circle_gradient(
    center_x: i32,
    center_y: i32,
    radius: f32,
    inner: Color,
    outer: Color,
) {
    unsafe {
        bindings::DrawCircleGradient(
            center_x as c_int,
            center_y as c_int,
            radius as c_float,
            inner,
            outer,
        )
    }
}

/// Draw a color-filled circle (Vector version)
pub fn draw_circle_v(center: Vector2, radius: f32, color: Color) {
    unsafe { bindings::DrawCircleV(center, radius as c_float, color) }
}

/// Draw circle outline
pub fn draw_circle_lines(center_x: i32, center_y: i32, radius: f32, color: Color) {
    unsafe {
        bindings::DrawCircleLines(center_x as c_int, center_y as c_int, radius as c_float, color)
    }
}

/// Draw circle outline (Vector version)
pub fn draw_circle_lines_v(center: Vector2, radius: f32, color: Color) {
    unsafe { bindings::DrawCircleLinesV(center, radius as c_float, color) }
}

/// Draw ellipse
pub fn draw_ellipse(center_x: i32, center_y: i32, radius_h: f32, radius_v: f32, color: Color) {
    unsafe {
        bindings::DrawEllipse(
            center_x as c_int,
            center_y as c_int,
            radius_h as c_float,
            radius_v as c_float,
            color,
        )
    }
}

/// Draw ellipse (Vector version)
pub fn draw_ellipse_v(center: Vector2, radius_h: f32, radius_v: f32, color: Color) {
    unsafe { bindings::DrawEllipseV(center, radius_h as c_float, radius_v as c_float, color) }
}

/// Draw ellipse outline
pub fn draw_ellipse_lines(
    center_x: i32,
    center_y: i32,
    radius_h: f32,
    radius_v: f32,
    color: Color,
) {
    unsafe {
        bindings::DrawEllipseLines(
            center_x as c_int,
            center_y as c_int,
            radius_h as c_float,
            radius_v as c_float,
            color,
        )
    }
}

/// Draw ellipse outline (Vector version)
pub fn draw_ellipse_lines_v(center: Vector2, radius_h: f32, radius_v: f32, color: Color) {
    unsafe { bindings::DrawEllipseLinesV(center, radius_h as c_float, radius_v as c_float, color) }
}

/// Draw ring
pub fn draw_ring(
    center: Vector2,
    inner_radius: f32,
    outer_radius: f32,
    start_angle: f32,
    end_angle: f32,
    segments: i32,
    color: Color,
) {
    unsafe {
        bindings::DrawRing(
            center,
            inner_radius as c_float,
            outer_radius as c_float,
            start_angle as c_float,
            end_angle as c_float,
            segments as c_int,
            color,
        )
    }
}

/// Draw ring outline
pub fn draw_ring_lines(
    center: Vector2,
    inner_radius: f32,
    outer_radius: f32,
    start_angle: f32,
    end_angle: f32,
    segments: i32,
    color: Color,
) {
    unsafe {
        bindings::DrawRingLines(
            center,
            inner_radius as c_float,
            outer_radius as c_float,
            start_angle as c_float,
            end_angle as c_float,
            segments as c_int,
            color,
        )
    }
}

/// Draw a color-filled rectangle
pub fn draw_rectangle(pos_x: i32, pos_y: i32, width: i32, height: i32, color: Color) {
    unsafe {
        bindings::DrawRectangle(
            pos_x as c_int,
            pos_y as c_int,
            width as c_int,
            height as c_int,
            color,
        )
    }
}

/// Draw a color-filled rectangle (Vector version)
pub fn draw_rectangle_v(position: Vector2, size: Vector2, color: Color) {
    unsafe { bindings::DrawRectangleV(position, size, color) }
}

/// Draw a color-filled rectangle
pub fn draw_rectangle_rec(rec: Rectangle, color: Color) {
    unsafe { bindings::DrawRectangleRec(rec, color) }
}

/// Draw a color-filled rectangle with pro parameters
pub fn draw_rectangle_pro(rec: Rectangle, origin: Vector2, rotation: f32, color: Color) {
    unsafe { bindings::DrawRectanglePro(rec, origin, rotation as c_float, color) }
}

/// Draw a vertical-gradient-filled rectangle
pub fn draw_rectangle_gradient_v(
    pos_x: i32,
    pos_y: i32,
    width: i32,
    height: i32,
    top: Color,
    bottom: Color,
) {
    unsafe {
        bindings::DrawRectangleGradientV(
            pos_x as c_int,
            pos_y as c_int,
            width as c_int,
            height as c_int,
            top,
            bottom,
        )
    }
}

/// Draw a horizontal-gradient-filled rectangle
pub fn draw_rectangle_gradient_h(
    pos_x: i32,
    pos_y: i32,
    width: i32,
    height: i32,
    left: Color,
    right: Color,
) {
    unsafe {
        bindings::DrawRectangleGradientH(
            pos_x as c_int,
            pos_y as c_int,
            width as c_int,
            height as c_int,
            left,
            right,
        )
    }
}

/// Draw a gradient-filled rectangle with custom vertex colors
pub fn draw_rectangle_gradient_ex(
    rec: Rectangle,
    top_left: Color,
    bottom_left: Color,
    bottom_right: Color,
    top_right: Color,
) {
    unsafe {
        bindings::DrawRectangleGradientEx(rec, top_left, bottom_left, bottom_right, top_right)
    }
}

/// Draw rectangle outline
pub fn draw_rectangle_lines(pos_x: i32, pos_y: i32, width: i32, height: i32, color: Color) {
    unsafe {
        bindings::DrawRectangleLines(
            pos_x as c_int,
            pos_y as c_int,
            width as c_int,
            height as c_int,
            color,
        )
    }
}

/// Draw rectangle outline with extended parameters
pub fn draw_rectangle_lines_ex(rec: Rectangle, line_thick: f32, color: Color) {
    unsafe { bindings::DrawRectangleLinesEx(rec, line_thick as c_float, color) }
}

/// Draw rectangle with rounded edges
pub fn draw_rectangle_rounded(rec: Rectangle, roundness: f32, segments: i32, color: Color) {
    unsafe {
        bindings::DrawRectangleRounded(rec, roundness as c_float, segments as c_int, color)
    }
}

/// Draw rectangle lines with rounded edges
pub fn draw_rectangle_rounded_lines(
    rec: Rectangle,
    roundness: f32,
    segments: i32,
    color: Color,
) {
    unsafe {
        bindings::DrawRectangleRoundedLines(rec, roundness as c_float, segments as c_int, color)
    }
}

/// Draw rectangle with rounded edges outline
pub fn draw_rectangle_rounded_lines_ex(
    rec: Rectangle,
    roundness: f32,
    segments: i32,
    line_thick: f32,
    color: Color,
) {
    unsafe {
        bindings::DrawRectangleRoundedLinesEx(
            rec,
            roundness as c_float,
            segments as c_int,
            line_thick as c_float,
            color,
        )
    }
}

/// Draw a color-filled triangle
pub fn draw_triangle(v1: Vector2, v2: Vector2, v3: Vector2, color: Color) {
    unsafe { bindings::DrawTriangle(v1, v2, v3, color) }
}

/// Draw triangle outline
pub fn draw_triangle_lines(v1: Vector2, v2: Vector2, v3: Vector2, color: Color) {
    unsafe { bindings::DrawTriangleLines(v1, v2, v3, color) }
}

/// Draw a triangle fan defined by points
pub fn draw_triangle_fan(points: &[Vector2], color: Color) {
    unsafe { bindings::DrawTriangleFan(points.as_ptr(), points.len() as c_int, color) }
}

/// Draw a triangle strip defined by points
pub fn draw_triangle_strip(points: &[Vector2], color: Color) {
    unsafe { bindings::DrawTriangleStrip(points.as_ptr(), points.len() as c_int, color) }
}

/// Draw a regular polygon
pub fn draw_poly(center: Vector2, sides: i32, radius: f32, rotation: f32, color: Color) {
    unsafe {
        bindings::DrawPoly(
            center,
            sides as c_int,
            radius as c_float,
            rotation as c_float,
            color,
        )
    }
}

/// Draw a polygon outline of n sides
pub fn draw_poly_lines(center: Vector2, sides: i32, radius: f32, rotation: f32, color: Color) {
    unsafe {
        bindings::DrawPolyLines(
            center,
            sides as c_int,
            radius as c_float,
            rotation as c_float,
            color,
        )
    }
}

/// Draw a polygon outline of n sides with extended parameters
pub fn draw_poly_lines_ex(
    center: Vector2,
    sides: i32,
    radius: f32,
    rotation: f32,
    line_thick: f32,
    color: Color,
) {
    unsafe {
        bindings::DrawPolyLinesEx(
            center,
            sides as c_int,
            radius as c_float,
            rotation as c_float,
            line_thick as c_float,
            color,
        )
    }
}

// ---------------------------------------------------------------------------------
// Splines drawing wrappers
// ---------------------------------------------------------------------------------

/// Draw spline: Linear
pub fn draw_spline_linear(points: &[Vector2], thick: f32, color: Color) {
    unsafe {
        bindings::DrawSplineLinear(points.as_ptr(), points.len() as c_int, thick as c_float, color)
    }
}

/// Draw spline: B-Spline
pub fn draw_spline_basis(points: &[Vector2], thick: f32, color: Color) {
    unsafe {
        bindings::DrawSplineBasis(points.as_ptr(), points.len() as c_int, thick as c_float, color)
    }
}

/// Draw spline: Catmull-Rom
pub fn draw_spline_catmull_rom(points: &[Vector2], thick: f32, color: Color) {
    unsafe {
        bindings::DrawSplineCatmullRom(
            points.as_ptr(),
            points.len() as c_int,
            thick as c_float,
            color,
        )
    }
}

/// Draw spline: Quadratic Bezier
pub fn draw_spline_bezier_quadratic(points: &[Vector2], thick: f32, color: Color) {
    unsafe {
        bindings::DrawSplineBezierQuadratic(
            points.as_ptr(),
            points.len() as c_int,
            thick as c_float,
            color,
        )
    }
}

/// Draw spline: Cubic Bezier
pub fn draw_spline_bezier_cubic(points: &[Vector2], thick: f32, color: Color) {
    unsafe {
        bindings::DrawSplineBezierCubic(
            points.as_ptr(),
            points.len() as c_int,
            thick as c_float,
            color,
        )
    }
}

/// Draw spline segment: Linear
pub fn draw_spline_segment_linear(p1: Vector2, p2: Vector2, thick: f32, color: Color) {
    unsafe { bindings::DrawSplineSegmentLinear(p1, p2, thick as c_float, color) }
}

/// Draw spline segment: B-Spline
pub fn draw_spline_segment_basis(
    p1: Vector2,
    p2: Vector2,
    p3: Vector2,
    p4: Vector2,
    thick: f32,
    color: Color,
) {
    unsafe { bindings::DrawSplineSegmentBasis(p1, p2, p3, p4, thick as c_float, color) }
}

/// Draw spline segment: Catmull-Rom
pub fn draw_spline_segment_catmull_rom(
    p1: Vector2,
    p2: Vector2,
    p3: Vector2,
    p4: Vector2,
    thick: f32,
    color: Color,
) {
    unsafe { bindings::DrawSplineSegmentCatmullRom(p1, p2, p3, p4, thick as c_float, color) }
}

/// Draw spline segment: Quadratic Bezier
pub fn draw_spline_segment_bezier_quadratic(
    p1: Vector2,
    c2: Vector2,
    p3: Vector2,
    thick: f32,
    color: Color,
) {
    unsafe { bindings::DrawSplineSegmentBezierQuadratic(p1, c2, p3, thick as c_float, color) }
}

/// Draw spline segment: Cubic Bezier
pub fn draw_spline_segment_bezier_cubic(
    p1: Vector2,
    c2: Vector2,
    c3: Vector2,
    p4: Vector2,
    thick: f32,
    color: Color,
) {
    unsafe { bindings::DrawSplineSegmentBezierCubic(p1, c2, c3, p4, thick as c_float, color) }
}

// ---------------------------------------------------------------------------------
// Spline segment point evaluation wrappers
// ---------------------------------------------------------------------------------

/// Get (evaluate) spline point: Linear
pub fn get_spline_point_linear(start_pos: Vector2, end_pos: Vector2, t: f32) -> Vector2 {
    unsafe { bindings::GetSplinePointLinear(start_pos, end_pos, t as c_float) }
}

/// Get (evaluate) spline point: B-Spline
pub fn get_spline_point_basis(
    p1: Vector2,
    p2: Vector2,
    p3: Vector2,
    p4: Vector2,
    t: f32,
) -> Vector2 {
    unsafe { bindings::GetSplinePointBasis(p1, p2, p3, p4, t as c_float) }
}

/// Get (evaluate) spline point: Catmull-Rom
pub fn get_spline_point_catmull_rom(
    p1: Vector2,
    p2: Vector2,
    p3: Vector2,
    p4: Vector2,
    t: f32,
) -> Vector2 {
    unsafe { bindings::GetSplinePointCatmullRom(p1, p2, p3, p4, t as c_float) }
}

/// Get (evaluate) spline point: Quadratic Bezier
pub fn get_spline_point_bezier_quad(p1: Vector2, c2: Vector2, p3: Vector2, t: f32) -> Vector2 {
    unsafe { bindings::GetSplinePointBezierQuad(p1, c2, p3, t as c_float) }
}

/// Get (evaluate) spline point: Cubic Bezier
pub fn get_spline_point_bezier_cubic(
    p1: Vector2,
    c2: Vector2,
    c3: Vector2,
    p4: Vector2,
    t: f32,
) -> Vector2 {
    unsafe { bindings::GetSplinePointBezierCubic(p1, c2, c3, p4, t as c_float) }
}

// ---------------------------------------------------------------------------------
// Basic shapes collision detection wrappers
// ---------------------------------------------------------------------------------

/// Check collision between two rectangles
pub fn check_collision_recs(rec1: Rectangle, rec2: Rectangle) -> bool {
    unsafe { bindings::CheckCollisionRecs(rec1, rec2) }
}

/// Check collision between two circles
pub fn check_collision_circles(
    center1: Vector2,
    radius1: f32,
    center2: Vector2,
    radius2: f32,
) -> bool {
    unsafe {
        bindings::CheckCollisionCircles(
            center1,
            radius1 as c_float,
            center2,
            radius2 as c_float,
        )
    }
}

/// Check collision between circle and rectangle
pub fn check_collision_circle_rec(center: Vector2, radius: f32, rec: Rectangle) -> bool {
    unsafe { bindings::CheckCollisionCircleRec(center, radius as c_float, rec) }
}

/// Check if circle collides with a line
pub fn check_collision_circle_line(
    center: Vector2,
    radius: f32,
    p1: Vector2,
    p2: Vector2,
) -> bool {
    unsafe { bindings::CheckCollisionCircleLine(center, radius as c_float, p1, p2) }
}

/// Check if point is inside rectangle
pub fn check_collision_point_rec(point: Vector2, rec: Rectangle) -> bool {
    unsafe { bindings::CheckCollisionPointRec(point, rec) }
}

/// Check if point is inside circle
pub fn check_collision_point_circle(point: Vector2, center: Vector2, radius: f32) -> bool {
    unsafe { bindings::CheckCollisionPointCircle(point, center, radius as c_float) }
}

/// Check if point is inside a triangle
pub fn check_collision_point_triangle(point: Vector2, p1: Vector2, p2: Vector2, p3: Vector2) -> bool {
    unsafe { bindings::CheckCollisionPointTriangle(point, p1, p2, p3) }
}

/// Check if point belongs to line with defined margin
pub fn check_collision_point_line(
    point: Vector2,
    p1: Vector2,
    p2: Vector2,
    threshold: i32,
) -> bool {
    unsafe { bindings::CheckCollisionPointLine(point, p1, p2, threshold as c_int) }
}

/// Check if point is within a polygon
pub fn check_collision_point_poly(point: Vector2, points: &[Vector2]) -> bool {
    unsafe { bindings::CheckCollisionPointPoly(point, points.as_ptr(), points.len() as c_int) }
}

/// Check the collision between two lines
/// Returns the collision point if they intersect, otherwise `None`.
pub fn check_collision_lines(
    start_pos1: Vector2,
    end_pos1: Vector2,
    start_pos2: Vector2,
    end_pos2: Vector2,
) -> Option<Vector2> {
    let mut collision_point = Vector2 { x: 0.0, y: 0.0 };
    let intersects = unsafe {
        bindings::CheckCollisionLines(
            start_pos1,
            end_pos1,
            start_pos2,
            end_pos2,
            &mut collision_point as *mut Vector2,
        )
    };

    if intersects {
        Some(collision_point)
    } else {
        None
    }
}

/// Get collision rectangle for two rectangles collision
pub fn get_collision_rec(rec1: Rectangle, rec2: Rectangle) -> Rectangle {
    unsafe { bindings::GetCollisionRec(rec1, rec2) }
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

// ---------------------------------------------------------------------------------
// Color/pixel related wrappers
// ---------------------------------------------------------------------------------

/// Check if two colors are equal
pub fn color_is_equal(col1: Color, col2: Color) -> bool {
    unsafe { bindings::ColorIsEqual(col1, col2) }
}

/// Get color with alpha applied
pub fn fade(color: Color, alpha: f32) -> Color {
    unsafe { bindings::Fade(color, alpha as c_float) }
}

/// Get hexadecimal value for a Color (0xRRGGBBAA)
pub fn color_to_int(color: Color) -> i32 {
    unsafe { bindings::ColorToInt(color) }
}

/// Get Color normalized as float [0..1]
pub fn color_normalize(color: Color) -> Vector4 {
    unsafe { bindings::ColorNormalize(color) }
}

/// Get Color from normalized values [0..1]
pub fn color_from_normalized(normalized: Vector4) -> Color {
    unsafe { bindings::ColorFromNormalized(normalized) }
}

/// Get HSV values for a Color
pub fn color_to_hsv(color: Color) -> Vector3 {
    unsafe { bindings::ColorToHSV(color) }
}

/// Get a Color from HSV values
pub fn color_from_hsv(hue: f32, saturation: f32, value: f32) -> Color {
    unsafe {
        bindings::ColorFromHSV(
            hue as c_float,
            saturation as c_float,
            value as c_float,
        )
    }
}

/// Get color multiplied with another color
pub fn color_tint(color: Color, tint: Color) -> Color {
    unsafe { bindings::ColorTint(color, tint) }
}

/// Get color with brightness correction
pub fn color_brightness(color: Color, factor: f32) -> Color {
    unsafe { bindings::ColorBrightness(color, factor as c_float) }
}

/// Get color with contrast correction
pub fn color_contrast(color: Color, contrast: f32) -> Color {
    unsafe { bindings::ColorContrast(color, contrast as c_float) }
}

/// Get color with alpha applied
pub fn color_alpha(color: Color, alpha: f32) -> Color {
    unsafe { bindings::ColorAlpha(color, alpha as c_float) }
}

/// Get src alpha-blended into dst color with tint
pub fn color_alpha_blend(dst: Color, src: Color, tint: Color) -> Color {
    unsafe { bindings::ColorAlphaBlend(dst, src, tint) }
}

/// Get color lerp interpolation between two colors
pub fn color_lerp(color1: Color, color2: Color, factor: f32) -> Color {
    unsafe { bindings::ColorLerp(color1, color2, factor as c_float) }
}

/// Get Color structure from hexadecimal value
pub fn get_color(hex_value: u32) -> Color {
    unsafe { bindings::GetColor(hex_value as c_uint) }
}

/// Get Color from a source pixel pointer of certain format
/// # Safety
/// This function is unsafe because it dereferences a raw pointer.
pub unsafe fn get_pixel_color(src_ptr: *mut c_void, format: PixelFormat) -> Color {
    unsafe {bindings::GetPixelColor(src_ptr, format as c_int)}
}

/// Set color formatted into destination pixel pointer
/// # Safety
/// This function is unsafe because it writes to a raw pointer.
pub unsafe fn set_pixel_color(dst_ptr: *mut c_void, color: Color, format: PixelFormat) {
    unsafe {bindings::SetPixelColor(dst_ptr, color, format as c_int)}
}

/// Get pixel data size in bytes for certain format
pub fn get_pixel_data_size(width: i32, height: i32, format: PixelFormat) -> i32 {
    unsafe { bindings::GetPixelDataSize(width as c_int, height as c_int, format as c_int) }
}

// ---------------------------------------------------------------------------------
// Font Loading and Text Drawing wrappers
// ---------------------------------------------------------------------------------

/// Get the default Font
pub fn get_font_default() -> Font {
    unsafe { bindings::GetFontDefault() }
}

/// Load font from file into GPU memory (VRAM)
pub fn load_font(filename: &str) -> Font {
    let c_filename = CString::new(filename).unwrap();
    unsafe { bindings::LoadFont(c_filename.as_ptr()) }
}

/// Load font from file with extended parameters
pub fn load_font_ex(filename: &str, font_size: i32, codepoints: Option<&[i32]>) -> Font {
    let c_filename = CString::new(filename).unwrap();
    let (codepoints_ptr, codepoints_count) = match codepoints {
        Some(slice) => (slice.as_ptr(), slice.len() as c_int),
        None => (std::ptr::null(), 0 as c_int),
    };
    unsafe {
        bindings::LoadFontEx(
            c_filename.as_ptr(),
            font_size as c_int,
            codepoints_ptr,
            codepoints_count,
        )
    }
}

/// Load font from Image (XNA style)
pub fn load_font_from_image(image: Image, key: Color, first_char: i32) -> Font {
    unsafe { bindings::LoadFontFromImage(image, key, first_char as c_int) }
}

/// Load font from memory buffer
pub fn load_font_from_memory(
    file_type: &str,
    file_data: &[u8],
    font_size: i32,
    codepoints: Option<&[i32]>,
) -> Font {
    let c_file_type = CString::new(file_type).unwrap();
    let (codepoints_ptr, codepoints_count) = match codepoints {
        Some(slice) => (slice.as_ptr(), slice.len() as c_int),
        None => (std::ptr::null(), 0 as c_int),
    };
    unsafe {
        bindings::LoadFontFromMemory(
            c_file_type.as_ptr(),
            file_data.as_ptr(),
            file_data.len() as c_int,
            font_size as c_int,
            codepoints_ptr,
            codepoints_count,
        )
    }
}

/// Check if a font is valid
pub fn is_font_valid(font: Font) -> bool {
    unsafe { bindings::IsFontValid(font) }
}

/// Load font data for further use.
/// NOTE: This wrapper allocates a `Vec<GlyphInfo>` and frees the C-pointer.
pub fn load_font_data(
    file_data: &[u8],
    font_size: i32,
    codepoints: Option<&[i32]>,
    type_: FontType,
) -> Vec<GlyphInfo> {
    let mut glyph_count: c_int = 0;
    let (codepoints_ptr, codepoints_count) = match codepoints {
        Some(slice) => (slice.as_ptr(), slice.len() as c_int),
        None => (std::ptr::null(), 0 as c_int),
    };
    unsafe {
        let glyphs_ptr = bindings::LoadFontData(
            file_data.as_ptr(),
            file_data.len() as c_int,
            font_size as c_int,
            codepoints_ptr,
            codepoints_count,
            type_ as c_int,
            &mut glyph_count as *mut c_int,
        );
        if glyphs_ptr.is_null() {
            return Vec::new();
        }
        let slice = slice::from_raw_parts(glyphs_ptr, glyph_count as usize);
        let vec = slice.to_vec();
        bindings::UnloadFontData(glyphs_ptr, glyph_count);
        vec
    }
}

/// Generate image font atlas using chars info.
/// NOTE: This wrapper allocates and frees the C-rectangle pointer.
pub fn gen_image_font_atlas(
    glyphs: &[GlyphInfo],
    font_size: i32,
    padding: i32,
    pack_method: i32,
) -> (Image, Vec<Rectangle>) {
    let mut recs_ptr: *mut Rectangle = std::ptr::null_mut();
    let glyph_count = glyphs.len() as c_int;
    unsafe {
        let image = bindings::GenImageFontAtlas(
            glyphs.as_ptr(),
            &mut recs_ptr as *mut *mut Rectangle,
            glyph_count,
            font_size as c_int,
            padding as c_int,
            pack_method as c_int,
        );

        if recs_ptr.is_null() {
            return (image, Vec::new());
        }
        let slice = slice::from_raw_parts(recs_ptr, glyph_count as usize);
        let vec = slice.to_vec();
        bindings::MemFree(recs_ptr as *mut c_void); // Corresponds to RL_FREE
        (image, vec)
    }
}

/// Unload font from GPU memory (VRAM)
pub fn unload_font(font: Font) {
    unsafe { bindings::UnloadFont(font) }
}

/// Export font as code file, returns true on success
pub fn export_font_as_code(font: Font, filename: &str) -> bool {
    let c_filename = CString::new(filename).unwrap();
    unsafe { bindings::ExportFontAsCode(font, c_filename.as_ptr()) }
}

// Text drawing functions

/// Draw current FPS
pub fn draw_fps(pos_x: i32, pos_y: i32) {
    unsafe { bindings::DrawFPS(pos_x as c_int, pos_y as c_int) }
}

/// Draw text (using default font)
pub fn draw_text(text: &str, pos_x: i32, pos_y: i32, font_size: i32, color: Color) {
    let c_text = CString::new(text).expect("Failed to create CString");
    unsafe { bindings::DrawText(c_text.as_ptr(), pos_x, pos_y, font_size, color) };
}

/// Draw text using font and additional parameters
pub fn draw_text_ex(
    font: Font,
    text: &str,
    position: Vector2,
    font_size: f32,
    spacing: f32,
    tint: Color,
) {
    let c_text = CString::new(text).unwrap();
    unsafe {
        bindings::DrawTextEx(
            font,
            c_text.as_ptr(),
            position,
            font_size as c_float,
            spacing as c_float,
            tint,
        )
    }
}

/// Draw text using Font and pro parameters (rotation)
pub fn draw_text_pro(
    font: Font,
    text: &str,
    position: Vector2,
    origin: Vector2,
    rotation: f32,
    font_size: f32,
    spacing: f32,
    tint: Color,
) {
    let c_text = CString::new(text).unwrap();
    unsafe {
        bindings::DrawTextPro(
            font,
            c_text.as_ptr(),
            position,
            origin,
            rotation as c_float,
            font_size as c_float,
            spacing as c_float,
            tint,
        )
    }
}

/// Draw one character (codepoint)
pub fn draw_text_codepoint(
    font: Font,
    codepoint: i32,
    position: Vector2,
    font_size: f32,
    tint: Color,
) {
    unsafe {
        bindings::DrawTextCodepoint(
            font,
            codepoint as c_int,
            position,
            font_size as c_float,
            tint,
        )
    }
}

/// Draw multiple character (codepoint)
pub fn draw_text_codepoints(
    font: Font,
    codepoints: &[i32],
    position: Vector2,
    font_size: f32,
    spacing: f32,
    tint: Color,
) {
    unsafe {
        bindings::DrawTextCodepoints(
            font,
            codepoints.as_ptr(),
            codepoints.len() as c_int,
            position,
            font_size as c_float,
            spacing as c_float,
            tint,
        )
    }
}

// Text font info functions

/// Set vertical line spacing when drawing with line-breaks
pub fn set_text_line_spacing(spacing: i32) {
    unsafe { bindings::SetTextLineSpacing(spacing as c_int) }
}

/// Measure string width for default font
pub fn measure_text(text: &str, font_size: i32) -> i32 {
    let c_text = CString::new(text).unwrap();
    unsafe { bindings::MeasureText(c_text.as_ptr(), font_size as c_int) }
}

/// Measure string size for Font
pub fn measure_text_ex(font: Font, text: &str, font_size: f32, spacing: f32) -> Vector2 {
    let c_text = CString::new(text).unwrap();
    unsafe {
        bindings::MeasureTextEx(
            font,
            c_text.as_ptr(),
            font_size as c_float,
            spacing as c_float,
        )
    }
}

/// Get glyph index position in font for a codepoint
pub fn get_glyph_index(font: Font, codepoint: i32) -> i32 {
    unsafe { bindings::GetGlyphIndex(font, codepoint as c_int) }
}

/// Get glyph font info data for a codepoint
pub fn get_glyph_info(font: Font, codepoint: i32) -> GlyphInfo {
    unsafe { bindings::GetGlyphInfo(font, codepoint as c_int) }
}

/// Get glyph rectangle in font atlas for a codepoint
pub fn get_glyph_atlas_rec(font: Font, codepoint: i32) -> Rectangle {
    unsafe { bindings::GetGlyphAtlasRec(font, codepoint as c_int) }
}

// Text codepoints management wrappers

/// Load UTF-8 text encoded from codepoints array
/// NOTE: This wrapper allocates a `String` and frees the C-pointer.
pub fn load_utf8(codepoints: &[i32]) -> String {
    unsafe {
        let c_str_ptr =
            bindings::LoadUTF8(codepoints.as_ptr(), codepoints.len() as c_int);
        if c_str_ptr.is_null() {
            return String::new();
        }
        let c_str = CStr::from_ptr(c_str_ptr);
        let rust_str = c_str.to_string_lossy().into_owned();
        bindings::UnloadUTF8(c_str_ptr);
        rust_str
    }
}

/// Load all codepoints from a UTF-8 text string
/// NOTE: This wrapper allocates a `Vec<i32>` and frees the C-pointer.
pub fn load_codepoints(text: &str) -> Vec<i32> {
    let c_text = CString::new(text).unwrap();
    let mut count: c_int = 0;
    unsafe {
        let codepoints_ptr =
            bindings::LoadCodepoints(c_text.as_ptr(), &mut count as *mut c_int);
        if codepoints_ptr.is_null() {
            return Vec::new();
        }
        let slice = slice::from_raw_parts(codepoints_ptr, count as usize);
        let vec = slice.to_vec();
        bindings::UnloadCodepoints(codepoints_ptr);
        vec
    }
}

/// Get total number of codepoints in a UTF-8 encoded string
pub fn get_codepoint_count(text: &str) -> i32 {
    let c_text = CString::new(text).unwrap();
    unsafe { bindings::GetCodepointCount(c_text.as_ptr()) }
}

/// Get next codepoint in a UTF-8 encoded string
pub fn get_codepoint(text: &str) -> (i32, i32) {
    let c_text = CString::new(text).unwrap();
    let mut codepoint_size: c_int = 0;
    let codepoint =
        unsafe { bindings::GetCodepoint(c_text.as_ptr(), &mut codepoint_size as *mut c_int) };
    (codepoint, codepoint_size)
}

/// Get next codepoint in a UTF-8 encoded string
pub fn get_codepoint_next(text: &str) -> (i32, i32) {
    let c_text = CString::new(text).unwrap();
    let mut codepoint_size: c_int = 0;
    let codepoint =
        unsafe { bindings::GetCodepointNext(c_text.as_ptr(), &mut codepoint_size as *mut c_int) };
    (codepoint, codepoint_size)
}

/// Get previous codepoint in a UTF-8 encoded string
pub fn get_codepoint_previous(text: &str) -> (i32, i32) {
    let c_text = CString::new(text).unwrap();
    let mut codepoint_size: c_int = 0;
    let codepoint = unsafe {
        bindings::GetCodepointPrevious(c_text.as_ptr(), &mut codepoint_size as *mut c_int)
    };
    (codepoint, codepoint_size)
}

/// Encode one codepoint into UTF-8 byte array
/// NOTE: This wrapper copies from a static C buffer.
pub fn codepoint_to_utf8(codepoint: i32) -> (String, i32) {
    let mut utf8_size: c_int = 0;
    unsafe {
        let c_str_ptr =
            bindings::CodepointToUTF8(codepoint as c_int, &mut utf8_size as *mut c_int);
        let rust_str = CStr::from_ptr(c_str_ptr).to_string_lossy().into_owned();
        (rust_str, utf8_size)
    }
}

// Text strings management wrappers

/// Load text as separate lines
/// NOTE: This wrapper allocates a `Vec<String>` and frees the C-pointers.
pub fn load_text_lines(text: &str) -> Vec<String> {
    let c_text = CString::new(text).unwrap();
    let mut count: c_int = 0;
    unsafe {
        let lines_ptr = bindings::LoadTextLines(c_text.as_ptr(), &mut count as *mut c_int);
        if lines_ptr.is_null() {
            return Vec::new();
        }
        let lines_slice = slice::from_raw_parts(lines_ptr, count as usize);
        let mut lines_vec: Vec<String> = Vec::with_capacity(count as usize);
        for &line_ptr in lines_slice {
            if !line_ptr.is_null() {
                lines_vec.push(CStr::from_ptr(line_ptr).to_string_lossy().into_owned());
            }
        }
        bindings::UnloadTextLines(lines_ptr, count);
        lines_vec
    }
}

/// Copy one C string to another, returns bytes copied
/// # Safety
/// This function is unsafe because it writes to a raw pointer `dst`.
/// `dst` must be a valid, null-terminated C string buffer large enough
/// to hold the contents of `src`.
pub unsafe fn text_copy(dst: *mut c_char, src: &str) -> i32 {
    let c_src = CString::new(src).unwrap();
    unsafe { bindings::TextCopy(dst, c_src.as_ptr()) }
}

/// Check if two text string are equal
pub fn text_is_equal(text1: &str, text2: &str) -> bool {
    let c_text1 = CString::new(text1).unwrap();
    let c_text2 = CString::new(text2).unwrap();
    unsafe { bindings::TextIsEqual(c_text1.as_ptr(), c_text2.as_ptr()) }
}

/// Get text length, checks for '\0' ending
pub fn text_length(text: &str) -> u32 {
    let c_text = CString::new(text).unwrap();
    unsafe { bindings::TextLength(c_text.as_ptr()) }
}

/// Text formatting with variables (sprintf() style)
/// NOTE: This function is variadic and cannot be safely wrapped in Rust.
/// Use `format!` macro instead or use `unsafe { bindings::TextFormat(...) }`.
pub fn text_format() {
    // This function is variadic and cannot be safely wrapped.
}

/// Get a piece of a text string
/// NOTE: This wrapper copies from a static C buffer.
pub fn text_subtext(text: &str, position: i32, length: i32) -> String {
    let c_text = CString::new(text).unwrap();
    unsafe {
        let c_str_ptr =
            bindings::TextSubtext(c_text.as_ptr(), position as c_int, length as c_int);
        CStr::from_ptr(c_str_ptr).to_string_lossy().into_owned()
    }
}

/// Remove text spaces, concat words
/// NOTE: This wrapper copies from a static C buffer.
pub fn text_remove_spaces(text: &str) -> String {
    let c_text = CString::new(text).unwrap();
    unsafe {
        let c_str_ptr = bindings::TextRemoveSpaces(c_text.as_ptr());
        CStr::from_ptr(c_str_ptr).to_string_lossy().into_owned()
    }
}

/// Get text between two strings
/// NOTE: This wrapper allocates a `String` and frees the C-pointer.
pub fn get_text_between(text: &str, begin: &str, end: &str) -> String {
    let c_text = CString::new(text).unwrap();
    let c_begin = CString::new(begin).unwrap();
    let c_end = CString::new(end).unwrap();
    unsafe {
        let c_str_ptr =
            bindings::GetTextBetween(c_text.as_ptr(), c_begin.as_ptr(), c_end.as_ptr());
        if c_str_ptr.is_null() {
            return String::new();
        }
        let s = CStr::from_ptr(c_str_ptr).to_string_lossy().into_owned();
        bindings::MemFree(c_str_ptr as *mut c_void);
        s
    }
}

/// Replace text string
/// NOTE: This wrapper allocates a `String` and frees the C-pointer.
pub fn text_replace(text: &str, search: &str, replacement: &str) -> String {
    let c_text = CString::new(text).unwrap();
    let c_search = CString::new(search).unwrap();
    let c_replacement = CString::new(replacement).unwrap();
    unsafe {
        let c_str_ptr = bindings::TextReplace(
            c_text.as_ptr(),
            c_search.as_ptr(),
            c_replacement.as_ptr(),
        );
        if c_str_ptr.is_null() {
            return String::new(); // Or return original string
        }
        let s = CStr::from_ptr(c_str_ptr).to_string_lossy().into_owned();
        bindings::MemFree(c_str_ptr as *mut c_void);
        s
    }
}

/// Replace text between two specific strings
/// NOTE: This wrapper allocates a `String` and frees the C-pointer.
pub fn text_replace_between(
    text: &str,
    begin: &str,
    end: &str,
    replacement: &str,
) -> String {
    let c_text = CString::new(text).unwrap();
    let c_begin = CString::new(begin).unwrap();
    let c_end = CString::new(end).unwrap();
    let c_replacement = CString::new(replacement).unwrap();
    unsafe {
        let c_str_ptr = bindings::TextReplaceBetween(
            c_text.as_ptr(),
            c_begin.as_ptr(),
            c_end.as_ptr(),
            c_replacement.as_ptr(),
        );
        if c_str_ptr.is_null() {
            return String::new();
        }
        let s = CStr::from_ptr(c_str_ptr).to_string_lossy().into_owned();
        bindings::MemFree(c_str_ptr as *mut c_void);
        s
    }
}

/// Insert text in a position
/// NOTE: This wrapper allocates a `String` and frees the C-pointer.
pub fn text_insert(text: &str, insert: &str, position: i32) -> String {
    let c_text = CString::new(text).unwrap();
    let c_insert = CString::new(insert).unwrap();
    unsafe {
        let c_str_ptr =
            bindings::TextInsert(c_text.as_ptr(), c_insert.as_ptr(), position as c_int);
        if c_str_ptr.is_null() {
            return String::new();
        }
        let s = CStr::from_ptr(c_str_ptr).to_string_lossy().into_owned();
        bindings::MemFree(c_str_ptr as *mut c_void);
        s
    }
}

/// Join text strings with delimiter
/// NOTE: This wrapper allocates a `String` and frees the C-pointer.
pub fn text_join(text_list: &[&str], delimiter: &str) -> String {
    let c_delimiter = CString::new(delimiter).unwrap();
    let c_text_list: Vec<CString> = text_list.iter().map(|&s| CString::new(s).unwrap()).collect();
    let c_ptr_list: Vec<*const c_char> = c_text_list.iter().map(|cs| cs.as_ptr()).collect();

    unsafe {
        let c_str_ptr = bindings::TextJoin(
            c_ptr_list.as_ptr(),
            c_ptr_list.len() as c_int,
            c_delimiter.as_ptr(),
        );
        if c_str_ptr.is_null() {
            return String::new();
        }
        let s = CStr::from_ptr(c_str_ptr).to_string_lossy().into_owned();
        bindings::MemFree(c_str_ptr as *mut c_void);
        s
    }
}

/// Split text into multiple strings
/// NOTE: This wrapper copies from static C buffers.
pub fn text_split(text: &str, delimiter: char) -> Vec<String> {
    let c_text = CString::new(text).unwrap();
    let mut count: c_int = 0;
    unsafe {
        let lines_ptr =
            bindings::TextSplit(c_text.as_ptr(), delimiter as c_char, &mut count as *mut c_int);
        if lines_ptr.is_null() {
            return Vec::new();
        }
        let lines_slice = slice::from_raw_parts(lines_ptr, count as usize);
        let mut lines_vec: Vec<String> = Vec::with_capacity(count as usize);
        for &line_ptr in lines_slice {
            if !line_ptr.is_null() {
                lines_vec.push(CStr::from_ptr(line_ptr).to_string_lossy().into_owned());
            }
        }
        // NOTE: No Unload function for TextSplit, it uses a static buffer pool.
        lines_vec
    }
}

/// Append text at specific position and move cursor
/// # Safety
/// This function is unsafe because it writes to a raw pointer `text`.
/// `text` must be a valid C string buffer.
/// `position` will be updated in place.
pub unsafe fn text_append(text: *mut c_char, append: &str, position: &mut i32) {
    let c_append = CString::new(append).unwrap();
    unsafe { bindings::TextAppend(text, c_append.as_ptr(), position as *mut c_int) }
}

/// Find first text occurrence within a string
pub fn text_find_index(text: &str, search: &str) -> i32 {
    let c_text = CString::new(text).unwrap();
    let c_search = CString::new(search).unwrap();
    unsafe { bindings::TextFindIndex(c_text.as_ptr(), c_search.as_ptr()) }
}

/// Get upper case version of provided string
/// NOTE: This wrapper copies from a static C buffer.
pub fn text_to_upper(text: &str) -> String {
    let c_text = CString::new(text).unwrap();
    unsafe {
        let c_str_ptr = bindings::TextToUpper(c_text.as_ptr());
        CStr::from_ptr(c_str_ptr).to_string_lossy().into_owned()
    }
}

/// Get lower case version of provided string
/// NOTE: This wrapper copies from a static C buffer.
pub fn text_to_lower(text: &str) -> String {
    let c_text = CString::new(text).unwrap();
    unsafe {
        let c_str_ptr = bindings::TextToLower(c_text.as_ptr());
        CStr::from_ptr(c_str_ptr).to_string_lossy().into_owned()
    }
}

/// Get Pascal case notation version of provided string
/// NOTE: This wrapper copies from a static C buffer.
pub fn text_to_pascal(text: &str) -> String {
    let c_text = CString::new(text).unwrap();
    unsafe {
        let c_str_ptr = bindings::TextToPascal(c_text.as_ptr());
        CStr::from_ptr(c_str_ptr).to_string_lossy().into_owned()
    }
}

/// Get Snake case notation version of provided string
/// NOTE: This wrapper copies from a static C buffer.
pub fn text_to_snake(text: &str) -> String {
    let c_text = CString::new(text).unwrap();
    unsafe {
        let c_str_ptr = bindings::TextToSnake(c_text.as_ptr());
        CStr::from_ptr(c_str_ptr).to_string_lossy().into_owned()
    }
}

/// Get Camel case notation version of provided string
/// NOTE: This wrapper copies from a static C buffer.
pub fn text_to_camel(text: &str) -> String {
    let c_text = CString::new(text).unwrap();
    unsafe {
        let c_str_ptr = bindings::TextToCamel(c_text.as_ptr());
        CStr::from_ptr(c_str_ptr).to_string_lossy().into_owned()
    }
}

/// Get integer value from text
pub fn text_to_integer(text: &str) -> i32 {
    let c_text = CString::new(text).unwrap();
    unsafe { bindings::TextToInteger(c_text.as_ptr()) }
}

/// Get float value from text
pub fn text_to_float(text: &str) -> f32 {
    let c_text = CString::new(text).unwrap();
    unsafe { bindings::TextToFloat(c_text.as_ptr()) }
}

// ---------------------------------------------------------------------------------
// Basic 3d Shapes Drawing wrappers
// ---------------------------------------------------------------------------------

/// Draw a line in 3D world space
pub fn draw_line_3d(start_pos: Vector3, end_pos: Vector3, color: Color) {
    unsafe { bindings::DrawLine3D(start_pos, end_pos, color) }
}

/// Draw a point in 3D space
pub fn draw_point_3d(position: Vector3, color: Color) {
    unsafe { bindings::DrawPoint3D(position, color) }
}

/// Draw a circle in 3D world space
pub fn draw_circle_3d(
    center: Vector3,
    radius: f32,
    rotation_axis: Vector3,
    rotation_angle: f32,
    color: Color,
) {
    unsafe {
        bindings::DrawCircle3D(
            center,
            radius as c_float,
            rotation_axis,
            rotation_angle as c_float,
            color,
        )
    }
}

/// Draw a color-filled triangle
pub fn draw_triangle_3d(v1: Vector3, v2: Vector3, v3: Vector3, color: Color) {
    unsafe { bindings::DrawTriangle3D(v1, v2, v3, color) }
}

/// Draw a triangle strip defined by points
pub fn draw_triangle_strip_3d(points: &[Vector3], color: Color) {
    unsafe { bindings::DrawTriangleStrip3D(points.as_ptr(), points.len() as c_int, color) }
}

/// Draw cube
pub fn draw_cube(position: Vector3, width: f32, height: f32, length: f32, color: Color) {
    unsafe {
        bindings::DrawCube(
            position,
            width as c_float,
            height as c_float,
            length as c_float,
            color,
        )
    }
}

/// Draw cube (Vector version)
pub fn draw_cube_v(position: Vector3, size: Vector3, color: Color) {
    unsafe { bindings::DrawCubeV(position, size, color) }
}

/// Draw cube wires
pub fn draw_cube_wires(position: Vector3, width: f32, height: f32, length: f32, color: Color) {
    unsafe {
        bindings::DrawCubeWires(
            position,
            width as c_float,
            height as c_float,
            length as c_float,
            color,
        )
    }
}

/// Draw cube wires (Vector version)
pub fn draw_cube_wires_v(position: Vector3, size: Vector3, color: Color) {
    unsafe { bindings::DrawCubeWiresV(position, size, color) }
}

/// Draw sphere
pub fn draw_sphere(center_pos: Vector3, radius: f32, color: Color) {
    unsafe { bindings::DrawSphere(center_pos, radius as c_float, color) }
}

/// Draw sphere with extended parameters
pub fn draw_sphere_ex(
    center_pos: Vector3,
    radius: f32,
    rings: i32,
    slices: i32,
    color: Color,
) {
    unsafe {
        bindings::DrawSphereEx(
            center_pos,
            radius as c_float,
            rings as c_int,
            slices as c_int,
            color,
        )
    }
}

/// Draw sphere wires
pub fn draw_sphere_wires(
    center_pos: Vector3,
    radius: f32,
    rings: i32,
    slices: i32,
    color: Color,
) {
    unsafe {
        bindings::DrawSphereWires(
            center_pos,
            radius as c_float,
            rings as c_int,
            slices as c_int,
            color,
        )
    }
}

/// Draw a cylinder/cone
pub fn draw_cylinder(
    position: Vector3,
    radius_top: f32,
    radius_bottom: f32,
    height: f32,
    slices: i32,
    color: Color,
) {
    unsafe {
        bindings::DrawCylinder(
            position,
            radius_top as c_float,
            radius_bottom as c_float,
            height as c_float,
            slices as c_int,
            color,
        )
    }
}

/// Draw a cylinder with base at startPos and top at endPos
pub fn draw_cylinder_ex(
    start_pos: Vector3,
    end_pos: Vector3,
    start_radius: f32,
    end_radius: f32,
    sides: i32,
    color: Color,
) {
    unsafe {
        bindings::DrawCylinderEx(
            start_pos,
            end_pos,
            start_radius as c_float,
            end_radius as c_float,
            sides as c_int,
            color,
        )
    }
}

/// Draw a cylinder/cone wires
pub fn draw_cylinder_wires(
    position: Vector3,
    radius_top: f32,
    radius_bottom: f32,
    height: f32,
    slices: i32,
    color: Color,
) {
    unsafe {
        bindings::DrawCylinderWires(
            position,
            radius_top as c_float,
            radius_bottom as c_float,
            height as c_float,
            slices as c_int,
            color,
        )
    }
}

/// Draw a cylinder wires with base at startPos and top at endPos
pub fn draw_cylinder_wires_ex(
    start_pos: Vector3,
    end_pos: Vector3,
    start_radius: f32,
    end_radius: f32,
    sides: i32,
    color: Color,
) {
    unsafe {
        bindings::DrawCylinderWiresEx(
            start_pos,
            end_pos,
            start_radius as c_float,
            end_radius as c_float,
            sides as c_int,
            color,
        )
    }
}

/// Draw a capsule
pub fn draw_capsule(
    start_pos: Vector3,
    end_pos: Vector3,
    radius: f32,
    slices: i32,
    rings: i32,
    color: Color,
) {
    unsafe {
        bindings::DrawCapsule(
            start_pos,
            end_pos,
            radius as c_float,
            slices as c_int,
            rings as c_int,
            color,
        )
    }
}

/// Draw capsule wireframe
pub fn draw_capsule_wires(
    start_pos: Vector3,
    end_pos: Vector3,
    radius: f32,
    slices: i32,
    rings: i32,
    color: Color,
) {
    unsafe {
        bindings::DrawCapsuleWires(
            start_pos,
            end_pos,
            radius as c_float,
            slices as c_int,
            rings as c_int,
            color,
        )
    }
}

/// Draw a plane XZ
pub fn draw_plane(center_pos: Vector3, size: Vector2, color: Color) {
    unsafe { bindings::DrawPlane(center_pos, size, color) }
}

/// Draw a ray line
pub fn draw_ray(ray: Ray, color: Color) {
    unsafe { bindings::DrawRay(ray, color) }
}

/// Draw a grid (centered at (0, 0, 0))
pub fn draw_grid(slices: i32, spacing: f32) {
    unsafe { bindings::DrawGrid(slices as c_int, spacing as c_float) }
}

// ---------------------------------------------------------------------------------
// Model 3d Loading and Drawing wrappers
// ---------------------------------------------------------------------------------

// Model management wrappers

/// Load model from files (meshes and materials)
pub fn load_model(filename: &str) -> Model {
    let c_filename = CString::new(filename).unwrap();
    unsafe { bindings::LoadModel(c_filename.as_ptr()) }
}

/// Load model from generated mesh (default material)
pub fn load_model_from_mesh(mesh: Mesh) -> Model {
    unsafe { bindings::LoadModelFromMesh(mesh) }
}

/// Check if a model is valid (loaded in GPU, VAO/VBOs)
pub fn is_model_valid(model: Model) -> bool {
    unsafe { bindings::IsModelValid(model) }
}

/// Unload model (including meshes) from memory (RAM and/or VRAM)
pub fn unload_model(model: Model) {
    unsafe { bindings::UnloadModel(model) }
}

/// Compute model bounding box limits (considers all meshes)
pub fn get_model_bounding_box(model: Model) -> BoundingBox {
    unsafe { bindings::GetModelBoundingBox(model) }
}

// Model drawing wrappers

/// Draw a model (with texture if set)
pub fn draw_model(model: Model, position: Vector3, scale: f32, tint: Color) {
    unsafe { bindings::DrawModel(model, position, scale as c_float, tint) }
}

/// Draw a model with extended parameters
pub fn draw_model_ex(
    model: Model,
    position: Vector3,
    rotation_axis: Vector3,
    rotation_angle: f32,
    scale: Vector3,
    tint: Color,
) {
    unsafe {
        bindings::DrawModelEx(
            model,
            position,
            rotation_axis,
            rotation_angle as c_float,
            scale,
            tint,
        )
    }
}

/// Draw a model wires (with texture if set)
pub fn draw_model_wires(model: Model, position: Vector3, scale: f32, tint: Color) {
    unsafe { bindings::DrawModelWires(model, position, scale as c_float, tint) }
}

/// Draw a model wires (with texture if set) with extended parameters
pub fn draw_model_wires_ex(
    model: Model,
    position: Vector3,
    rotation_axis: Vector3,
    rotation_angle: f32,
    scale: Vector3,
    tint: Color,
) {
    unsafe {
        bindings::DrawModelWiresEx(
            model,
            position,
            rotation_axis,
            rotation_angle as c_float,
            scale,
            tint,
        )
    }
}

/// Draw a model as points
pub fn draw_model_points(model: Model, position: Vector3, scale: f32, tint: Color) {
    unsafe { bindings::DrawModelPoints(model, position, scale as c_float, tint) }
}

/// Draw a model as points with extended parameters
pub fn draw_model_points_ex(
    model: Model,
    position: Vector3,
    rotation_axis: Vector3,
    rotation_angle: f32,
    scale: Vector3,
    tint: Color,
) {
    unsafe {
        bindings::DrawModelPointsEx(
            model,
            position,
            rotation_axis,
            rotation_angle as c_float,
            scale,
            tint,
        )
    }
}

/// Draw bounding box (wires)
pub fn draw_bounding_box(box_: BoundingBox, color: Color) {
    unsafe { bindings::DrawBoundingBox(box_, color) }
}

/// Draw a billboard texture
pub fn draw_billboard(
    camera: Camera,
    texture: Texture2D,
    position: Vector3,
    scale: f32,
    tint: Color,
) {
    unsafe { bindings::DrawBillboard(camera, texture, position, scale as c_float, tint) }
}

/// Draw a billboard texture defined by source
pub fn draw_billboard_rec(
    camera: Camera,
    texture: Texture2D,
    source: Rectangle,
    position: Vector3,
    size: Vector2,
    tint: Color,
) {
    unsafe { bindings::DrawBillboardRec(camera, texture, source, position, size, tint) }
}

/// Draw a billboard texture defined by source and rotation
pub fn draw_billboard_pro(
    camera: Camera,
    texture: Texture2D,
    source: Rectangle,
    position: Vector3,
    up: Vector3,
    size: Vector2,
    origin: Vector2,
    rotation: f32,
    tint: Color,
) {
    unsafe {
        bindings::DrawBillboardPro(
            camera,
            texture,
            source,
            position,
            up,
            size,
            origin,
            rotation as c_float,
            tint,
        )
    }
}

// Mesh management wrappers

/// Upload mesh vertex data in GPU and provide VAO/VBO ids
pub fn upload_mesh(mesh: &mut Mesh, dynamic: bool) {
    unsafe { bindings::UploadMesh(mesh as *mut Mesh, dynamic) }
}

/// Update mesh vertex data in GPU for a specific buffer index
pub fn update_mesh_buffer<T>(mesh: Mesh, index: i32, data: &[T], offset: i32) {
    unsafe {
        bindings::UpdateMeshBuffer(
            mesh,
            index as c_int,
            data.as_ptr() as *const c_void,
            (data.len() * size_of::<T>()) as c_int,
            offset as c_int,
        )
    }
}

/// Unload mesh data from CPU and GPU
pub fn unload_mesh(mesh: Mesh) {
    unsafe { bindings::UnloadMesh(mesh) }
}

/// Draw a 3d mesh with material and transform
pub fn draw_mesh(mesh: Mesh, material: Material, transform: Matrix) {
    unsafe { bindings::DrawMesh(mesh, material, transform) }
}

/// Draw multiple mesh instances with material and different transforms
pub fn draw_mesh_instanced(mesh: Mesh, material: Material, transforms: &[Matrix]) {
    unsafe {
        bindings::DrawMeshInstanced(
            mesh,
            material,
            transforms.as_ptr(),
            transforms.len() as c_int,
        )
    }
}

/// Compute mesh bounding box limits
pub fn get_mesh_bounding_box(mesh: Mesh) -> BoundingBox {
    unsafe { bindings::GetMeshBoundingBox(mesh) }
}

/// Compute mesh tangents
pub fn gen_mesh_tangents(mesh: &mut Mesh) {
    unsafe { bindings::GenMeshTangents(mesh as *mut Mesh) }
}

/// Export mesh data to file, returns true on success
pub fn export_mesh(mesh: Mesh, filename: &str) -> bool {
    let c_filename = CString::new(filename).unwrap();
    unsafe { bindings::ExportMesh(mesh, c_filename.as_ptr()) }
}

/// Export mesh as code file (.h) defining multiple arrays of vertex attributes
pub fn export_mesh_as_code(mesh: Mesh, filename: &str) -> bool {
    let c_filename = CString::new(filename).unwrap();
    unsafe { bindings::ExportMeshAsCode(mesh, c_filename.as_ptr()) }
}

// Mesh generation wrappers

/// Generate polygonal mesh
pub fn gen_mesh_poly(sides: i32, radius: f32) -> Mesh {
    unsafe { bindings::GenMeshPoly(sides as c_int, radius as c_float) }
}

/// Generate plane mesh (with subdivisions)
pub fn gen_mesh_plane(width: f32, length: f32, res_x: i32, res_z: i32) -> Mesh {
    unsafe {
        bindings::GenMeshPlane(
            width as c_float,
            length as c_float,
            res_x as c_int,
            res_z as c_int,
        )
    }
}

/// Generate cuboid mesh
pub fn gen_mesh_cube(width: f32, height: f32, length: f32) -> Mesh {
    unsafe {
        bindings::GenMeshCube(
            width as c_float,
            height as c_float,
            length as c_float,
        )
    }
}

/// Generate sphere mesh (standard sphere)
pub fn gen_mesh_sphere(radius: f32, rings: i32, slices: i32) -> Mesh {
    unsafe { bindings::GenMeshSphere(radius as c_float, rings as c_int, slices as c_int) }
}

/// Generate half-sphere mesh (no bottom cap)
pub fn gen_mesh_hemi_sphere(radius: f32, rings: i32, slices: i32) -> Mesh {
    unsafe { bindings::GenMeshHemiSphere(radius as c_float, rings as c_int, slices as c_int) }
}

/// Generate cylinder mesh
pub fn gen_mesh_cylinder(radius: f32, height: f32, slices: i32) -> Mesh {
    unsafe { bindings::GenMeshCylinder(radius as c_float, height as c_float, slices as c_int) }
}

/// Generate cone/pyramid mesh
pub fn gen_mesh_cone(radius: f32, height: f32, slices: i32) -> Mesh {
    unsafe { bindings::GenMeshCone(radius as c_float, height as c_float, slices as c_int) }
}

/// Generate torus mesh
pub fn gen_mesh_torus(radius: f32, size: f32, rad_seg: i32, sides: i32) -> Mesh {
    unsafe {
        bindings::GenMeshTorus(
            radius as c_float,
            size as c_float,
            rad_seg as c_int,
            sides as c_int,
        )
    }
}

/// Generate trefoil knot mesh
pub fn gen_mesh_knot(radius: f32, size: f32, rad_seg: i32, sides: i32) -> Mesh {
    unsafe {
        bindings::GenMeshKnot(
            radius as c_float,
            size as c_float,
            rad_seg as c_int,
            sides as c_int,
        )
    }
}

/// Generate heightmap mesh from image data
pub fn gen_mesh_heightmap(heightmap: Image, size: Vector3) -> Mesh {
    unsafe { bindings::GenMeshHeightmap(heightmap, size) }
}

/// Generate cubes-based map mesh from image data
pub fn gen_mesh_cubicmap(cubicmap: Image, cube_size: Vector3) -> Mesh {
    unsafe { bindings::GenMeshCubicmap(cubicmap, cube_size) }
}

// Material loading/unloading wrappers

/// Load materials from model file
pub fn load_materials(filename: &str) -> Vec<Material> {
    let c_filename = CString::new(filename).unwrap();
    let mut count: c_int = 0;
    unsafe {
        let materials_ptr =
            bindings::LoadMaterials(c_filename.as_ptr(), &mut count as *mut c_int);
        if materials_ptr.is_null() {
            return Vec::new();
        }
        let slice = slice::from_raw_parts(materials_ptr, count as usize);
        let vec = slice.to_vec();
        bindings::MemFree(materials_ptr as *mut c_void); // Free the *array*, not the materials
        return vec;
    }
}

/// Load default material (Supports: DIFFUSE, SPECULAR, NORMAL maps)
pub fn load_material_default() -> Material {
    unsafe { bindings::LoadMaterialDefault() }
}

/// Check if a material is valid (shader assigned, map textures loaded in GPU)
pub fn is_material_valid(material: Material) -> bool {
    unsafe { bindings::IsMaterialValid(material) }
}

/// Unload material from GPU memory (VRAM)
pub fn unload_material(material: Material) {
    unsafe { bindings::UnloadMaterial(material) }
}

/// Set texture for a material map type
pub fn set_material_texture(
    material: &mut Material,
    map_type: MaterialMapIndex,
    texture: Texture2D,
) {
    unsafe { bindings::SetMaterialTexture(material as *mut Material, map_type as c_int, texture) }
}

/// Set material for a mesh
pub fn set_model_mesh_material(model: &mut Model, mesh_id: i32, material_id: i32) {
    unsafe {
        bindings::SetModelMeshMaterial(
            model as *mut Model,
            mesh_id as c_int,
            material_id as c_int,
        )
    }
}

// Model animations loading/unloading wrappers

/// Load model animations from file
/// NOTE: This wrapper allocates a `Vec<ModelAnimation>` and frees the C-pointer.
pub fn load_model_animations(filename: &str) -> Vec<ModelAnimation> {
    let c_filename = CString::new(filename).unwrap();
    let mut count: c_int = 0;
    unsafe {
        let animations_ptr =
            bindings::LoadModelAnimations(c_filename.as_ptr(), &mut count as *mut c_int);
        if animations_ptr.is_null() {
            return Vec::new();
        }
        let slice = slice::from_raw_parts_mut(animations_ptr, count as usize);
        let vec = slice.to_vec();
        bindings::UnloadModelAnimations(animations_ptr, count);
        vec
    }
}

/// Update model animation pose (CPU)
pub fn update_model_animation(model: Model, anim: ModelAnimation, frame: i32) {
    unsafe { bindings::UpdateModelAnimation(model, anim, frame as c_int) }
}

/// Update model animation mesh bone matrices (GPU skinning)
pub fn update_model_animation_bones(model: Model, anim: ModelAnimation, frame: i32) {
    unsafe { bindings::UpdateModelAnimationBones(model, anim, frame as c_int) }
}

/// Unload animation data
pub fn unload_model_animation(anim: ModelAnimation) {
    unsafe { bindings::UnloadModelAnimation(anim) }
}

/// Check model animation skeleton match
pub fn is_model_animation_valid(model: Model, anim: ModelAnimation) -> bool {
    unsafe { bindings::IsModelAnimationValid(model, anim) }
}

// Collision detection wrappers

/// Check collision between two spheres
pub fn check_collision_spheres(
    center1: Vector3,
    radius1: f32,
    center2: Vector3,
    radius2: f32,
) -> bool {
    unsafe {
        bindings::CheckCollisionSpheres(
            center1,
            radius1 as c_float,
            center2,
            radius2 as c_float,
        )
    }
}

/// Check collision between two bounding boxes
pub fn check_collision_boxes(box1: BoundingBox, box2: BoundingBox) -> bool {
    unsafe { bindings::CheckCollisionBoxes(box1, box2) }
}

/// Check collision between box and sphere
pub fn check_collision_box_sphere(box_: BoundingBox, center: Vector3, radius: f32) -> bool {
    unsafe { bindings::CheckCollisionBoxSphere(box_, center, radius as c_float) }
}

/// Get collision info between ray and sphere
pub fn get_ray_collision_sphere(ray: Ray, center: Vector3, radius: f32) -> RayCollision {
    unsafe { bindings::GetRayCollisionSphere(ray, center, radius as c_float) }
}

/// Get collision info between ray and box
pub fn get_ray_collision_box(ray: Ray, box_: BoundingBox) -> RayCollision {
    unsafe { bindings::GetRayCollisionBox(ray, box_) }
}

/// Get collision info between ray and mesh
pub fn get_ray_collision_mesh(ray: Ray, mesh: Mesh, transform: Matrix) -> RayCollision {
    unsafe { bindings::GetRayCollisionMesh(ray, mesh, transform) }
}

/// Get collision info between ray and triangle
pub fn get_ray_collision_triangle(ray: Ray, p1: Vector3, p2: Vector3, p3: Vector3) -> RayCollision {
    unsafe { bindings::GetRayCollisionTriangle(ray, p1, p2, p3) }
}

/// Get collision info between ray and quad
pub fn get_ray_collision_quad(
    ray: Ray,
    p1: Vector3,
    p2: Vector3,
    p3: Vector3,
    p4: Vector3,
) -> RayCollision {
    unsafe { bindings::GetRayCollisionQuad(ray, p1, p2, p3, p4) }
}

// ---------------------------------------------------------------------------------
// Audio Loading and Playing wrappers
// ---------------------------------------------------------------------------------

// Audio device management wrappers

/// Initialize audio device and context
pub fn init_audio_device() {
    unsafe { bindings::InitAudioDevice() }
}

/// Close the audio device and context
pub fn close_audio_device() {
    unsafe { bindings::CloseAudioDevice() }
}

/// Check if audio device has been initialized successfully
pub fn is_audio_device_ready() -> bool {
    unsafe { bindings::IsAudioDeviceReady() }
}

/// Set master volume (listener)
pub fn set_master_volume(volume: f32) {
    unsafe { bindings::SetMasterVolume(volume as c_float) }
}

/// Get master volume (listener)
pub fn get_master_volume() -> f32 {
    unsafe { bindings::GetMasterVolume() }
}

// Wave/Sound loading/unloading wrappers

/// Load wave data from file
pub fn load_wave(filename: &str) -> Wave {
    let c_filename = CString::new(filename).unwrap();
    unsafe { bindings::LoadWave(c_filename.as_ptr()) }
}

/// Load wave from memory buffer
pub fn load_wave_from_memory(file_type: &str, file_data: &[u8]) -> Wave {
    let c_file_type = CString::new(file_type).unwrap();
    unsafe {
        bindings::LoadWaveFromMemory(
            c_file_type.as_ptr(),
            file_data.as_ptr(),
            file_data.len() as c_int,
        )
    }
}

/// Checks if wave data is valid
pub fn is_wave_valid(wave: Wave) -> bool {
    unsafe { bindings::IsWaveValid(wave) }
}

/// Load sound from file
pub fn load_sound(filename: &str) -> Sound {
    let c_filename = CString::new(filename).unwrap();
    unsafe { bindings::LoadSound(c_filename.as_ptr()) }
}

/// Load sound from wave data
pub fn load_sound_from_wave(wave: Wave) -> Sound {
    unsafe { bindings::LoadSoundFromWave(wave) }
}

/// Create a new sound that shares the same sample data as the source sound
pub fn load_sound_alias(source: Sound) -> Sound {
    return unsafe {
            bindings::LoadSoundAlias(source)
    };
}

/// Checks if a sound is valid
pub fn is_sound_valid(sound: Sound) -> bool {
    unsafe { bindings::IsSoundValid(sound) }
}

/// Update sound buffer with new data
pub fn update_sound<T>(sound: Sound, data: &[T]) {
    unsafe {
        bindings::UpdateSound(
            sound,
            data.as_ptr() as *const c_void,
            data.len() as c_int,
        )
    }
}

/// Unload wave data
pub fn unload_wave(wave: Wave) {
    unsafe { bindings::UnloadWave(wave) }
}

/// Unload sound
pub fn unload_sound(sound: Sound) {
    unsafe { bindings::UnloadSound(sound) }
}

/// Unload a sound alias (does not deallocate sample data)
pub fn unload_sound_alias(alias: Sound) {
    unsafe {bindings::UnloadSoundAlias(alias)}
}

/// Export wave data to file
pub fn export_wave(wave: Wave, filename: &str) -> bool {
    let c_filename = CString::new(filename).unwrap();
    unsafe { bindings::ExportWave(wave, c_filename.as_ptr()) }
}

/// Export wave sample data to code (.h)
pub fn export_wave_as_code(wave: Wave, filename: &str) -> bool {
    let c_filename = CString::new(filename).unwrap();
    unsafe { bindings::ExportWaveAsCode(wave, c_filename.as_ptr()) }
}

// Wave/Sound management wrappers

/// Play a sound
pub fn play_sound(sound: Sound) {
    unsafe { bindings::PlaySound(sound) }
}

/// Stop playing a sound
pub fn stop_sound(sound: Sound) {
    unsafe { bindings::StopSound(sound) }
}

/// Pause a sound
pub fn pause_sound(sound: Sound) {
    unsafe { bindings::PauseSound(sound) }
}

/// Resume a paused sound
pub fn resume_sound(sound: Sound) {
    unsafe { bindings::ResumeSound(sound) }
}

/// Check if a sound is currently playing
pub fn is_sound_playing(sound: Sound) -> bool {
    unsafe { bindings::IsSoundPlaying(sound) }
}

/// Set volume for a sound
pub fn set_sound_volume(sound: Sound, volume: f32) {
    unsafe { bindings::SetSoundVolume(sound, volume as c_float) }
}

/// Set pitch for a sound
pub fn set_sound_pitch(sound: Sound, pitch: f32) {
    unsafe { bindings::SetSoundPitch(sound, pitch as c_float) }
}

/// Set pan for a sound
pub fn set_sound_pan(sound: Sound, pan: f32) {
    unsafe { bindings::SetSoundPan(sound, pan as c_float) }
}

/// Copy a wave to a new wave
pub fn wave_copy(wave: Wave) -> Wave {
    unsafe { bindings::WaveCopy(wave) }
}

/// Crop a wave to defined frames range
pub fn wave_crop(wave: &mut Wave, init_frame: i32, final_frame: i32) {
    unsafe {
        bindings::WaveCrop(
            wave as *mut Wave,
            init_frame as c_int,
            final_frame as c_int,
        )
    }
}

/// Convert wave data to desired format
pub fn wave_format(wave: &mut Wave, sample_rate: i32, sample_size: i32, channels: i32) {
    unsafe {
        bindings::WaveFormat(
            wave as *mut Wave,
            sample_rate as c_int,
            sample_size as c_int,
            channels as c_int,
        )
    }
}

/// Load samples data from wave as a 32bit float data array
/// NOTE: This wrapper allocates a `Vec<f32>` and frees the C-pointer.
pub fn load_wave_samples(wave: Wave) -> Vec<f32> {
    unsafe {
        let samples_ptr = bindings::LoadWaveSamples(wave);
        if samples_ptr.is_null() {
            return Vec::new();
        }
        let count = (wave.frame_count * wave.channels) as usize;
        let slice = slice::from_raw_parts(samples_ptr, count);
        let vec = slice.to_vec();
        bindings::UnloadWaveSamples(samples_ptr);
        vec
    }
}

// Music management wrappers

/// Load music stream from file
pub fn load_music_stream(filename: &str) -> Music {
    let c_filename = CString::new(filename).unwrap();
    unsafe { bindings::LoadMusicStream(c_filename.as_ptr()) }
}

/// Load music stream from data
pub fn load_music_stream_from_memory(file_type: &str, data: &[u8]) -> Music {
    let c_file_type = CString::new(file_type).unwrap();
    unsafe {
        bindings::LoadMusicStreamFromMemory(
            c_file_type.as_ptr(),
            data.as_ptr(),
            data.len() as c_int,
        )
    }
}

/// Checks if a music stream is valid
pub fn is_music_valid(music: Music) -> bool {
    unsafe { bindings::IsMusicValid(music) }
}

/// Unload music stream
pub fn unload_music_stream(music: Music) {
    unsafe { bindings::UnloadMusicStream(music) }
}

/// Start music playing
pub fn play_music_stream(music: Music) {
    unsafe { bindings::PlayMusicStream(music) }
}

/// Check if music is playing
pub fn is_music_stream_playing(music: Music) -> bool {
    unsafe { bindings::IsMusicStreamPlaying(music) }
}

/// Updates buffers for music streaming
pub fn update_music_stream(music: Music) {
    unsafe { bindings::UpdateMusicStream(music) }
}

/// Stop music playing
pub fn stop_music_stream(music: Music) {
    unsafe { bindings::StopMusicStream(music) }
}

/// Pause music playing
pub fn pause_music_stream(music: Music) {
    unsafe { bindings::PauseMusicStream(music) }
}

/// Resume playing paused music
pub fn resume_music_stream(music: Music) {
    unsafe { bindings::ResumeMusicStream(music) }
}

/// Seek music to a position (in seconds)
pub fn seek_music_stream(music: Music, position: f32) {
    unsafe { bindings::SeekMusicStream(music, position as c_float) }
}

/// Set volume for music
pub fn set_music_volume(music: Music, volume: f32) {
    unsafe { bindings::SetMusicVolume(music, volume as c_float) }
}

/// Set pitch for a music
pub fn set_music_pitch(music: Music, pitch: f32) {
    unsafe { bindings::SetMusicPitch(music, pitch as c_float) }
}

/// Set pan for a music
pub fn set_music_pan(music: Music, pan: f32) {
    unsafe { bindings::SetMusicPan(music, pan as c_float) }
}

/// Get music time length (in seconds)
pub fn get_music_time_length(music: Music) -> f32 {
    unsafe { bindings::GetMusicTimeLength(music) }
}

/// Get current music time played (in seconds)
pub fn get_music_time_played(music: Music) -> f32 {
    unsafe { bindings::GetMusicTimePlayed(music) }
}

// AudioStream management wrappers

/// Load audio stream (to stream raw audio pcm data)
pub fn load_audio_stream(sample_rate: u32, sample_size: u32, channels: u32) -> AudioStream {
    unsafe {
        bindings::LoadAudioStream(
            sample_rate as c_uint,
            sample_size as c_uint,
            channels as c_uint,
        )
    }
}

/// Checks if an audio stream is valid
pub fn is_audio_stream_valid(stream: AudioStream) -> bool {
    unsafe { bindings::IsAudioStreamValid(stream) }
}

/// Unload audio stream and free memory
pub fn unload_audio_stream(stream: AudioStream) {
    unsafe { bindings::UnloadAudioStream(stream) }
}

/// Update audio stream buffers with data
pub fn update_audio_stream<T>(stream: AudioStream, data: &[T]) {
    unsafe {
        bindings::UpdateAudioStream(
            stream,
            data.as_ptr() as *const c_void,
            data.len() as c_int,
        )
    }
}

/// Check if any audio stream buffers requires refill
pub fn is_audio_stream_processed(stream: AudioStream) -> bool {
    unsafe { bindings::IsAudioStreamProcessed(stream) }
}

/// Play audio stream
pub fn play_audio_stream(stream: AudioStream) {
    unsafe { bindings::PlayAudioStream(stream) }
}

/// Pause audio stream
pub fn pause_audio_stream(stream: AudioStream) {
    unsafe { bindings::PauseAudioStream(stream) }
}

/// Resume audio stream
pub fn resume_audio_stream(stream: AudioStream) {
    unsafe { bindings::ResumeAudioStream(stream) }
}

/// Check if audio stream is playing
pub fn is_audio_stream_playing(stream: AudioStream) -> bool {
    unsafe { bindings::IsAudioStreamPlaying(stream) }
}

/// Stop audio stream
pub fn stop_audio_stream(stream: AudioStream) {
    unsafe { bindings::StopAudioStream(stream) }
}

/// Set volume for audio stream
pub fn set_audio_stream_volume(stream: AudioStream, volume: f32) {
    unsafe { bindings::SetAudioStreamVolume(stream, volume as c_float) }
}

/// Set pitch for audio stream
pub fn set_audio_stream_pitch(stream: AudioStream, pitch: f32) {
    unsafe { bindings::SetAudioStreamPitch(stream, pitch as c_float) }
}

/// Set pan for audio stream
pub fn set_audio_stream_pan(stream: AudioStream, pan: f32) {
    unsafe { bindings::SetAudioStreamPan(stream, pan as c_float) }
}

/// Default size for new audio streams
pub fn set_audio_stream_buffer_size_default(size: i32) {
    unsafe { bindings::SetAudioStreamBufferSizeDefault(size as c_int) }
}

/// Audio thread callback to request new data
pub fn set_audio_stream_callback(stream: AudioStream, callback: AudioCallback) {
    unsafe { bindings::SetAudioStreamCallback(stream, callback) }
}

/// Attach audio stream processor to stream
pub fn attach_audio_stream_processor(stream: AudioStream, processor: AudioCallback) {
    unsafe { bindings::AttachAudioStreamProcessor(stream, processor) }
}

/// Detach audio stream processor from stream
pub fn detach_audio_stream_processor(stream: AudioStream, processor: AudioCallback) {
    unsafe { bindings::DetachAudioStreamProcessor(stream, processor) }
}

/// Attach audio stream processor to the entire audio pipeline
pub fn attach_audio_mixed_processor(processor: AudioCallback) {
    unsafe { bindings::AttachAudioMixedProcessor(processor) }
}

/// Detach audio stream processor from the entire audio pipeline
pub fn detach_audio_mixed_processor(processor: AudioCallback) {
    unsafe { bindings::DetachAudioMixedProcessor(processor) }
}