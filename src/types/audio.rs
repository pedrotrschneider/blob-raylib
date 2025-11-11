use std::ffi::c_void;

/// Wave, audio wave data
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Wave {
    /// Total number of frames (considering channels)
    pub frame_count: u32,
    /// Frequency (samples per second)
    pub sample_rate: u32,
    /// Bit depth (bits per sample): 8, 16, 32 (24 not supported)
    pub sample_size: u32,
    /// Number of channels (1-mono, 2-stereo, ...)
    pub channels: u32,
    /// Buffer data pointer
    pub(crate) data: *mut c_void,
}

/// ----------------
/// Opaque structs declaration
/// NOTE: Actual structs are defined internally in raudio module
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct rAudioBuffer {
    _private: (),
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct rAudioProcessor {
    _private: (),
}
/// ----------------

/// AudioStream, custom audio stream
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AudioStream {
    /// Pointer to internal data used by the audio system
    pub(crate) buffer: *mut rAudioBuffer,
    /// Pointer to internal data processor, useful for audio effects
    pub(crate) processor: *mut rAudioProcessor,

    /// Frequency (samples per second)
    pub sample_rate: u32,
    /// Bit depth (bits per sample): 8, 16, 32 (24 not supported)
    pub sample_size: u32,
    /// Number of channels (1-mono, 2-stereo, ...)
    pub channels: u32,
}

/// Sound
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Sound {
    /// Audio stream
    pub stream: AudioStream,
    /// Total number of frames (considering channels)
    pub frame_count: i32,
}

/// Music, audio stream, anything longer than ~10 seconds should be streamed
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Music {
    /// Audio stream
    pub stream: AudioStream,
    /// Total number of frames (considering channels)
    pub frame_count: i32,
    /// Music looping enable
    pub looping: bool,

    /// Type of music context (audio filetype)
    pub(crate) ctx_type: i32,
    /// Audio context data, depends on type
    pub(crate) ctx_data: *mut c_void,
}
