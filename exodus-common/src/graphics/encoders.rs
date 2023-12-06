use drm::{drmModeFreeEncoder, drmModeGetEncoder};
use exodus_errors::ErrorKind;

use crate::{debug, error};

use super::device::GPUID;

#[derive(Debug, Clone, Copy)]
pub struct Encoder {
    id: u32,
    crtc_id: u32,
    encoder_type: u32,
    possible_crtcs: u32,
    possible_clones: u32,
}

impl Encoder {
    pub fn new(id: u32, gpu: GPUID) -> Result<Self, ErrorKind> {
        debug!("Getting encoder. - EncoderID: {} - GPUID: {}", id, gpu);

        let encoder_ptr = unsafe { drmModeGetEncoder(gpu, id) };

        if encoder_ptr.is_null() {
            let err = ErrorKind::ENCODER_FAILED;
            error!("Failed to get encoder, is null. - ErrorKind: {:?}", err);
            return Err(err);
        }

        let encoder = unsafe { encoder_ptr.as_ref().unwrap() };
        let encoder = Encoder {
            id,
            encoder_type: encoder.encoder_type,
            possible_crtcs: encoder.possible_crtcs,
            possible_clones: encoder.possible_clones,
            crtc_id: encoder.crtc_id,
        };

        unsafe { drmModeFreeEncoder(encoder_ptr) };
        Ok(encoder)
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
