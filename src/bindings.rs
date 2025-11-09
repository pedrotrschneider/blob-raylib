#![allow(non_snake_case)]

use crate::types::Color;
use crate::{Font, Image, NPatchInfo, Rectangle, RenderTexture2D, Texture2D, TextureCubemap, Vector2};
use std::ffi::c_uint;
use std::os::raw::{c_char, c_float, c_int, c_uchar, c_void};

unsafe extern "C" {
    // ---------------------------------------------------------------------------------
    // Window and Graphics Device Management
    // ---------------------------------------------------------------------------------
    pub(crate) fn InitWindow(width: c_int, height: c_int, title: *const c_char);
    pub(crate) fn WindowShouldClose() -> bool;
    pub(crate) fn CloseWindow();

    // ---------------------------------------------------------------------------------
    // Timing related functions
    // ---------------------------------------------------------------------------------
    pub(crate) fn SetTargetFPS(fps: c_int);
    pub(crate) fn GetFPS() -> c_int;
    pub(crate) fn GetFrameTime() -> c_float;

    // ---------------------------------------------------------------------------------
    // Drawing-related functions
    // ---------------------------------------------------------------------------------
    pub(crate) fn BeginDrawing();
    pub(crate) fn EndDrawing();
    pub(crate) fn ClearBackground(color: Color);
    pub(crate) fn DrawText(text: *const c_char, posX: c_int, posY: c_int, fontSize: c_int, color: Color);

    // ---------------------------------------------------------------------------------
    // Basic Shapes drawing functions
    // ---------------------------------------------------------------------------------
    pub(crate) fn DrawCircle(centerX: c_int, centerY: c_int, radius: c_float, color: Color);
    pub(crate) fn DrawCircleV(center: Vector2, radius: c_float, color: Color);
    pub(crate) fn DrawRectangle(posX: c_int, posY: c_int, width: c_int, height: c_int, color: Color);
    pub(crate) fn DrawRectangleRounded(rect: Rectangle, roundness: c_float, segments: c_int, color: Color);
    pub(crate) fn DrawTriangle(v1: Vector2, v2: Vector2, v3: Vector2, color: Color);

    // ---------------------------------------------------------------------------------
    // Input-related functions: keyboard
    // ---------------------------------------------------------------------------------
    pub(crate) fn IsKeyPressed(key: c_int) -> bool;
    pub(crate) fn IsKeyPressedRepeat(key: c_int) -> bool;
    pub(crate) fn IsKeyDown(key: c_int) -> bool;

    // ---------------------------------------------------------------------------------
    // Input-related functions: gamepads
    // ---------------------------------------------------------------------------------
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

    // ---------------------------------------------------------------------------------
    // Input-related functions: mouse
    // ---------------------------------------------------------------------------------
    pub(crate) fn IsMouseButtonPressed(key: c_int) -> bool;
    pub(crate) fn IsMouseButtonDown(key: c_int) -> bool;
    pub(crate) fn IsMouseButtonReleased(key: c_int) -> bool;
    pub(crate) fn IsMouseButtonUp(key: c_int) -> bool;
    pub(crate) fn GetMouseX() -> c_int;
    pub(crate) fn GetMouseY() -> c_int;
    pub(crate) fn GetMousePosition() -> Vector2;
    pub(crate) fn GetMouseWheelMove() -> c_float;

    // ---------------------------------------------------------------------------------
    // Cursor related functions
    // ---------------------------------------------------------------------------------
    pub(crate) fn ShowCursor();
    pub(crate) fn HideCursor();
    pub(crate) fn IsCursorHidden() -> bool;

    // ---------------------------------------------------------------------------------
    // Misc. functions
    // ---------------------------------------------------------------------------------
    pub(crate) fn SetConfigFlags(flags: c_uint);
    pub(crate) fn MemFree(ptr: *mut c_void);

    // ---------------------------------------------------------------------------------
    // Image loading functions
    // ---------------------------------------------------------------------------------
    pub(crate) fn LoadImage(fileName: *const c_char) -> Image;
    pub(crate) fn LoadImageRaw(fileName: *const c_char, width: c_int, height: c_int, format: c_int, headerSize: c_int) -> Image;
    pub(crate) fn LoadImageAnim(fileName: *const c_char, frames: *mut c_int) -> Image;
    pub(crate) fn LoadImageAnimFromMemory(fileType: *const c_char, fileData: *const c_uchar, dataSize: c_int, frames: *mut c_int) -> Image;
    pub(crate) fn LoadImageFromMemory(fileType: *const c_char, fileData: *const c_uchar, dataSize: c_int) -> Image;
    pub(crate) fn LoadImageFromTexture(texture: Texture2D) -> Image;
    pub(crate) fn LoadImageFromScreen() -> Image;
    pub(crate) fn IsImageValid(image: Image) -> bool;
    pub(crate) fn UnloadImage(image: Image);
    pub(crate) fn ExportImage(image: Image, fileName: *const c_char) -> bool;
    pub(crate) fn ExportImageToMemory(image: Image, fileType: *const c_char, fileSize: *mut c_int) -> *mut c_uchar;
    pub(crate) fn ExportImageAsCode(image: Image, fileName: *const c_char) -> bool;

    // ---------------------------------------------------------------------------------
    // Image generation functions
    // ---------------------------------------------------------------------------------
    pub(crate) fn GenImageColor(width: c_int, height: c_int, color: Color) -> Image;
    pub(crate) fn GenImageGradientLinear(width: c_int, height: c_int, direction: c_int, start: Color, end: Color) -> Image;
    pub(crate) fn GenImageGradientRadial(width: c_int, height: c_int, density: c_float, inner: Color, outer: Color) -> Image;
    pub(crate) fn GenImageGradientSquare(width: c_int, height: c_int, density: c_float, inner: Color, outer: Color) -> Image;
    pub(crate) fn GenImageChecked(width: c_int, height: c_int, checksX: c_int, checksY: c_int, col1: Color, col2: Color) -> Image;
    pub(crate) fn GenImageWhiteNoise(width: c_int, height: c_int, factor: c_float) -> Image;
    pub(crate) fn GenImagePerlinNoise(width: c_int, height: c_int, offsetX: c_int, offsetY: c_int, scale: c_float) -> Image;
    pub(crate) fn GenImageCellular(width: c_int, height: c_int, tileSize: c_int) -> Image;
    pub(crate) fn GenImageText(width: c_int, height: c_int, text: *const c_char) -> Image;

    // ---------------------------------------------------------------------------------
    // Image manipulation functions
    // ---------------------------------------------------------------------------------
    pub(crate) fn ImageCopy(image: Image) -> Image;
    pub(crate) fn ImageFromImage(image: Image, rec: Rectangle) -> Image;
    pub(crate) fn ImageFromChannel(image: Image, selectedChannel: c_int) -> Image;
    pub(crate) fn ImageText(text: *const c_char, fontSize: c_int, color: Color) -> Image;
    pub(crate) fn ImageTextEx(font: Font, text: *const c_char, fontSize: c_float, spacing: c_float, tint: Color) -> Image;
    pub(crate) fn ImageFormat(image: *mut Image, newFormat: c_int);
    pub(crate) fn ImageToPOT(image: *mut Image, fill: Color);
    pub(crate) fn ImageCrop(image: *mut Image, crop: Rectangle);
    pub(crate) fn ImageAlphaCrop(image: *mut Image, threshold: c_float);
    pub(crate) fn ImageAlphaClear(image: *mut Image, color: Color, threshold: c_float);
    pub(crate) fn ImageAlphaMask(image: *mut Image, alphaMask: Image);
    pub(crate) fn ImageAlphaPremultiply(image: *mut Image);
    pub(crate) fn ImageBlurGaussian(image: *mut Image, blurSize: c_int);
    pub(crate) fn ImageKernelConvolution(image: *mut Image, kernel: *const c_float, kernelSize: c_int);
    pub(crate) fn ImageResize(image: *mut Image, newWidth: c_int, newHeight: c_int);
    pub(crate) fn ImageResizeNN(image: *mut Image, newWidth: c_int, newHeight: c_int);
    pub(crate) fn ImageResizeCanvas(image: *mut Image, newWidth: c_int, newHeight: c_int, offsetX: c_int, offsetY: c_int, fill: Color);
    pub(crate) fn ImageMipmaps(image: *mut Image);
    pub(crate) fn ImageDither(image: *mut Image, rBpp: c_int, gBpp: c_int, bBpp: c_int, aBpp: c_int);
    pub(crate) fn ImageFlipVertical(image: *mut Image);
    pub(crate) fn ImageFlipHorizontal(image: *mut Image);
    pub(crate) fn ImageRotate(image: *mut Image, degrees: c_int);
    pub(crate) fn ImageRotateCW(image: *mut Image);
    pub(crate) fn ImageRotateCCW(image: *mut Image);
    pub(crate) fn ImageColorTint(image: *mut Image, color: Color);
    pub(crate) fn ImageColorInvert(image: *mut Image);
    pub(crate) fn ImageColorGrayscale(image: *mut Image);
    pub(crate) fn ImageColorContrast(image: *mut Image, contrast: c_float);
    pub(crate) fn ImageColorBrightness(image: *mut Image, brightness: c_int);
    pub(crate) fn ImageColorReplace(image: *mut Image, color: Color, replace: Color);
    pub(crate) fn LoadImageColors(image: Image) -> *mut Color;
    pub(crate) fn LoadImagePalette(image: Image, maxPaletteSize: c_int, colorCount: *mut c_int) -> *mut Color;
    pub(crate) fn UnloadImageColors(colors: *mut Color);
    pub(crate) fn UnloadImagePalette(colors: *mut Color);
    pub(crate) fn GetImageAlphaBorder(image: Image, threshold: c_float) -> Rectangle;
    pub(crate) fn GetImageColor(image: Image, x: c_int, y: c_int) -> Color;

    // ---------------------------------------------------------------------------------
    // Image drawing functions
    // ---------------------------------------------------------------------------------
    pub(crate) fn ImageClearBackground(dst: *mut Image, color: Color);
    pub(crate) fn ImageDrawPixel(dst: *mut Image, posX: c_int, posY: c_int, color: Color);
    pub(crate) fn ImageDrawPixelV(dst: *mut Image, position: Vector2, color: Color);
    pub(crate) fn ImageDrawLine(dst: *mut Image, startPosX: c_int, startPosY: c_int, endPosX: c_int, endPosY: c_int, color: Color);
    pub(crate) fn ImageDrawLineV(dst: *mut Image, start: Vector2, end: Vector2, color: Color);
    pub(crate) fn ImageDrawLineEx(dst: *mut Image, start: Vector2, end: Vector2, thick: c_int, color: Color);
    pub(crate) fn ImageDrawCircle(dst: *mut Image, centerX: c_int, centerY: c_int, radius: c_int, color: Color);
    pub(crate) fn ImageDrawCircleV(dst: *mut Image, center: Vector2, radius: c_int, color: Color);
    pub(crate) fn ImageDrawCircleLines(dst: *mut Image, centerX: c_int, centerY: c_int, radius: c_int, color: Color);
    pub(crate) fn ImageDrawCircleLinesV(dst: *mut Image, center: Vector2, radius: c_int, color: Color);
    pub(crate) fn ImageDrawRectangle(dst: *mut Image, posX: c_int, posY: c_int, width: c_int, height: c_int, color: Color);
    pub(crate) fn ImageDrawRectangleV(dst: *mut Image, position: Vector2, size: Vector2, color: Color);
    pub(crate) fn ImageDrawRectangleRec(dst: *mut Image, rec: Rectangle, color: Color);
    pub(crate) fn ImageDrawRectangleLines(dst: *mut Image, rec: Rectangle, thick: c_int, color: Color);
    pub(crate) fn ImageDrawTriangle(dst: *mut Image, v1: Vector2, v2: Vector2, v3: Vector2, color: Color);
    pub(crate) fn ImageDrawTriangleEx(dst: *mut Image, v1: Vector2, v2: Vector2, v3: Vector2, c1: Color, c2: Color, c3: Color);
    pub(crate) fn ImageDrawTriangleLines(dst: *mut Image, v1: Vector2, v2: Vector2, v3: Vector2, color: Color);
    pub(crate) fn ImageDrawTriangleFan(dst: *mut Image, points: *const Vector2, pointCount: c_int, color: Color);
    pub(crate) fn ImageDrawTriangleStrip(dst: *mut Image, points: *const Vector2, pointCount: c_int, color: Color);
    pub(crate) fn ImageDraw(dst: *mut Image, src: Image, srcRec: Rectangle, dstRec: Rectangle, tint: Color);
    pub(crate) fn ImageDrawText(dst: *mut Image, text: *const c_char, posX: c_int, posY: c_int, fontSize: c_int, color: Color);
    pub(crate) fn ImageDrawTextEx(dst: *mut Image, font: Font, text: *const c_char, position: Vector2, fontSize: c_float, spacing: c_float, tint: Color);

    // ---------------------------------------------------------------------------------
    // Texture loading functions
    // ---------------------------------------------------------------------------------
    pub(crate) fn LoadTexture(filename: *const i8) -> Texture2D;
    pub(crate) fn LoadTextureFromImage(image: Image) -> Texture2D;
    pub(crate) fn LoadTextureCubemap(image: Image, layout: c_int) -> TextureCubemap;
    pub(crate) fn LoadRenderTexture(width: c_int, height: c_int) -> RenderTexture2D;
    pub(crate) fn IsTextureValid(texture: Texture2D) -> bool;
    pub(crate) fn UnloadTexture(texture: Texture2D);
    pub(crate) fn IsRenderTextureValid(target: RenderTexture2D) -> bool;
    pub(crate) fn UnloadRenderTexture(target: RenderTexture2D);
    pub(crate) fn UpdateTexture(texture: Texture2D, pixels: *const c_void);
    pub(crate) fn UpdateTextureRec(texture: Texture2D, rec: Rectangle, pixels: *const c_void);

    // ---------------------------------------------------------------------------------
    // Texture configuration functions
    // ---------------------------------------------------------------------------------
    pub(crate) fn GenTextureMipmaps(texture: *mut Texture2D);
    pub(crate) fn SetTextureFilter(texture: Texture2D, filter: c_int);
    pub(crate) fn SetTextureWrap(texture: Texture2D, wrap: c_int);

    // ---------------------------------------------------------------------------------
    // Texture drawing functions
    // ---------------------------------------------------------------------------------
    pub(crate) fn DrawTexture(texture: Texture2D, posX: c_int, posY: c_int, tint: Color);
    pub(crate) fn DrawTextureV(texture: Texture2D, position: Vector2, tint: Color);
    pub(crate) fn DrawTextureEx(texture: Texture2D, position: Vector2, rotation: c_float, scale: c_float, tint: Color);
    pub(crate) fn DrawTextureRec(texture: Texture2D, source: Rectangle, position: Vector2, tint: Color);
    pub(crate) fn DrawTexturePro(texture: Texture2D, source: Rectangle, dest: Rectangle, origin: Vector2, rotation: c_float, tint: Color);
    pub(crate) fn DrawTextureNPatch(texture: Texture2D, nPatchInfo: NPatchInfo, dest: Rectangle, origin: Vector2, rotation: c_float, tint: Color);
}
