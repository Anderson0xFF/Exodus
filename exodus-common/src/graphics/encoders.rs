use std::{fmt::Display, rc::Rc};
use drm::{drmModeGetEncoder, drmModeFreeEncoder};
use exodus_errors::ErrorKind;

use crate::{debug, error};


pub type EncoderRef = Rc<Encoder>;

#[derive(Debug, Clone, Copy)]
pub struct Encoder {
    id: u32,
    crtc_id: u32,
    encoder_type: u32,
    possible_crtcs: u32,
    possible_clones: u32,
}

impl Encoder {
    pub fn new(id: u32, device: i32) -> Result<Self, ErrorKind> {
        debug!("Getting encoder...");
        unsafe {
            let encoder_ptr: *mut drm::_drmModeEncoder = drmModeGetEncoder(device, id);

            if encoder_ptr.is_null() {
                let err = ErrorKind::EXODUS_ENCODER_FAILED;
                error!("Failed to get encoder. - ErrorKind: {:?}", err);
                return Err(err);
            }

            let encoder = encoder_ptr.as_ref().unwrap();
            let encoder = Encoder {
                id,
                encoder_type: encoder.encoder_type,
                possible_crtcs: encoder.possible_crtcs,
                possible_clones: encoder.possible_clones,
                crtc_id: encoder.crtc_id,
            };

            debug!("Found encoder: {}", encoder);

            drmModeFreeEncoder(encoder_ptr);

            Ok(encoder)
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn encoder_type(&self) -> u32 {
        self.encoder_type
    }

    pub fn possible_crtcs(&self) -> u32 {
        self.possible_crtcs
    }

    pub fn possible_clones(&self) -> u32 {
        self.possible_clones
    }

    pub fn crtc_id(&self) -> u32 {
        self.crtc_id
    }
}

impl Display for Encoder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Encoder {{ id: {}, encoder_type: {}, possible_crtcs: {}, possible_clones: {} }}", self.id, self.encoder_type,  self.possible_crtcs, self.possible_clones)
    }
}
