use std::fmt::Display;

use drm::{drmModeGetEncoder, drmModeFreeEncoder};
use crate::{errors::ErrorKind, error, debug};
use super::crtcs::Crtc;


#[derive(Debug, Clone, Copy)]
pub struct Encoder {
    id: u32,
    encoder_type: u32,
    crtc: Option<Crtc>,
    possible_crtcs: u32,
    possible_clones: u32,
}

impl Encoder {
    pub fn new(device: i32, id: u32) -> Result<Self, ErrorKind> {
        unsafe {
            let encoder_ptr = drmModeGetEncoder(device, id);

            if encoder_ptr.is_null() {
                let err = ErrorKind::RAMEN_CONNECTOR_ENCODER_FAILED;
                error!("Failed to get encoder. - ErrorKind: {:?}", err);
                return Err(err);
            }

            let encoder = encoder_ptr.as_ref().unwrap();
            let mut crtc = None;

            if encoder.crtc_id != 0 {
                crtc = Some(Crtc::new(device, encoder.crtc_id)?);
            }

            drmModeFreeEncoder(encoder_ptr);

            let encoder = Encoder {
                id,
                encoder_type: encoder.encoder_type,
                crtc,
                possible_crtcs: encoder.possible_crtcs,
                possible_clones: encoder.possible_clones,
            };

            debug!("Found encoder: {}", encoder);

            Ok(encoder)
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn encoder_type(&self) -> u32 {
        self.encoder_type
    }

    pub fn crtc(&self) -> Option<Crtc> {
        self.crtc
    }

    pub fn possible_crtcs(&self) -> u32 {
        self.possible_crtcs
    }

    pub fn possible_clones(&self) -> u32 {
        self.possible_clones
    }
}

impl Display for Encoder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Encoder {{ id: {}, encoder_type: {}, possible_crtcs: {}, possible_clones: {} }}", self.id, self.encoder_type,  self.possible_crtcs, self.possible_clones)
    }
}
