#![allow(non_snake_case)]

use crate::{AudioStream, AutomationEvent, AutomationEventList, BoundingBox, Camera, Camera2D, Camera3D, Color, FilePathList, Font, GlyphInfo, Image, Material, Matrix, Mesh, Model, ModelAnimation, Music, NPatchInfo, Ray, RayCollision, Rectangle, RenderTexture2D, Shader, Sound, Texture2D, TextureCubemap, Vector2, Vector3, Vector4, VrDeviceInfo, VrStereoConfig, Wave};
use std::ffi::{c_char, c_double, c_float, c_int, c_long, c_uchar, c_uint, c_void};

/// `va_list` from C. This is an opaque pointer.
pub type VaList = *mut c_void;

/// Logging: Redirect trace log messages
pub type TraceLogCallback = Option<unsafe extern "C" fn(log_level: c_int, text: *const c_char, args: VaList)>;

/// FileIO: Load binary data
pub type LoadFileDataCallback =
    Option<unsafe extern "C" fn(file_name: *const c_char, data_size: *mut c_int) -> *mut c_uchar>;

/// FileIO: Save binary data
pub type SaveFileDataCallback =
    Option<unsafe extern "C" fn(file_name: *const c_char, data: *mut c_void, data_size: c_int) -> bool>;

/// FileIO: Load text data
pub type LoadFileTextCallback = Option<unsafe extern "C" fn(file_name: *const c_char) -> *mut c_char>;

/// FileIO: Save text data
pub type SaveFileTextCallback = Option<unsafe extern "C" fn(file_name: *const c_char, text: *const c_char) -> bool>;

pub type AudioCallback = Option<unsafe extern "C" fn(buffer_data: *mut c_void, frames: u32)>;

