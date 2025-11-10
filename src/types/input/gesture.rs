use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

pub struct GestureFlag {
    pub flag: u32,
}

impl GestureFlag {
    pub fn bits(&self) -> u32 {
        return self.flag;
    }
}

impl BitOr for GestureFlag {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        let result = self.flag | rhs.flag;
        return Self { flag: result };
    }
}

impl BitOrAssign for GestureFlag {
    fn bitor_assign(&mut self, rhs: Self) {
        self.flag |= rhs.flag;
    }
}

impl BitAnd for GestureFlag {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        let result = self.flag & rhs.flag;
        return Self { flag: result };
    }
}

impl BitAndAssign for GestureFlag {
    fn bitand_assign(&mut self, rhs: Self) {
        self.flag &= rhs.flag;
    }
}

impl BitXor for GestureFlag {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        let result = self.flag ^ rhs.flag;
        return Self { flag: result };
    }
}

impl BitXorAssign for GestureFlag {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.flag ^= rhs.flag;
    }
}

impl Not for GestureFlag {
    type Output = Self;

    fn not(self) -> Self::Output {
        return Self { flag: !self.flag };
    }
}

impl TryFrom<u32> for GestureFlag {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        return Ok(Self { flag: value });
    }
}

impl TryFrom<i32> for GestureFlag {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value < 0 {
            return Err(());
        }
        return Ok(Self { flag: value as u32 });
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GestureFlags {
    None = 0,
    Tap = 1,
    DoubleTap = 2,
    Hold = 4,
    Drag = 8,
    SwipeRight = 16,
    SwipeLeft = 32,
    SwipeUp = 64,
    SwipeDown = 128,
    PinchIn = 256,
    PinchOut = 512,
}

impl BitOr for GestureFlags {
    type Output = GestureFlag;

    fn bitor(self, rhs: Self) -> Self::Output {
        let result = self as u32 | rhs as u32;
        return Self::Output { flag: result };
    }
}

impl BitAnd for GestureFlags {
    type Output = GestureFlag;

    fn bitand(self, rhs: Self) -> Self::Output {
        let result = self as u32 & rhs as u32;
        return Self::Output { flag: result };
    }
}

impl BitXor for GestureFlags {
    type Output = GestureFlag;

    fn bitxor(self, rhs: Self) -> Self::Output {
        let result = self as u32 ^ rhs as u32;
        return Self::Output { flag: result };
    }
}

impl Not for GestureFlags {
    type Output = GestureFlag;

    fn not(self) -> Self::Output {
        return Self::Output { flag: !(self as u32) };
    }
}

impl Into<GestureFlag> for GestureFlags {
    fn into(self) -> GestureFlag {
        return GestureFlag { flag: self as u32 };
    }
}
