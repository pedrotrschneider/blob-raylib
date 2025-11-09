use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

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
