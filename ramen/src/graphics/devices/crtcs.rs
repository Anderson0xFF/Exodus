use std::fmt::Display;
use drm::{drmModeFreeCrtc, drmModeGetCrtc};
use crate::{debug, graphics::Size, errors::ErrorKind, types::ModeInfo, error};


#[derive(Debug, Clone, Copy)]
pub struct Crtc {
    id: u32,
    buffer_id: u32,
    x: u32,
    y: u32,
    size: Size<u32>,
    mode: ModeInfo,
    gamma_size: i32,
}

impl Crtc {
    pub fn new(device: i32, id: u32) -> Result<Self, ErrorKind> {
        unsafe {
            if id == 0 {
                let err = ErrorKind::RAMEN_CONNECTOR_ENCODER_CRTC_NOT_FOUND;
                error!("Failed to get crtc. - ErrorKind: {:?}", err);
                return Err(err);
            }

            let crtc_ptr = drmModeGetCrtc(device, id);

            if crtc_ptr.is_null() {
                let err = ErrorKind::RAMEN_CONNECTOR_ENCODER_CRTC_FAILED;
                error!("Failed to get crtc. - ErrorKind: {:?}", err);
                return Err(err);
            }

            let crtc = crtc_ptr.as_ref().unwrap().clone();

            drmModeFreeCrtc(crtc_ptr);

            let crtc = Crtc {
                id,
                buffer_id: crtc.buffer_id,
                x: crtc.x,
                y: crtc.y,
                size: Size {
                    width: crtc.width,
                    height: crtc.height,
                },
                mode: crtc.mode,
                gamma_size: crtc.gamma_size,
            };

            debug!("Found crtc: {}", crtc);

            Ok(crtc)
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn buffer_id(&self) -> u32 {
        self.buffer_id
    }

    pub fn x(&self) -> u32 {
        self.x
    }

    pub fn y(&self) -> u32 {
        self.y
    }

    pub fn size(&self) -> &Size<u32> {
        &self.size
    }

    pub fn mode(&self) -> ModeInfo {
        self.mode
    }

    pub fn gamma_size(&self) -> i32 {
        self.gamma_size
    }
}

impl Display for Crtc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Crtc {{ id: {}, buffer_id: {}, x: {}, y: {}, size: {}, gamma_size: {} }}", self.id, self.buffer_id, self.x, self.y, self.size, self.gamma_size)
    }
}
