use crate::Vector3;

/// Bounding box
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BoundingBox {
    /// Minimum vertex box-corner
    pub min: Vector3,
    /// Maximum vertex box-corner
    pub max: Vector3,
}

/// Ray, ray for ray casting
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray {
    /// Ray position (origin)
    pub position: Vector3,
    /// Ray direction (normalized)
    pub direction: Vector3,
}

/// RayCollision, ray hit information
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RayCollision {
    /// Did the ray hit something?
    pub hit: bool,
    /// Distance to the nearest hit
    pub distance: f32,
    /// Point of the nearest hit
    pub point: Vector3,
    /// Surface normal of hit
    pub normal: Vector3,
}
