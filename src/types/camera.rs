use crate::{Vector2, Vector3};

/// Camera system modes
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CameraMode {
    /// Camera custom, controlled by user (UpdateCamera() does nothing)
    Custom,
    /// Camera free mode
    Free,
    /// Camera orbital, around target, zoom supported
    Orbital,
    /// Camera first person
    FirstPerson,
    /// Camera third person
    ThirdPerson,
}

/// Camera projection
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CameraProjection {
    /// Perspective projection
    Perspective = 0,
    /// Orthographic projection
    Orthographic = 1,
}

/// Camera, defines position/orientation in 3d space
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Camera3D {
    /// Camera position
    pub position: Vector3,
    /// Camera target it looks-at
    pub target: Vector3,
    /// Camera up vector (rotation over its axis)
    pub up: Vector3,
    /// Camera field-of-view aperture in Y (degrees) in perspective,
    /// used as near plane height in world units in orthographic
    pub fov_y: f32,
    /// Camera projection: CAMERA_PERSPECTIVE or CAMERA_ORTHOGRAPHIC
    pub projection: CameraProjection,
}

/// Camera type fallback, defaults to Camera3D
pub type Camera = Camera3D;

/// Camera2D, defines position/orientation in 2d space
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Camera2D {
    /// Camera offset (displacement from target)
    pub offset: Vector2,
    /// Camera target (rotation and zoom origin)
    pub target: Vector2,
    /// Camera rotation in degrees
    pub rotation: f32,
    /// Camera zoom (scaling), should be 1.0f by default
    pub zoom: f32,
}
