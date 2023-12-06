#![allow(non_camel_case_types)]

use drm::*;

pub type ConnectorPtr = *mut drmModeConnector;
pub type ResourcesPtr = *mut drmModeRes;
pub type EncoderPtr = *mut drmModeEncoder;
pub type CrtcPtr = *mut drmModeCrtc;
pub type FramebufferPtr = *mut drmModeFB;
pub type PlanePtr = *mut drmModePlane;

pub type ModeInfo = drmModeModeInfo;

pub type int8_t = i8;
pub type int16_t = i16;
pub type int32_t = i32;
pub type int64_t = i64;

pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;


