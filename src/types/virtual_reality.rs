use crate::Matrix;

/// VrDeviceInfo, Head-Mounted-Display device parameters
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VrDeviceInfo {
    /// Horizontal resolution in pixels
    pub h_resolution: i32,
    /// Vertical resolution in pixels
    pub v_resolution: i32,
    /// Horizontal size in meters
    pub h_screen_size: f32,
    /// Vertical size in meters
    pub v_screen_size: f32,
    /// Distance between eye and display in meters
    pub eye_to_screen_distance: f32,
    /// Lens separation distance in meters
    pub lens_separation_distance: f32,
    /// IPD (distance between pupils) in meters
    pub interpupillary_distance: f32,
    /// Lens distortion constant parameters
    pub lens_distortion_values: [f32; 4],
    /// Chromatic aberration correction parameters
    pub chroma_ab_correction: [f32; 4],
}

/// VrStereoConfig, VR stereo rendering configuration for simulator
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct VrStereoConfig {
    /// VR projection matrices (per eye)
    pub projection: [Matrix; 2],
    /// VR view offset matrices (per eye)
    pub view_offset: [Matrix; 2],
    /// VR left lens center
    pub left_lens_center: [f32; 2],
    /// VR right lens center
    pub right_lens_center: [f32; 2],
    /// VR left screen center
    pub left_screen_center: [f32; 2],
    /// VR right screen center
    pub right_screen_center: [f32; 2],
    /// VR distortion scale
    pub scale: [f32; 2],
    /// VR distortion scale in
    pub scale_in: [f32; 2],
}
