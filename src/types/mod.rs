mod audio;
mod camera;
mod color;
mod config_flag;
mod font;
mod image;
mod input;
mod math;
mod physics;
mod rendering;
mod texture;
mod virtual_reality;

pub use audio::*;
pub use camera::*;
pub use color::*;
pub use config_flag::*;
pub use font::*;
pub use image::*;
pub use input::*;
pub use math::*;
pub use physics::*;
pub use rendering::*;
pub use texture::*;
pub use virtual_reality::*;

/// File path list
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FilePathList {
    /// File paths max entries
    pub capacity: u32,
    /// File paths entries count
    pub count: u32,
    /// File paths entries
    pub paths: *mut *mut u8,
}

/// Automation event
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AutomationEvent {
    /// Events max entries (MAX_AUTOMATION_EVENTS)
    pub frame: u32,
    /// Events entries count
    pub event_type: u32,
    /// Events entries
    pub params: [i32; 4],
}

/// Trace log level
/// NOTE: Organized by priority level
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TraceLogLevel {
    /// Display all logs
    All = 0,
    /// Trace logging, intended for internal use only
    Trace,
    /// Debug logging, used for internal debugging, it should be disabled on release builds
    Debug,
    /// Info logging, used for program execution info
    Info,
    /// Warning logging, used on recoverable failures
    Warning,
    /// Error logging, used on unrecoverable failures
    Error,
    /// Fatal logging, used to abort program: exit(EXIT_FAILURE)
    Fatal,
    /// Disable logging
    None,
}
