use crate::{begin_mode_2d, begin_mode_3d, end_mode_2d, end_mode_3d, get_camera_matrix, get_camera_matrix_2d, get_screen_to_world_2d, get_screen_to_world_ray, get_screen_to_world_ray_ex, get_world_to_screen, get_world_to_screen_2d, get_world_to_screen_ex, update_camera, update_camera_pro, Matrix, Ray, Vector2, Vector3};
use crate::bindings::BeginMode2D;

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

impl Camera3D {
    pub fn begin_mode(&self) {
        begin_mode_3d(*self);
    }
    
    pub fn end_mode(&self) {
        end_mode_3d();
    }
    
    pub fn mouse_ray(&self, position: Vector2) -> Ray {
        return get_screen_to_world_ray(position, *self);
    }
    
    pub fn screen_to_world_ray(&self, position: Vector2) -> Ray {
        return get_screen_to_world_ray(position, *self);
    }
    
    pub fn screen_to_world_ray_ex(&self, position: Vector2, width: i32, height: i32) -> Ray {
        return get_screen_to_world_ray_ex(position, *self, width, height);
    }
    
    pub fn world_to_screen(&self, position: Vector3) -> Vector2 {
        return get_world_to_screen(position, *self);
    }
    
    pub fn world_to_screen_ex(&self, position: Vector3, width: i32, height: i32) -> Vector2 {
        return get_world_to_screen_ex(position, *self, width, height);
    }
    
    pub fn matrix(&self) -> Matrix {
        return get_camera_matrix(*self);
    }
    
    pub fn update(&mut self, mode: CameraMode) {
        update_camera(self, mode);
    }
    
    pub fn update_pro(&mut self, movement: Vector3, rotation: Vector3, zoom: f32) {
        update_camera_pro(self, movement, rotation, zoom);
    }
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

impl Camera2D {
    pub fn begin_mode(&self) {
        begin_mode_2d(*self);
    }
    
    pub fn end_mode(&self) {
        end_mode_2d();
    }
    
    pub fn world_to_screen(&self, position: Vector2) -> Vector2 {
        return get_world_to_screen_2d(position, *self);
    }
    
    pub fn screen_to_world(&self, position: Vector2) -> Vector2 {
        return get_screen_to_world_2d(position, *self);
    }
    
    pub fn matrix(&self) -> Matrix {
        return get_camera_matrix_2d(*self);
    }
}