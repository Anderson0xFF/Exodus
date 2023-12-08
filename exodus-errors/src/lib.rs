#![allow(dead_code)]
#![allow(non_camel_case_types)]

#[derive(Debug, Clone, Copy)]
pub enum ErrorKind {
    // Connection
    CONNECTION_FAILED = 1,
    CONNECTION_CLOSED,
    CONNECTION_TIMEOUT,

    NETWORKMESSAGE_FAILED,
    NETWORKMESSAGE_EMPTY,

    // GPU
    GPU_LOAD_FAILED,
    GPU_RESOURCES_FAILED,
    GPU_NOT_FOUND,
    GPUS_LIST_FAILED,

    // Display
    DISPLAY_NOT_FOUND,
    DISPLAY_LISTENER_FAILED,

    //NativeDevice
    NATIVE_DEVICE_NOT_FOUND,
    DEVICE_MANAGER_CREATE_FAILED,

    // Buffer
    BUFFER_EMPTY,
    BUFFER_CREATE_FAILED,
    BUFFER_MAPPING_FAILED,
    /// This error is thrown when the buffer is out of bounds.
    BUFFER_OUT_OF_BOUNDS,
    /// This error is thrown when the pixel buffer length is not equal to width * height.
    BUFFER_INVALID_PIXELS,

    // Surface
    SURFACE_CREATE_FAILED,
    SURFACE_GET_BUFFER_FAILED,

    // SurfaceLock
    SURFACE_LOCK_FAILED,
    SURFACE_LOCK_MAPPING_FAILED,

    // Framebuffer
    FRAMEBUFFER_CREATE_FAILED,

    // Crtc
    CRTC_NOT_FOUND,
    CRTC_FAILED,
    CRTC_SET_FAILED,

    // Encoder
    ENCODER_FAILED,

    // Screen
    CONNECTOR_FAILED,
    CONNECTOR_MODE_FAILED,
    SCREEN_DISCONNECTED,
    SCREEN_NOT_FOUND,

    // Protocol
    PROTOCOL_FAILED,


}