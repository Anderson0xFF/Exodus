#![allow(dead_code)]
#![allow(non_camel_case_types)]

#[derive(Debug, Clone, Copy)]
pub enum ErrorKind {   
    EXODUS_GRAPHICS_DEVICES_NOT_FOUND,
    EXODUS_GRAPHICS_DEVICES_LIST_FAILED,
    EXODUS_GRAPHICS_DEVICES_LOAD_FAILED,
    EXODUS_GRAPHIC_DEVICE_RESOURCES_FAILED,

    EXODUS_NATIVE_VISION_CREATE_FAILED,
    EXODUS_NATIVE_VISION_START_FAILED,
    EXODUS_NATIVE_VISION_CONNECTORS_NOT_FOUND,
    EXODUS_NATIVE_VISION_GBM_DEVICE_FAILED,
    EXODUS_NATIVE_VISION_NO_CONNECTION_HANDLER,
    EXODUS_NATIVE_VISION_NO_PROTOCOL_HANDLER,
    EXODUS_NATIVE_VISION_NOT_RUNNING,

    EXODUS_VISION_ACTIVITY_CONNECT_FAILED,

    EXODUS_CONNECTOR_FAILED,
    EXODUS_CONNECTOR_NOT_CONNECTED,
    EXODUS_CONNECTOR_NO_MODES,
    EXODUS_CONNECTOR_ENCODER_NOT_FOUND,

    EXODUS_ENCODER_FAILED,

    EXODUS_CRTC_NOT_FOUND,
    EXODUS_CRTC_FAILED,
    
    EXODUS_CONNECTION_FAILED,
    EXODUS_CONNECTION_CLOSED,

    EXODUS_PROTOCOL_FAILED,
    EXODUS_PROTOCOL_NOT_SUPPORTED,
    EXODUS_PROTOCOL_INVALID,
    
    EXODUS_NETWORK_MESSAGE_EMPTY,
    EXODUS_NETWORK_MESSAGE_OVERFLOW,

    EXODUS_SURFACE_CREATE_FAILED,
    EXODUS_SURFACE_LOCK_FAILED,
    EXODUS_SURFACE_MAP_FAILED,
    EXODUS_SURFACE_FORMAT_UNKNOWN
}