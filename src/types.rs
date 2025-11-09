use std::ffi::c_void;
use crate::{get_gamepad_axis_count, get_gamepad_axis_movement, get_gamepad_name, is_gamepad_available, is_gamepad_button_down, is_gamepad_button_pressed, is_gamepad_button_released, is_gamepad_button_up, load_texture, set_gamepad_vibration};
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

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
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Image {
    data: *mut c_void,  // Image raw data
    width: i32,         // Image base width
    height: i32,        // Image base height
    mipmaps: i32,       // Mipmap levels, 1 by default
    format: i32,        // Data format (PixelFormat type)
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Texture {
    id: u32,      // OpenGL texture id
    width: i32,   // Texture base width
    height: i32,  // Texture base height
    mipmaps: i32, // Mipmap levels, 1 by default
    format: i32,  // Data format (PixelFormat type)
}

pub type Texture2D = Texture;
pub type TextureCubeMap = Texture;

impl Texture {
    pub fn load(filename: &str) -> Texture2D {
        return load_texture(filename);
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
