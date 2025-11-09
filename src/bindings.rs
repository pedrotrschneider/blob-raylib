#![allow(non_snake_case)]

use crate::types::Color;
use crate::{Rectangle, Texture2D, Vector2};
use std::ffi::c_uint;
use std::os::raw::{c_char, c_float, c_int};

unsafe extern "C" {
    // Window and Graphics Device Management
    pub(crate) fn InitWindow(width: c_int, height: c_int, title: *const c_char);
    pub(crate) fn WindowShouldClose() -> bool;
    pub(crate) fn CloseWindow();

    // Timing related functions
    pub(crate) fn SetTargetFPS(fps: c_int);
    pub(crate) fn GetFPS() -> c_int;
    pub(crate) fn GetFrameTime() -> c_float;

    // Drawing-related functions
    pub(crate) fn BeginDrawing();
    pub(crate) fn EndDrawing();
    pub(crate) fn ClearBackground(color: Color);
    pub(crate) fn DrawText(text: *const c_char, posX: c_int, posY: c_int, fontSize: c_int, color: Color);

    // Basic Shapes drawing functions
    pub(crate) fn DrawCircle(centerX: c_int, centerY: c_int, radius: c_float, color: Color);
    pub(crate) fn DrawCircleV(center: Vector2, radius: c_float, color: Color);
    pub(crate) fn DrawRectangle(posX: c_int, posY: c_int, width: c_int, height: c_int, color: Color);
    pub(crate) fn DrawRectangleRounded(rect: Rectangle, roundness: c_float, segments: c_int, color: Color);
    pub(crate) fn DrawTriangle(v1: Vector2, v2: Vector2, v3: Vector2, color: Color);

    // Input-related functions: keyboard
    pub(crate) fn IsKeyPressed(key: c_int) -> bool;
    pub(crate) fn IsKeyPressedRepeat(key: c_int) -> bool;
    pub(crate) fn IsKeyDown(key: c_int) -> bool;

    // Input-related functions: gamepads
    pub(crate) fn IsGamepadAvailable(gamepad: c_int) -> bool;
    pub(crate) fn GetGamepadName(gamepad: c_int) -> *const c_char;
    pub(crate) fn IsGamepadButtonPressed(gamepad: c_int, button: c_int) -> bool;
    pub(crate) fn IsGamepadButtonDown(gamepad: c_int, button: c_int) -> bool;
    pub(crate) fn IsGamepadButtonReleased(gamepad: c_int, button: c_int) -> bool;
    pub(crate) fn IsGamepadButtonUp(gamepad: c_int, button: c_int) -> bool;
    pub(crate) fn GetGamepadButtonPressed() -> c_int;
    pub(crate) fn GetGamepadAxisCount(gamepad: c_int) -> c_int;
    pub(crate) fn GetGamepadAxisMovement(gamepad: c_int, axis: c_int) -> c_float;
    pub(crate) fn SetGamepadMappings(mappings: *const c_char) -> c_int;
    pub(crate) fn SetGamepadVibration(gamepad: c_int, left_motor: c_float, right_motor: c_float, duration: c_float);

    // Input-related functions: mouse
    pub(crate) fn IsMouseButtonPressed(key: c_int) -> bool;
    pub(crate) fn IsMouseButtonDown(key: c_int) -> bool;
    pub(crate) fn IsMouseButtonReleased(key: c_int) -> bool;
    pub(crate) fn IsMouseButtonUp(key: c_int) -> bool;
    pub(crate) fn GetMouseX() -> c_int;
    pub(crate) fn GetMouseY() -> c_int;
    pub(crate) fn GetMousePosition() -> Vector2;
    pub(crate) fn GetMouseWheelMove() -> c_float;

    // Cursor related functions
    pub(crate) fn ShowCursor();
    pub(crate) fn HideCursor();
    pub(crate) fn IsCursorHidden() -> bool;

    // Misc. functions
    pub(crate) fn SetConfigFlags(flags: c_uint);

    // Texture loading functions
    pub(crate) fn LoadTexture(filename: *const i8) -> Texture2D;

    // Texture drawing functions
    pub(crate) fn DrawTexture(texture: Texture2D, posX: c_int, posY: c_int, tint: Color);
}
