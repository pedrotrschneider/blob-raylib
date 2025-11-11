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

    pub fn zero() -> Self {
        return Self { x: 0.0, y: 0.0 };
    }
}

impl From<(f32, f32)> for Vector2 {
    fn from((x, y): (f32, f32)) -> Self {
        return Vector2::new(x, y);
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        return Self { x, y, z };
    }

    pub fn zero() -> Self {
        return Self { x: 0.0, y: 0.0, z: 0.0 };
    }
}

impl From<(f32, f32, f32)> for Vector3 {
    fn from((x, y, z): (f32, f32, f32)) -> Self {
        return Vector3::new(x, y, z);
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vector4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        return Self { x, y, z, w };
    }

    pub fn zero() -> Self {
        return Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        };
    }
}

impl From<(f32, f32, f32, f32)> for Vector4 {
    fn from((x, y, z, w): (f32, f32, f32, f32)) -> Self {
        return Vector4::new(x, y, z, w);
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Quaternion(Vector4);

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Matrix {
    pub m0: f32,
    pub m4: f32,
    pub m8: f32,
    pub m12: f32,
    pub m1: f32,
    pub m5: f32,
    pub m9: f32,
    pub m13: f32,
    pub m2: f32,
    pub m6: f32,
    pub m10: f32,
    pub m14: f32,
    pub m3: f32,
    pub m7: f32,
    pub m11: f32,
    pub m15: f32,
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

/// Transform, vertex transformation data
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Transform {
    /// Translation
    pub translation: Vector3,
    /// Rotation
    pub rotation: Quaternion,
    /// Scale
    pub scale: Vector3,
}