unsafe extern "C" {
    //------------------------------------------------------------------------------------
    // Window and Graphics Device Functions (Module: core)
    //------------------------------------------------------------------------------------

    // Window-related functions
    pub(crate) fn InitWindow(width: c_int, height: c_int, title: *const c_char);
    pub(crate) fn WindowShouldClose() -> bool;
    pub(crate) fn CloseWindow();
    pub(crate) fn IsWindowReady() -> bool;
    pub(crate) fn IsWindowFullscreen() -> bool;
    pub(crate) fn IsWindowHidden() -> bool;
    pub(crate) fn IsWindowMinimized() -> bool;
    pub(crate) fn IsWindowMaximized() -> bool;
    pub(crate) fn IsWindowFocused() -> bool;
    pub(crate) fn IsWindowResized() -> bool;
    pub(crate) fn IsWindowState(flag: c_uint) -> bool;
    pub(crate) fn SetWindowState(flags: c_uint);
    pub(crate) fn ClearWindowState(flags: c_uint);
    pub(crate) fn ToggleFullscreen();
    pub(crate) fn ToggleBorderlessWindowed();
    pub(crate) fn MaximizeWindow();
    pub(crate) fn MinimizeWindow();
    pub(crate) fn RestoreWindow();
    pub(crate) fn SetWindowIcon(image: Image);
    pub(crate) fn SetWindowIcons(images: *mut Image, count: c_int);
    pub(crate) fn SetWindowTitle(title: *const c_char);
    pub(crate) fn SetWindowPosition(x: c_int, y: c_int);
    pub(crate) fn SetWindowMonitor(monitor: c_int);
    pub(crate) fn SetWindowMinSize(width: c_int, height: c_int);
    pub(crate) fn SetWindowMaxSize(width: c_int, height: c_int);
    pub(crate) fn SetWindowSize(width: c_int, height: c_int);
    pub(crate) fn SetWindowOpacity(opacity: c_float);
    pub(crate) fn SetWindowFocused();
    pub(crate) fn GetWindowHandle() -> *mut c_void;
    pub(crate) fn GetScreenWidth() -> c_int;
    pub(crate) fn GetScreenHeight() -> c_int;
    pub(crate) fn GetRenderWidth() -> c_int;
    pub(crate) fn GetRenderHeight() -> c_int;
    pub(crate) fn GetMonitorCount() -> c_int;
    pub(crate) fn GetCurrentMonitor() -> c_int;
    pub(crate) fn GetMonitorPosition(monitor: c_int) -> Vector2;
    pub(crate) fn GetMonitorWidth(monitor: c_int) -> c_int;
    pub(crate) fn GetMonitorHeight(monitor: c_int) -> c_int;
    pub(crate) fn GetMonitorPhysicalWidth(monitor: c_int) -> c_int;
    pub(crate) fn GetMonitorPhysicalHeight(monitor: c_int) -> c_int;
    pub(crate) fn GetMonitorRefreshRate(monitor: c_int) -> c_int;
    pub(crate) fn GetWindowPosition() -> Vector2;
    pub(crate) fn GetWindowScaleDPI() -> Vector2;
    pub(crate) fn GetMonitorName(monitor: c_int) -> *const c_char;
    pub(crate) fn SetClipboardText(text: *const c_char);
    pub(crate) fn GetClipboardText() -> *const c_char;
    pub(crate) fn GetClipboardImage() -> Image;
    pub(crate) fn EnableEventWaiting();
    pub(crate) fn DisableEventWaiting();

    // Cursor related functions
    pub(crate) fn ShowCursor();
    pub(crate) fn HideCursor();
    pub(crate) fn IsCursorHidden() -> bool;
    pub(crate) fn EnableCursor();
    pub(crate) fn DisableCursor();
    pub(crate) fn IsCursorOnScreen() -> bool;

    // Drawing-related functions
    pub(crate) fn ClearBackground(color: Color);
    pub(crate) fn BeginDrawing();
    pub(crate) fn EndDrawing();
    pub(crate) fn BeginMode2D(camera: Camera2D);
    pub(crate) fn EndMode2D();
    pub(crate) fn BeginMode3D(camera: Camera3D);
    pub(crate) fn EndMode3D();
    pub(crate) fn BeginTextureMode(target: RenderTexture2D);
    pub(crate) fn EndTextureMode();
    pub(crate) fn BeginShaderMode(shader: Shader);
    pub(crate) fn EndShaderMode();
    pub(crate) fn BeginBlendMode(mode: c_int);
    pub(crate) fn EndBlendMode();
    pub(crate) fn BeginScissorMode(x: c_int, y: c_int, width: c_int, height: c_int);
    pub(crate) fn EndScissorMode();
    pub(crate) fn BeginVrStereoMode(config: VrStereoConfig);
    pub(crate) fn EndVrStereoMode();

    // VR stereo config functions for VR simulator
    pub(crate) fn LoadVrStereoConfig(device: VrDeviceInfo) -> VrStereoConfig;
    pub(crate) fn UnloadVrStereoConfig(config: VrStereoConfig);

    // Shader management functions
    pub(crate) fn LoadShader(vsFileName: *const c_char, fsFileName: *const c_char) -> Shader;
    pub(crate) fn LoadShaderFromMemory(vsCode: *const c_char, fsCode: *const c_char) -> Shader;
    pub(crate) fn IsShaderValid(shader: Shader) -> bool;
    pub(crate) fn GetShaderLocation(shader: Shader, uniformName: *const c_char) -> c_int;
    pub(crate) fn GetShaderLocationAttrib(shader: Shader, attribName: *const c_char) -> c_int;
    pub(crate) fn SetShaderValue(shader: Shader, locIndex: c_int, value: *const c_void, uniformType: c_int);
    pub(crate) fn SetShaderValueV(
        shader: Shader,
        locIndex: c_int,
        value: *const c_void,
        uniformType: c_int,
        count: c_int,
    );
    pub(crate) fn SetShaderValueMatrix(shader: Shader, locIndex: c_int, mat: Matrix);
    pub(crate) fn SetShaderValueTexture(shader: Shader, locIndex: c_int, texture: Texture2D);
    pub(crate) fn UnloadShader(shader: Shader);

    // Screen-space-related functions
    pub(crate) fn GetScreenToWorldRay(position: Vector2, camera: Camera) -> Ray;
    pub(crate) fn GetScreenToWorldRayEx(position: Vector2, camera: Camera, width: c_int, height: c_int) -> Ray;
    pub(crate) fn GetWorldToScreen(position: Vector3, camera: Camera) -> Vector2;
    pub(crate) fn GetWorldToScreenEx(position: Vector3, camera: Camera, width: c_int, height: c_int) -> Vector2;
    pub(crate) fn GetWorldToScreen2D(position: Vector2, camera: Camera2D) -> Vector2;
    pub(crate) fn GetScreenToWorld2D(position: Vector2, camera: Camera2D) -> Vector2;
    pub(crate) fn GetCameraMatrix(camera: Camera) -> Matrix;
    pub(crate) fn GetCameraMatrix2D(camera: Camera2D) -> Matrix;

    // Timing related functions
    pub(crate) fn SetTargetFPS(fps: c_int);
    pub(crate) fn GetFrameTime() -> c_float;
    pub(crate) fn GetTime() -> c_double;
    pub(crate) fn GetFPS() -> c_int;

    // Custom frame control functions
    pub(crate) fn SwapScreenBuffer();
    pub(crate) fn PollInputEvents();
    pub(crate) fn WaitTime(seconds: c_double);

    // Random values generation functions
    pub(crate) fn SetRandomSeed(seed: c_uint);
    pub(crate) fn GetRandomValue(min: c_int, max: c_int) -> c_int;
    pub(crate) fn LoadRandomSequence(count: c_uint, min: c_int, max: c_int) -> *mut c_int;
    pub(crate) fn UnloadRandomSequence(sequence: *mut c_int);

    // Misc. functions
    pub(crate) fn TakeScreenshot(fileName: *const c_char);
    pub(crate) fn SetConfigFlags(flags: c_uint);
    pub(crate) fn OpenURL(url: *const c_char);

    // Utils
    pub(crate) fn TraceLog(logLevel: c_int, text: *const c_char, ...);
    pub(crate) fn SetTraceLogLevel(logLevel: c_int);
    pub(crate) fn MemAlloc(size: c_uint) -> *mut c_void;
    pub(crate) fn MemRealloc(ptr: *mut c_void, size: c_uint) -> *mut c_void;
    pub(crate) fn MemFree(ptr: *mut c_void);

    // Custom callbacks
    pub(crate) fn SetTraceLogCallback(callback: TraceLogCallback);
    pub(crate) fn SetLoadFileDataCallback(callback: LoadFileDataCallback);
    pub(crate) fn SetSaveFileDataCallback(callback: SaveFileDataCallback);
    pub(crate) fn SetLoadFileTextCallback(callback: LoadFileTextCallback);
    pub(crate) fn SetSaveFileTextCallback(callback: SaveFileTextCallback);

    // Files management functions
    pub(crate) fn LoadFileData(fileName: *const c_char, dataSize: *mut c_int) -> *mut c_uchar;
    pub(crate) fn UnloadFileData(data: *mut c_uchar);
    pub(crate) fn SaveFileData(fileName: *const c_char, data: *mut c_void, dataSize: c_int) -> bool;
    pub(crate) fn ExportDataAsCode(data: *const c_uchar, dataSize: c_int, fileName: *const c_char) -> bool;
    pub(crate) fn LoadFileText(fileName: *const c_char) -> *mut c_char;
    pub(crate) fn UnloadFileText(text: *mut c_char);
    pub(crate) fn SaveFileText(fileName: *const c_char, text: *const c_char) -> bool;

    // File system functions
    pub(crate) fn FileRename(fileName: *const c_char, fileRename: *const c_char) -> c_int;
    pub(crate) fn FileRemove(fileName: *const c_char) -> c_int;
    pub(crate) fn FileCopy(srcPath: *const c_char, dstPath: *const c_char) -> c_int;
    pub(crate) fn FileMove(srcPath: *const c_char, dstPath: *const c_char) -> c_int;
    pub(crate) fn FileTextReplace(fileName: *const c_char, search: *const c_char, replacement: *const c_char) -> c_int;
    pub(crate) fn FileTextFindIndex(fileName: *const c_char, search: *const c_char) -> c_int;
    pub(crate) fn FileExists(fileName: *const c_char) -> bool;
    pub(crate) fn DirectoryExists(dirPath: *const c_char) -> bool;
    pub(crate) fn IsFileExtension(fileName: *const c_char, ext: *const c_char) -> bool;
    pub(crate) fn GetFileLength(fileName: *const c_char) -> c_int;
    pub(crate) fn GetFileModTime(fileName: *const c_char) -> c_long;
    pub(crate) fn GetFileExtension(fileName: *const c_char) -> *const c_char;
    pub(crate) fn GetFileName(filePath: *const c_char) -> *const c_char;
    pub(crate) fn GetFileNameWithoutExt(filePath: *const c_char) -> *const c_char;
    pub(crate) fn GetDirectoryPath(filePath: *const c_char) -> *const c_char;
    pub(crate) fn GetPrevDirectoryPath(dirPath: *const c_char) -> *const c_char;
    pub(crate) fn GetWorkingDirectory() -> *const c_char;
    pub(crate) fn GetApplicationDirectory() -> *const c_char;
    pub(crate) fn MakeDirectory(dirPath: *const c_char) -> c_int;
    pub(crate) fn ChangeDirectory(dir: *const c_char) -> bool;
    pub(crate) fn IsPathFile(path: *const c_char) -> bool;
    pub(crate) fn IsFileNameValid(fileName: *const c_char) -> bool;
    pub(crate) fn LoadDirectoryFiles(dirPath: *const c_char) -> FilePathList;
    pub(crate) fn LoadDirectoryFilesEx(
        basePath: *const c_char,
        filter: *const c_char,
        scanSubdirs: bool,
    ) -> FilePathList;
    pub(crate) fn UnloadDirectoryFiles(files: FilePathList);
    pub(crate) fn IsFileDropped() -> bool;
    pub(crate) fn LoadDroppedFiles() -> FilePathList;
    pub(crate) fn UnloadDroppedFiles(files: FilePathList);

    // Compression/Encoding functionality
    pub(crate) fn CompressData(data: *const c_uchar, dataSize: c_int, compDataSize: *mut c_int) -> *mut c_uchar;
    pub(crate) fn DecompressData(compData: *const c_uchar, compDataSize: c_int, dataSize: *mut c_int) -> *mut c_uchar;
    pub(crate) fn EncodeDataBase64(data: *const c_uchar, dataSize: c_int, outputSize: *mut c_int) -> *mut c_char;
    pub(crate) fn DecodeDataBase64(text: *const c_char, outputSize: *mut c_int) -> *mut c_uchar;
    pub(crate) fn ComputeCRC32(data: *mut c_uchar, dataSize: c_int) -> c_uint;
    pub(crate) fn ComputeMD5(data: *mut c_uchar, dataSize: c_int) -> *mut c_uint;
    pub(crate) fn ComputeSHA1(data: *mut c_uchar, dataSize: c_int) -> *mut c_uint;
    pub(crate) fn ComputeSHA256(data: *mut c_uchar, dataSize: c_int) -> *mut c_uint;

    // Automation events functionality
    pub(crate) fn LoadAutomationEventList(fileName: *const c_char) -> AutomationEventList;
    pub(crate) fn UnloadAutomationEventList(list: AutomationEventList);
    pub(crate) fn ExportAutomationEventList(list: AutomationEventList, fileName: *const c_char) -> bool;
    pub(crate) fn SetAutomationEventList(list: *mut AutomationEventList);
    pub(crate) fn SetAutomationEventBaseFrame(frame: c_int);
    pub(crate) fn StartAutomationEventRecording();
    pub(crate) fn StopAutomationEventRecording();
    pub(crate) fn PlayAutomationEvent(event: AutomationEvent);

    //------------------------------------------------------------------------------------
    // Input Handling Functions (Module: core)
    //------------------------------------------------------------------------------------

    // Input-related functions: keyboard
    pub(crate) fn IsKeyPressed(key: c_int) -> bool;
    pub(crate) fn IsKeyPressedRepeat(key: c_int) -> bool;
    pub(crate) fn IsKeyDown(key: c_int) -> bool;
    pub(crate) fn IsKeyReleased(key: c_int) -> bool;
    pub(crate) fn IsKeyUp(key: c_int) -> bool;
    pub(crate) fn GetKeyPressed() -> c_int;
    pub(crate) fn GetCharPressed() -> c_int;
    pub(crate) fn GetKeyName(key: c_int) -> *const c_char;
    pub(crate) fn SetExitKey(key: c_int);

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
    pub(crate) fn GetMouseDelta() -> Vector2;
    pub(crate) fn SetMousePosition(x: c_int, y: c_int);
    pub(crate) fn SetMouseOffset(offsetX: c_int, offsetY: c_int);
    pub(crate) fn SetMouseScale(scaleX: c_float, scaleY: c_float);
    pub(crate) fn GetMouseWheelMoveV() -> Vector2;
    pub(crate) fn SetMouseCursor(cursor: c_int);

    // Input-related functions: touch
    pub(crate) fn GetTouchX() -> c_int;
    pub(crate) fn GetTouchY() -> c_int;
    pub(crate) fn GetTouchPosition(index: c_int) -> Vector2;
    pub(crate) fn GetTouchPointId(index: c_int) -> c_int;
    pub(crate) fn GetTouchPointCount() -> c_int;

    //------------------------------------------------------------------------------------
    // Gestures and Touch Handling Functions (Module: rgestures)
    //------------------------------------------------------------------------------------

    // Gestures and Touch Handling Functions
    pub(crate) fn SetGesturesEnabled(flags: c_uint);
    pub(crate) fn IsGestureDetected(gesture: c_uint) -> bool;
    pub(crate) fn GetGestureDetected() -> c_int;
    pub(crate) fn GetGestureHoldDuration() -> c_float;
    pub(crate) fn GetGestureDragVector() -> Vector2;
    pub(crate) fn GetGestureDragAngle() -> c_float;
    pub(crate) fn GetGesturePinchVector() -> Vector2;
    pub(crate) fn GetGesturePinchAngle() -> c_float;

    //------------------------------------------------------------------------------------
    // Camera System Functions (Module: rcamera)
    //------------------------------------------------------------------------------------
    pub(crate) fn UpdateCamera(camera: *mut Camera, mode: c_int);
    pub(crate) fn UpdateCameraPro(
        camera: *mut Camera,
        movement: Vector3,
        rotation: Vector3,
        zoom: c_float,
    );

    //------------------------------------------------------------------------------------
    // Basic Shapes Drawing Functions (Module: shapes)
    //------------------------------------------------------------------------------------
    pub(crate) fn SetShapesTexture(texture: Texture2D, source: Rectangle);
    pub(crate) fn GetShapesTexture() -> Texture2D;
    pub(crate) fn GetShapesTextureRectangle() -> Rectangle;

    // Basic shapes drawing functions
    pub(crate) fn DrawPixel(posX: c_int, posY: c_int, color: Color);
    pub(crate) fn DrawPixelV(position: Vector2, color: Color);
    pub(crate) fn DrawLine(
        startPosX: c_int,
        startPosY: c_int,
        endPosX: c_int,
        endPosY: c_int,
        color: Color,
    );
    pub(crate) fn DrawLineV(startPos: Vector2, endPos: Vector2, color: Color);
    pub(crate) fn DrawLineEx(startPos: Vector2, endPos: Vector2, thick: c_float, color: Color);
    pub(crate) fn DrawLineStrip(points: *const Vector2, pointCount: c_int, color: Color);
    pub(crate) fn DrawLineBezier(startPos: Vector2, endPos: Vector2, thick: c_float, color: Color);
    pub(crate) fn DrawLineDashed(
        startPos: Vector2,
        endPos: Vector2,
        dashSize: c_int,
        spaceSize: c_int,
        color: Color,
    );
    pub(crate) fn DrawCircle(centerX: c_int, centerY: c_int, radius: c_float, color: Color);
    pub(crate) fn DrawCircleSector(
        center: Vector2,
        radius: c_float,
        startAngle: c_float,
        endAngle: c_float,
        segments: c_int,
        color: Color,
    );
    pub(crate) fn DrawCircleSectorLines(
        center: Vector2,
        radius: c_float,
        startAngle: c_float,
        endAngle: c_float,
        segments: c_int,
        color: Color,
    );
    pub(crate) fn DrawCircleGradient(
        centerX: c_int,
        centerY: c_int,
        radius: c_float,
        color1: Color,
        color2: Color,
    );
    pub(crate) fn DrawCircleV(center: Vector2, radius: c_float, color: Color);
    pub(crate) fn DrawCircleLines(centerX: c_int, centerY: c_int, radius: c_float, color: Color);
    pub(crate) fn DrawCircleLinesV(center: Vector2, radius: c_float, color: Color);
    pub(crate) fn DrawEllipse(
        centerX: c_int,
        centerY: c_int,
        radiusH: c_float,
        radiusV: c_float,
        color: Color,
    );
    pub(crate) fn DrawEllipseV(center: Vector2, radiusH: c_float, radiusV: c_float, color: Color);
    pub(crate) fn DrawEllipseLines(
        centerX: c_int,
        centerY: c_int,
        radiusH: c_float,
        radiusV: c_float,
        color: Color,
    );
    pub(crate) fn DrawEllipseLinesV(
        center: Vector2,
        radiusH: c_float,
        radiusV: c_float,
        color: Color,
    );
    pub(crate) fn DrawRing(
        center: Vector2,
        innerRadius: c_float,
        outerRadius: c_float,
        startAngle: c_float,
        endAngle: c_float,
        segments: c_int,
        color: Color,
    );
    pub(crate) fn DrawRingLines(
        center: Vector2,
        innerRadius: c_float,
        outerRadius: c_float,
        startAngle: c_float,
        endAngle: c_float,
        segments: c_int,
        color: Color,
    );
    pub(crate) fn DrawRectangle(
        posX: c_int,
        posY: c_int,
        width: c_int,
        height: c_int,
        color: Color,
    );
    pub(crate) fn DrawRectangleV(position: Vector2, size: Vector2, color: Color);
    pub(crate) fn DrawRectangleRec(rec: Rectangle, color: Color);
    pub(crate) fn DrawRectanglePro(
        rec: Rectangle,
        origin: Vector2,
        rotation: c_float,
        color: Color,
    );
    pub(crate) fn DrawRectangleGradientV(
        posX: c_int,
        posY: c_int,
        width: c_int,
        height: c_int,
        color1: Color,
        color2: Color,
    );
    pub(crate) fn DrawRectangleGradientH(
        posX: c_int,
        posY: c_int,
        width: c_int,
        height: c_int,
        color1: Color,
        color2: Color,
    );
    pub(crate) fn DrawRectangleGradientEx(
        rec: Rectangle,
        col1: Color,
        col2: Color,
        col3: Color,
        col4: Color,
    );
    pub(crate) fn DrawRectangleLines(
        posX: c_int,
        posY: c_int,
        width: c_int,
        height: c_int,
        color: Color,
    );
    pub(crate) fn DrawRectangleLinesEx(rec: Rectangle, lineThick: c_float, color: Color);
    pub(crate) fn DrawRectangleRounded(
        rec: Rectangle,
        roundness: c_float,
        segments: c_int,
        color: Color,
    );
    pub(crate) fn DrawRectangleRoundedLines(
        rec: Rectangle,
        roundness: c_float,
        segments: c_int,
        color: Color,
    );
    pub(crate) fn DrawRectangleRoundedLinesEx(
        rec: Rectangle,
        roundness: c_float,
        segments: c_int,
        lineThick: c_float,
        color: Color,
    );
    pub(crate) fn DrawTriangle(v1: Vector2, v2: Vector2, v3: Vector2, color: Color);
    pub(crate) fn DrawTriangleLines(v1: Vector2, v2: Vector2, v3: Vector2, color: Color);
    pub(crate) fn DrawTriangleFan(points: *const Vector2, pointCount: c_int, color: Color);
    pub(crate) fn DrawTriangleStrip(points: *const Vector2, pointCount: c_int, color: Color);
    pub(crate) fn DrawPoly(
        center: Vector2,
        sides: c_int,
        radius: c_float,
        rotation: c_float,
        color: Color,
    );
    pub(crate) fn DrawPolyLines(
        center: Vector2,
        sides: c_int,
        radius: c_float,
        rotation: c_float,
        color: Color,
    );
    pub(crate) fn DrawPolyLinesEx(
        center: Vector2,
        sides: c_int,
        radius: c_float,
        rotation: c_float,
        lineThick: c_float,
        color: Color,
    );

    // Splines drawing functions
    pub(crate) fn DrawSplineLinear(
        points: *const Vector2,
        pointCount: c_int,
        thick: c_float,
        color: Color,
    );
    pub(crate) fn DrawSplineBasis(
        points: *const Vector2,
        pointCount: c_int,
        thick: c_float,
        color: Color,
    );
    pub(crate) fn DrawSplineCatmullRom(
        points: *const Vector2,
        pointCount: c_int,
        thick: c_float,
        color: Color,
    );
    pub(crate) fn DrawSplineBezierQuadratic(
        points: *const Vector2,
        pointCount: c_int,
        thick: c_float,
        color: Color,
    );
    pub(crate) fn DrawSplineBezierCubic(
        points: *const Vector2,
        pointCount: c_int,
        thick: c_float,
        color: Color,
    );
    pub(crate) fn DrawSplineSegmentLinear(p1: Vector2, p2: Vector2, thick: c_float, color: Color);
    pub(crate) fn DrawSplineSegmentBasis(
        p1: Vector2,
        p2: Vector2,
        p3: Vector2,
        p4: Vector2,
        thick: c_float,
        color: Color,
    );
    pub(crate) fn DrawSplineSegmentCatmullRom(
        p1: Vector2,
        p2: Vector2,
        p3: Vector2,
        p4: Vector2,
        thick: c_float,
        color: Color,
    );
    pub(crate) fn DrawSplineSegmentBezierQuadratic(
        p1: Vector2,
        c2: Vector2,
        p3: Vector2,
        thick: c_float,
        color: Color,
    );
    pub(crate) fn DrawSplineSegmentBezierCubic(
        p1: Vector2,
        c2: Vector2,
        c3: Vector2,
        p4: Vector2,
        thick: c_float,
        color: Color,
    );

    // Spline segment point evaluation functions
    pub(crate) fn GetSplinePointLinear(startPos: Vector2, endPos: Vector2, t: c_float) -> Vector2;
    pub(crate) fn GetSplinePointBasis(
        p1: Vector2,
        p2: Vector2,
        p3: Vector2,
        p4: Vector2,
        t: c_float,
    ) -> Vector2;
    pub(crate) fn GetSplinePointCatmullRom(
        p1: Vector2,
        p2: Vector2,
        p3: Vector2,
        p4: Vector2,
        t: c_float,
    ) -> Vector2;
    pub(crate) fn GetSplinePointBezierQuad(
        p1: Vector2,
        c2: Vector2,
        p3: Vector2,
        t: c_float,
    ) -> Vector2;
    pub(crate) fn GetSplinePointBezierCubic(
        p1: Vector2,
        c2: Vector2,
        c3: Vector2,
        p4: Vector2,
        t: c_float,
    ) -> Vector2;

    // Basic shapes collision detection functions
    pub(crate) fn CheckCollisionRecs(rec1: Rectangle, rec2: Rectangle) -> bool;
    pub(crate) fn CheckCollisionCircles(
        center1: Vector2,
        radius1: c_float,
        center2: Vector2,
        radius2: c_float,
    ) -> bool;
    pub(crate) fn CheckCollisionCircleRec(center: Vector2, radius: c_float, rec: Rectangle) -> bool;
    pub(crate) fn CheckCollisionCircleLine(
        center: Vector2,
        radius: c_float,
        p1: Vector2,
        p2: Vector2,
    ) -> bool;
    pub(crate) fn CheckCollisionPointRec(point: Vector2, rec: Rectangle) -> bool;
    pub(crate) fn CheckCollisionPointCircle(
        point: Vector2,
        center: Vector2,
        radius: c_float,
    ) -> bool;
    pub(crate) fn CheckCollisionPointTriangle(point: Vector2, p1: Vector2, p2: Vector2, p3: Vector2)
                                              -> bool;
    pub(crate) fn CheckCollisionPointLine(
        point: Vector2,
        p1: Vector2,
        p2: Vector2,
        threshold: c_int,
    ) -> bool;
    pub(crate) fn CheckCollisionPointPoly(
        point: Vector2,
        points: *const Vector2,
        pointCount: c_int,
    ) -> bool;
    pub(crate) fn CheckCollisionLines(
        startPos1: Vector2,
        endPos1: Vector2,
        startPos2: Vector2,
        endPos2: Vector2,
        collisionPoint: *mut Vector2,
    ) -> bool;
    pub(crate) fn GetCollisionRec(rec1: Rectangle, rec2: Rectangle) -> Rectangle;

    //------------------------------------------------------------------------------------
    // Texture Loading and Drawing Functions (Module: textures)
    //------------------------------------------------------------------------------------

    // Image loading functions
    pub(crate) fn LoadImage(fileName: *const c_char) -> Image;
    pub(crate) fn LoadImageRaw(
        fileName: *const c_char,
        width: c_int,
        height: c_int,
        format: c_int,
        headerSize: c_int,
    ) -> Image;
    pub(crate) fn LoadImageAnim(fileName: *const c_char, frames: *mut c_int) -> Image;
    pub(crate) fn LoadImageAnimFromMemory(
        fileType: *const c_char,
        fileData: *const c_uchar,
        dataSize: c_int,
        frames: *mut c_int,
    ) -> Image;
    pub(crate) fn LoadImageFromMemory(fileType: *const c_char, fileData: *const c_uchar, dataSize: c_int) -> Image;
    pub(crate) fn LoadImageFromTexture(texture: Texture2D) -> Image;
    pub(crate) fn LoadImageFromScreen() -> Image;
    pub(crate) fn IsImageValid(image: Image) -> bool;
    pub(crate) fn UnloadImage(image: Image);
    pub(crate) fn ExportImage(image: Image, fileName: *const c_char) -> bool;
    pub(crate) fn ExportImageToMemory(image: Image, fileType: *const c_char, fileSize: *mut c_int) -> *mut c_uchar;
    pub(crate) fn ExportImageAsCode(image: Image, fileName: *const c_char) -> bool;

    // Image generation functions
    pub(crate) fn GenImageColor(width: c_int, height: c_int, color: Color) -> Image;
    pub(crate) fn GenImageGradientLinear(
        width: c_int,
        height: c_int,
        direction: c_int,
        start: Color,
        end: Color,
    ) -> Image;
    pub(crate) fn GenImageGradientRadial(
        width: c_int,
        height: c_int,
        density: c_float,
        inner: Color,
        outer: Color,
    ) -> Image;
    pub(crate) fn GenImageGradientSquare(
        width: c_int,
        height: c_int,
        density: c_float,
        inner: Color,
        outer: Color,
    ) -> Image;
    pub(crate) fn GenImageChecked(
        width: c_int,
        height: c_int,
        checksX: c_int,
        checksY: c_int,
        col1: Color,
        col2: Color,
    ) -> Image;
    pub(crate) fn GenImageWhiteNoise(width: c_int, height: c_int, factor: c_float) -> Image;
    pub(crate) fn GenImagePerlinNoise(
        width: c_int,
        height: c_int,
        offsetX: c_int,
        offsetY: c_int,
        scale: c_float,
    ) -> Image;
    pub(crate) fn GenImageCellular(width: c_int, height: c_int, tileSize: c_int) -> Image;
    pub(crate) fn GenImageText(width: c_int, height: c_int, text: *const c_char) -> Image;

    // Image manipulation functions
    pub(crate) fn ImageCopy(image: Image) -> Image;
    pub(crate) fn ImageFromImage(image: Image, rec: Rectangle) -> Image;
    pub(crate) fn ImageFromChannel(image: Image, selectedChannel: c_int) -> Image;
    pub(crate) fn ImageText(text: *const c_char, fontSize: c_int, color: Color) -> Image;
    pub(crate) fn ImageTextEx(
        font: Font,
        text: *const c_char,
        fontSize: c_float,
        spacing: c_float,
        tint: Color,
    ) -> Image;
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
    pub(crate) fn ImageResizeCanvas(
        image: *mut Image,
        newWidth: c_int,
        newHeight: c_int,
        offsetX: c_int,
        offsetY: c_int,
        fill: Color,
    );
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

    // Image drawing functions
    pub(crate) fn ImageClearBackground(dst: *mut Image, color: Color);
    pub(crate) fn ImageDrawPixel(dst: *mut Image, posX: c_int, posY: c_int, color: Color);
    pub(crate) fn ImageDrawPixelV(dst: *mut Image, position: Vector2, color: Color);
    pub(crate) fn ImageDrawLine(
        dst: *mut Image,
        startPosX: c_int,
        startPosY: c_int,
        endPosX: c_int,
        endPosY: c_int,
        color: Color,
    );
    pub(crate) fn ImageDrawLineV(dst: *mut Image, start: Vector2, end: Vector2, color: Color);
    pub(crate) fn ImageDrawLineEx(dst: *mut Image, start: Vector2, end: Vector2, thick: c_int, color: Color);
    pub(crate) fn ImageDrawCircle(dst: *mut Image, centerX: c_int, centerY: c_int, radius: c_int, color: Color);
    pub(crate) fn ImageDrawCircleV(dst: *mut Image, center: Vector2, radius: c_int, color: Color);
    pub(crate) fn ImageDrawCircleLines(dst: *mut Image, centerX: c_int, centerY: c_int, radius: c_int, color: Color);
    pub(crate) fn ImageDrawCircleLinesV(dst: *mut Image, center: Vector2, radius: c_int, color: Color);
    pub(crate) fn ImageDrawRectangle(
        dst: *mut Image,
        posX: c_int,
        posY: c_int,
        width: c_int,
        height: c_int,
        color: Color,
    );
    pub(crate) fn ImageDrawRectangleV(dst: *mut Image, position: Vector2, size: Vector2, color: Color);
    pub(crate) fn ImageDrawRectangleRec(dst: *mut Image, rec: Rectangle, color: Color);
    pub(crate) fn ImageDrawRectangleLines(dst: *mut Image, rec: Rectangle, thick: c_int, color: Color);
    pub(crate) fn ImageDrawTriangle(dst: *mut Image, v1: Vector2, v2: Vector2, v3: Vector2, color: Color);
    pub(crate) fn ImageDrawTriangleEx(
        dst: *mut Image,
        v1: Vector2,
        v2: Vector2,
        v3: Vector2,
        c1: Color,
        c2: Color,
        c3: Color,
    );
    pub(crate) fn ImageDrawTriangleLines(dst: *mut Image, v1: Vector2, v2: Vector2, v3: Vector2, color: Color);
    pub(crate) fn ImageDrawTriangleFan(dst: *mut Image, points: *const Vector2, pointCount: c_int, color: Color);
    pub(crate) fn ImageDrawTriangleStrip(dst: *mut Image, points: *const Vector2, pointCount: c_int, color: Color);
    pub(crate) fn ImageDraw(dst: *mut Image, src: Image, srcRec: Rectangle, dstRec: Rectangle, tint: Color);
    pub(crate) fn ImageDrawText(
        dst: *mut Image,
        text: *const c_char,
        posX: c_int,
        posY: c_int,
        fontSize: c_int,
        color: Color,
    );
    pub(crate) fn ImageDrawTextEx(
        dst: *mut Image,
        font: Font,
        text: *const c_char,
        position: Vector2,
        fontSize: c_float,
        spacing: c_float,
        tint: Color,
    );

    // Texture loading functions
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

    // Texture configuration functions
    pub(crate) fn GenTextureMipmaps(texture: *mut Texture2D);
    pub(crate) fn SetTextureFilter(texture: Texture2D, filter: c_int);
    pub(crate) fn SetTextureWrap(texture: Texture2D, wrap: c_int);

    // Texture drawing functions
    pub(crate) fn DrawTexture(texture: Texture2D, posX: c_int, posY: c_int, tint: Color);
    pub(crate) fn DrawTextureV(texture: Texture2D, position: Vector2, tint: Color);
    pub(crate) fn DrawTextureEx(texture: Texture2D, position: Vector2, rotation: c_float, scale: c_float, tint: Color);
    pub(crate) fn DrawTextureRec(texture: Texture2D, source: Rectangle, position: Vector2, tint: Color);
    pub(crate) fn DrawTexturePro(
        texture: Texture2D,
        source: Rectangle,
        dest: Rectangle,
        origin: Vector2,
        rotation: c_float,
        tint: Color,
    );
    pub(crate) fn DrawTextureNPatch(
        texture: Texture2D,
        nPatchInfo: NPatchInfo,
        dest: Rectangle,
        origin: Vector2,
        rotation: c_float,
        tint: Color,
    );

    // Color/pixel related functions
    pub(crate) fn ColorIsEqual(col1: Color, col2: Color) -> bool;
    pub(crate) fn Fade(color: Color, alpha: c_float) -> Color;
    pub(crate) fn ColorToInt(color: Color) -> c_int;
    pub(crate) fn ColorNormalize(color: Color) -> Vector4;
    pub(crate) fn ColorFromNormalized(normalized: Vector4) -> Color;
    pub(crate) fn ColorToHSV(color: Color) -> Vector3;
    pub(crate) fn ColorFromHSV(hue: c_float, saturation: c_float, value: c_float) -> Color;
    pub(crate) fn ColorTint(color: Color, tint: Color) -> Color;
    pub(crate) fn ColorBrightness(color: Color, factor: c_float) -> Color;
    pub(crate) fn ColorContrast(color: Color, contrast: c_float) -> Color;
    pub(crate) fn ColorAlpha(color: Color, alpha: c_float) -> Color;
    pub(crate) fn ColorAlphaBlend(dst: Color, src: Color, tint: Color) -> Color;
    pub(crate) fn ColorLerp(color1: Color, color2: Color, factor: c_float) -> Color;
    pub(crate) fn GetColor(hexValue: c_uint) -> Color;
    pub(crate) fn GetPixelColor(srcPtr: *mut c_void, format: c_int) -> Color;
    pub(crate) fn SetPixelColor(dstPtr: *mut c_void, color: Color, format: c_int);
    pub(crate) fn GetPixelDataSize(width: c_int, height: c_int, format: c_int) -> c_int;

    //------------------------------------------------------------------------------------
    // Font Loading and Text Drawing Functions (Module: text)
    //------------------------------------------------------------------------------------

    // Font loading/unloading functions
    pub(crate) fn GetFontDefault() -> Font;
    pub(crate) fn LoadFont(fileName: *const c_char) -> Font;
    pub(crate) fn LoadFontEx(
        fileName: *const c_char,
        fontSize: c_int,
        codepoints: *const c_int,
        codepointCount: c_int,
    ) -> Font;
    pub(crate) fn LoadFontFromImage(image: Image, key: Color, firstChar: c_int) -> Font;
    pub(crate) fn LoadFontFromMemory(
        fileType: *const c_char,
        fileData: *const c_uchar,
        dataSize: c_int,
        fontSize: c_int,
        codepoints: *const c_int,
        codepointCount: c_int,
    ) -> Font;
    pub(crate) fn IsFontValid(font: Font) -> bool;
    pub(crate) fn LoadFontData(
        fileData: *const c_uchar,
        dataSize: c_int,
        fontSize: c_int,
        codepoints: *const c_int,
        codepointCount: c_int,
        type_: c_int,
        glyphCount: *mut c_int,
    ) -> *mut GlyphInfo;
    pub(crate) fn GenImageFontAtlas(
        glyphs: *const GlyphInfo,
        glyphRecs: *mut *mut Rectangle,
        glyphCount: c_int,
        fontSize: c_int,
        padding: c_int,
        packMethod: c_int,
    ) -> Image;
    pub(crate) fn UnloadFontData(glyphs: *mut GlyphInfo, glyphCount: c_int);
    pub(crate) fn UnloadFont(font: Font);
    pub(crate) fn ExportFontAsCode(font: Font, fileName: *const c_char) -> bool;

    // Text drawing functions
    pub(crate) fn DrawFPS(posX: c_int, posY: c_int);
    pub(crate) fn DrawText(text: *const c_char, posX: c_int, posY: c_int, fontSize: c_int, color: Color);
    pub(crate) fn DrawTextEx(
        font: Font,
        text: *const c_char,
        position: Vector2,
        fontSize: c_float,
        spacing: c_float,
        tint: Color,
    );
    pub(crate) fn DrawTextPro(
        font: Font,
        text: *const c_char,
        position: Vector2,
        origin: Vector2,
        rotation: c_float,
        fontSize: c_float,
        spacing: c_float,
        tint: Color,
    );
    pub(crate) fn DrawTextCodepoint(
        font: Font,
        codepoint: c_int,
        position: Vector2,
        fontSize: c_float,
        tint: Color,
    );
    pub(crate) fn DrawTextCodepoints(
        font: Font,
        codepoints: *const c_int,
        codepointCount: c_int,
        position: Vector2,
        fontSize: c_float,
        spacing: c_float,
        tint: Color,
    );

    // Text font info functions
    pub(crate) fn SetTextLineSpacing(spacing: c_int);
    pub(crate) fn MeasureText(text: *const c_char, fontSize: c_int) -> c_int;
    pub(crate) fn MeasureTextEx(
        font: Font,
        text: *const c_char,
        fontSize: c_float,
        spacing: c_float,
    ) -> Vector2;
    pub(crate) fn GetGlyphIndex(font: Font, codepoint: c_int) -> c_int;
    pub(crate) fn GetGlyphInfo(font: Font, codepoint: c_int) -> GlyphInfo;
    pub(crate) fn GetGlyphAtlasRec(font: Font, codepoint: c_int) -> Rectangle;

    // Text codepoints management functions
    pub(crate) fn LoadUTF8(codepoints: *const c_int, length: c_int) -> *mut c_char;
    pub(crate) fn UnloadUTF8(text: *mut c_char);
    pub(crate) fn LoadCodepoints(text: *const c_char, count: *mut c_int) -> *mut c_int;
    pub(crate) fn UnloadCodepoints(codepoints: *mut c_int);
    pub(crate) fn GetCodepointCount(text: *const c_char) -> c_int;
    pub(crate) fn GetCodepoint(text: *const c_char, codepointSize: *mut c_int) -> c_int;
    pub(crate) fn GetCodepointNext(text: *const c_char, codepointSize: *mut c_int) -> c_int;
    pub(crate) fn GetCodepointPrevious(text: *const c_char, codepointSize: *mut c_int) -> c_int;
    pub(crate) fn CodepointToUTF8(codepoint: c_int, utf8Size: *mut c_int) -> *const c_char;

    // Text strings management functions
    pub(crate) fn LoadTextLines(text: *const c_char, count: *mut c_int) -> *mut *mut c_char;
    pub(crate) fn UnloadTextLines(text: *mut *mut c_char, lineCount: c_int);
    pub(crate) fn TextCopy(dst: *mut c_char, src: *const c_char) -> c_int;
    pub(crate) fn TextIsEqual(text1: *const c_char, text2: *const c_char) -> bool;
    pub(crate) fn TextLength(text: *const c_char) -> c_uint;
    // pub(crate) fn TextFormat(text: *const c_char, ...) -> *const c_char;
    pub(crate) fn TextSubtext(text: *const c_char, position: c_int, length: c_int)
                              -> *const c_char;
    pub(crate) fn TextRemoveSpaces(text: *const c_char) -> *const c_char;
    pub(crate) fn GetTextBetween(
        text: *const c_char,
        begin: *const c_char,
        end: *const c_char,
    ) -> *mut c_char;
    pub(crate) fn TextReplace(
        text: *const c_char,
        search: *const c_char,
        replacement: *const c_char,
    ) -> *mut c_char;
    pub(crate) fn TextReplaceBetween(
        text: *const c_char,
        begin: *const c_char,
        end: *const c_char,
        replacement: *const c_char,
    ) -> *mut c_char;
    pub(crate) fn TextInsert(
        text: *const c_char,
        insert: *const c_char,
        position: c_int,
    ) -> *mut c_char;
    pub(crate) fn TextJoin(
        textList: *const *const c_char,
        count: c_int,
        delimiter: *const c_char,
    ) -> *mut c_char;
    pub(crate) fn TextSplit(
        text: *const c_char,
        delimiter: c_char,
        count: *mut c_int,
    ) -> *mut *mut c_char;
    pub(crate) fn TextAppend(text: *mut c_char, append: *const c_char, position: *mut c_int);
    pub(crate) fn TextFindIndex(text: *const c_char, search: *const c_char) -> c_int;
    pub(crate) fn TextToUpper(text: *const c_char) -> *mut c_char;
    pub(crate) fn TextToLower(text: *const c_char) -> *mut c_char;
    pub(crate) fn TextToPascal(text: *const c_char) -> *mut c_char;
    pub(crate) fn TextToSnake(text: *const c_char) -> *mut c_char;
    pub(crate) fn TextToCamel(text: *const c_char) -> *mut c_char;
    pub(crate) fn TextToInteger(text: *const c_char) -> c_int;
    pub(crate) fn TextToFloat(text: *const c_char) -> c_float;

    //------------------------------------------------------------------------------------
    // Basic 3d Shapes Drawing Functions (Module: models)
    //------------------------------------------------------------------------------------

    // Basic geometric 3D shapes drawing functions
    pub(crate) fn DrawLine3D(startPos: Vector3, endPos: Vector3, color: Color);
    pub(crate) fn DrawPoint3D(position: Vector3, color: Color);
    pub(crate) fn DrawCircle3D(
        center: Vector3,
        radius: c_float,
        rotationAxis: Vector3,
        rotationAngle: c_float,
        color: Color,
    );
    pub(crate) fn DrawTriangle3D(v1: Vector3, v2: Vector3, v3: Vector3, color: Color);
    pub(crate) fn DrawTriangleStrip3D(points: *const Vector3, pointCount: c_int, color: Color);
    pub(crate) fn DrawCube(
        position: Vector3,
        width: c_float,
        height: c_float,
        length: c_float,
        color: Color,
    );
    pub(crate) fn DrawCubeV(position: Vector3, size: Vector3, color: Color);
    pub(crate) fn DrawCubeWires(
        position: Vector3,
        width: c_float,
        height: c_float,
        length: c_float,
        color: Color,
    );
    pub(crate) fn DrawCubeWiresV(position: Vector3, size: Vector3, color: Color);
    pub(crate) fn DrawSphere(centerPos: Vector3, radius: c_float, color: Color);
    pub(crate) fn DrawSphereEx(
        centerPos: Vector3,
        radius: c_float,
        rings: c_int,
        slices: c_int,
        color: Color,
    );
    pub(crate) fn DrawSphereWires(
        centerPos: Vector3,
        radius: c_float,
        rings: c_int,
        slices: c_int,
        color: Color,
    );
    pub(crate) fn DrawCylinder(
        position: Vector3,
        radiusTop: c_float,
        radiusBottom: c_float,
        height: c_float,
        slices: c_int,
        color: Color,
    );
    pub(crate) fn DrawCylinderEx(
        startPos: Vector3,
        endPos: Vector3,
        startRadius: c_float,
        endRadius: c_float,
        sides: c_int,
        color: Color,
    );
    pub(crate) fn DrawCylinderWires(
        position: Vector3,
        radiusTop: c_float,
        radiusBottom: c_float,
        height: c_float,
        slices: c_int,
        color: Color,
    );
    pub(crate) fn DrawCylinderWiresEx(
        startPos: Vector3,
        endPos: Vector3,
        startRadius: c_float,
        endRadius: c_float,
        sides: c_int,
        color: Color,
    );
    pub(crate) fn DrawCapsule(
        startPos: Vector3,
        endPos: Vector3,
        radius: c_float,
        slices: c_int,
        rings: c_int,
        color: Color,
    );
    pub(crate) fn DrawCapsuleWires(
        startPos: Vector3,
        endPos: Vector3,
        radius: c_float,
        slices: c_int,
        rings: c_int,
        color: Color,
    );
    pub(crate) fn DrawPlane(centerPos: Vector3, size: Vector2, color: Color);
    pub(crate) fn DrawRay(ray: Ray, color: Color);
    pub(crate) fn DrawGrid(slices: c_int, spacing: c_float);

    //------------------------------------------------------------------------------------
    // Model 3d Loading and Drawing Functions (Module: models)
    //------------------------------------------------------------------------------------

    // Model management functions
    pub(crate) fn LoadModel(fileName: *const c_char) -> Model;
    pub(crate) fn LoadModelFromMesh(mesh: Mesh) -> Model;
    pub(crate) fn IsModelValid(model: Model) -> bool;
    pub(crate) fn UnloadModel(model: Model);
    pub(crate) fn GetModelBoundingBox(model: Model) -> BoundingBox;

    // Model drawing functions
    pub(crate) fn DrawModel(model: Model, position: Vector3, scale: c_float, tint: Color);
    pub(crate) fn DrawModelEx(
        model: Model,
        position: Vector3,
        rotationAxis: Vector3,
        rotationAngle: c_float,
        scale: Vector3,
        tint: Color,
    );
    pub(crate) fn DrawModelWires(model: Model, position: Vector3, scale: c_float, tint: Color);
    pub(crate) fn DrawModelWiresEx(
        model: Model,
        position: Vector3,
        rotationAxis: Vector3,
        rotationAngle: c_float,
        scale: Vector3,
        tint: Color,
    );
    pub(crate) fn DrawModelPoints(model: Model, position: Vector3, scale: c_float, tint: Color);
    pub(crate) fn DrawModelPointsEx(
        model: Model,
        position: Vector3,
        rotationAxis: Vector3,
        rotationAngle: c_float,
        scale: Vector3,
        tint: Color,
    );
    pub(crate) fn DrawBoundingBox(box_: BoundingBox, color: Color);
    pub(crate) fn DrawBillboard(
        camera: Camera,
        texture: Texture2D,
        position: Vector3,
        scale: c_float,
        tint: Color,
    );
    pub(crate) fn DrawBillboardRec(
        camera: Camera,
        texture: Texture2D,
        source: Rectangle,
        position: Vector3,
        size: Vector2,
        tint: Color,
    );
    pub(crate) fn DrawBillboardPro(
        camera: Camera,
        texture: Texture2D,
        source: Rectangle,
        position: Vector3,
        up: Vector3,
        size: Vector2,
        origin: Vector2,
        rotation: c_float,
        tint: Color,
    );

    // Mesh management functions
    pub(crate) fn UploadMesh(mesh: *mut Mesh, dynamic: bool);
    pub(crate) fn UpdateMeshBuffer(
        mesh: Mesh,
        index: c_int,
        data: *const c_void,
        dataSize: c_int,
        offset: c_int,
    );
    pub(crate) fn UnloadMesh(mesh: Mesh);
    pub(crate) fn DrawMesh(mesh: Mesh, material: Material, transform: Matrix);
    pub(crate) fn DrawMeshInstanced(
        mesh: Mesh,
        material: Material,
        transforms: *const Matrix,
        instances: c_int,
    );
    pub(crate) fn GetMeshBoundingBox(mesh: Mesh) -> BoundingBox;
    pub(crate) fn GenMeshTangents(mesh: *mut Mesh);
    pub(crate) fn ExportMesh(mesh: Mesh, fileName: *const c_char) -> bool;
    pub(crate) fn ExportMeshAsCode(mesh: Mesh, fileName: *const c_char) -> bool;

    // Mesh generation functions
    pub(crate) fn GenMeshPoly(sides: c_int, radius: c_float) -> Mesh;
    pub(crate) fn GenMeshPlane(
        width: c_float,
        length: c_float,
        resX: c_int,
        resZ: c_int,
    ) -> Mesh;
    pub(crate) fn GenMeshCube(width: c_float, height: c_float, length: c_float) -> Mesh;
    pub(crate) fn GenMeshSphere(radius: c_float, rings: c_int, slices: c_int) -> Mesh;
    pub(crate) fn GenMeshHemiSphere(radius: c_float, rings: c_int, slices: c_int) -> Mesh;
    pub(crate) fn GenMeshCylinder(radius: c_float, height: c_float, slices: c_int) -> Mesh;
    pub(crate) fn GenMeshCone(radius: c_float, height: c_float, slices: c_int) -> Mesh;
    pub(crate) fn GenMeshTorus(
        radius: c_float,
        size: c_float,
        radSeg: c_int,
        sides: c_int,
    ) -> Mesh;
    pub(crate) fn GenMeshKnot(
        radius: c_float,
        size: c_float,
        radSeg: c_int,
        sides: c_int,
    ) -> Mesh;
    pub(crate) fn GenMeshHeightmap(heightmap: Image, size: Vector3) -> Mesh;
    pub(crate) fn GenMeshCubicmap(cubicmap: Image, cubeSize: Vector3) -> Mesh;

    // Material loading/unloading functions
    pub(crate) fn LoadMaterials(fileName: *const c_char, materialCount: *mut c_int)
                                -> *mut Material;
    pub(crate) fn LoadMaterialDefault() -> Material;
    pub(crate) fn IsMaterialValid(material: Material) -> bool;
    pub(crate) fn UnloadMaterial(material: Material);
    pub(crate) fn SetMaterialTexture(
        material: *mut Material,
        mapType: c_int,
        texture: Texture2D,
    );
    pub(crate) fn SetModelMeshMaterial(model: *mut Model, meshId: c_int, materialId: c_int);

    // Model animations loading/unloading functions
    pub(crate) fn LoadModelAnimations(
        fileName: *const c_char,
        animCount: *mut c_int,
    ) -> *mut ModelAnimation;
    pub(crate) fn UpdateModelAnimation(model: Model, anim: ModelAnimation, frame: c_int);
    pub(crate) fn UpdateModelAnimationBones(model: Model, anim: ModelAnimation, frame: c_int);
    pub(crate) fn UnloadModelAnimation(anim: ModelAnimation);
    pub(crate) fn UnloadModelAnimations(animations: *mut ModelAnimation, animCount: c_int);
    pub(crate) fn IsModelAnimationValid(model: Model, anim: ModelAnimation) -> bool;

    // Collision detection functions
    pub(crate) fn CheckCollisionSpheres(
        center1: Vector3,
        radius1: c_float,
        center2: Vector3,
        radius2: c_float,
    ) -> bool;
    pub(crate) fn CheckCollisionBoxes(box1: BoundingBox, box2: BoundingBox) -> bool;
    pub(crate) fn CheckCollisionBoxSphere(
        box_: BoundingBox,
        center: Vector3,
        radius: c_float,
    ) -> bool;
    pub(crate) fn GetRayCollisionSphere(ray: Ray, center: Vector3, radius: c_float)
                                        -> RayCollision;
    pub(crate) fn GetRayCollisionBox(ray: Ray, box_: BoundingBox) -> RayCollision;
    pub(crate) fn GetRayCollisionMesh(ray: Ray, mesh: Mesh, transform: Matrix) -> RayCollision;
    pub(crate) fn GetRayCollisionTriangle(
        ray: Ray,
        p1: Vector3,
        p2: Vector3,
        p3: Vector3,
    ) -> RayCollision;
    pub(crate) fn GetRayCollisionQuad(
        ray: Ray,
        p1: Vector3,
        p2: Vector3,
        p3: Vector3,
        p4: Vector3,
    ) -> RayCollision;

    //------------------------------------------------------------------------------------
    // Audio Loading and Playing Functions (Module: audio)
    //------------------------------------------------------------------------------------

    // Audio device management functions
    pub(crate) fn InitAudioDevice();
    pub(crate) fn CloseAudioDevice();
    pub(crate) fn IsAudioDeviceReady() -> bool;
    pub(crate) fn SetMasterVolume(volume: c_float);
    pub(crate) fn GetMasterVolume() -> c_float;

    // Wave/Sound loading/unloading functions
    pub(crate) fn LoadWave(fileName: *const c_char) -> Wave;
    pub(crate) fn LoadWaveFromMemory(
        fileType: *const c_char,
        fileData: *const c_uchar,
        dataSize: c_int,
    ) -> Wave;
    pub(crate) fn IsWaveValid(wave: Wave) -> bool;
    pub(crate) fn LoadSound(fileName: *const c_char) -> Sound;
    pub(crate) fn LoadSoundFromWave(wave: Wave) -> Sound;
    pub(crate) fn LoadSoundAlias(source: Sound) -> Sound;
    pub(crate) fn IsSoundValid(sound: Sound) -> bool;
    pub(crate) fn UpdateSound(sound: Sound, data: *const c_void, sampleCount: c_int);
    pub(crate) fn UnloadWave(wave: Wave);
    pub(crate) fn UnloadSound(sound: Sound);
    pub(crate) fn UnloadSoundAlias(alias: Sound);
    pub(crate) fn ExportWave(wave: Wave, fileName: *const c_char) -> bool;
    pub(crate) fn ExportWaveAsCode(wave: Wave, fileName: *const c_char) -> bool;

    // Wave/Sound management functions
    pub(crate) fn PlaySound(sound: Sound);
    pub(crate) fn StopSound(sound: Sound);
    pub(crate) fn PauseSound(sound: Sound);
    pub(crate) fn ResumeSound(sound: Sound);
    pub(crate) fn IsSoundPlaying(sound: Sound) -> bool;
    pub(crate) fn SetSoundVolume(sound: Sound, volume: c_float);
    pub(crate) fn SetSoundPitch(sound: Sound, pitch: c_float);
    pub(crate) fn SetSoundPan(sound: Sound, pan: c_float);
    pub(crate) fn WaveCopy(wave: Wave) -> Wave;
    pub(crate) fn WaveCrop(wave: *mut Wave, initFrame: c_int, finalFrame: c_int);
    pub(crate) fn WaveFormat(
        wave: *mut Wave,
        sampleRate: c_int,
        sampleSize: c_int,
        channels: c_int,
    );
    pub(crate) fn LoadWaveSamples(wave: Wave) -> *mut c_float;
    pub(crate) fn UnloadWaveSamples(samples: *mut c_float);

    // Music management functions
    pub(crate) fn LoadMusicStream(fileName: *const c_char) -> Music;
    pub(crate) fn LoadMusicStreamFromMemory(
        fileType: *const c_char,
        data: *const c_uchar,
        dataSize: c_int,
    ) -> Music;
    pub(crate) fn IsMusicValid(music: Music) -> bool;
    pub(crate) fn UnloadMusicStream(music: Music);
    pub(crate) fn PlayMusicStream(music: Music);
    pub(crate) fn IsMusicStreamPlaying(music: Music) -> bool;
    pub(crate) fn UpdateMusicStream(music: Music);
    pub(crate) fn StopMusicStream(music: Music);
    pub(crate) fn PauseMusicStream(music: Music);
    pub(crate) fn ResumeMusicStream(music: Music);
    pub(crate) fn SeekMusicStream(music: Music, position: c_float);
    pub(crate) fn SetMusicVolume(music: Music, volume: c_float);
    pub(crate) fn SetMusicPitch(music: Music, pitch: c_float);
    pub(crate) fn SetMusicPan(music: Music, pan: c_float);
    pub(crate) fn GetMusicTimeLength(music: Music) -> c_float;
    pub(crate) fn GetMusicTimePlayed(music: Music) -> c_float;

    // AudioStream management functions
    pub(crate) fn LoadAudioStream(
        sampleRate: c_uint,
        sampleSize: c_uint,
        channels: c_uint,
    ) -> AudioStream;
    pub(crate) fn IsAudioStreamValid(stream: AudioStream) -> bool;
    pub(crate) fn UnloadAudioStream(stream: AudioStream);
    pub(crate) fn UpdateAudioStream(stream: AudioStream, data: *const c_void, frameCount: c_int);
    pub(crate) fn IsAudioStreamProcessed(stream: AudioStream) -> bool;
    pub(crate) fn PlayAudioStream(stream: AudioStream);
    pub(crate) fn PauseAudioStream(stream: AudioStream);
    pub(crate) fn ResumeAudioStream(stream: AudioStream);
    pub(crate) fn IsAudioStreamPlaying(stream: AudioStream) -> bool;
    pub(crate) fn StopAudioStream(stream: AudioStream);
    pub(crate) fn SetAudioStreamVolume(stream: AudioStream, volume: c_float);
    pub(crate) fn SetAudioStreamPitch(stream: AudioStream, pitch: c_float);
    pub(crate) fn SetAudioStreamPan(stream: AudioStream, pan: c_float);
    pub(crate) fn SetAudioStreamBufferSizeDefault(size: c_int);
    pub(crate) fn SetAudioStreamCallback(stream: AudioStream, callback: AudioCallback);

    pub(crate) fn AttachAudioStreamProcessor(stream: AudioStream, processor: AudioCallback);
    pub(crate) fn DetachAudioStreamProcessor(stream: AudioStream, processor: AudioCallback);

    pub(crate) fn AttachAudioMixedProcessor(processor: AudioCallback);
    pub(crate) fn DetachAudioMixedProcessor(processor: AudioCallback);
}