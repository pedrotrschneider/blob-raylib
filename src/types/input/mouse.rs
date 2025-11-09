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
