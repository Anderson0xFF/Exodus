#![allow(dead_code)]
#![allow(non_camel_case_types)]

#[derive(Debug, Clone, Copy)]
pub enum ErrorKind {
    RAMEN_GRAPHICS_DEVICES_NOT_FOUND,
    RAMEN_GRAPHICS_DEVICES_LIST_FAILED,

    RAMEN_GRAPHIC_DEVICE_LOAD_FAILED,
    RAMEN_GRAPHIC_DEVICE_RESOURCES_FAILED,

    RAMEN_DISPLAY_CONNECTORS_NOT_FOUND,

    RAMEN_CONNECTOR_NOT_CONNECTED,
    RAMEN_CONNECTOR_ENCODER_NOT_FOUND,
    RAMEN_CONNECTOR_ENCODER_FAILED,
    RAMEN_CONNECTOR_ENCODER_CRTC_NOT_FOUND,
    RAMEN_CONNECTOR_ENCODER_CRTC_FAILED,
    
    RAMEN_DISPLAY_GBM_DEVICE_FAILED,
    RAMEN_CONNECTOR_FAILED
}