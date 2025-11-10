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

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseCursor {
    Default = 0,      // Default pointer shape
    Arrow = 1,        // Arrow shape
    IBeam = 2,        // Text-write-insertion shape
    Crosshair = 3,    // Cross-hair shape
    PointingHand = 4, // Pointing-hand shape
    ResizeEW = 5,     // Horizontal-resize shape
    ResizeNS = 6,     // Vertical-resize shape
    ResizeNWSE = 7,   // Top-left to bottom-right diagonal resize shape
    ResizeNESW = 8,   // Top-right to bottom-left diagonal resize shape
    ResizeAll = 9,    // Move-all direction shape
    NotAllowed = 10,  // Operation-not-allowed shape
}
