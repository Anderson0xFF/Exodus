use drm::{drmModeFreeCrtc, drmModeGetCrtc, drmModeModeInfo, drmModeModeInfoPtr, drmModeSetCrtc};
use exodus_errors::ErrorKind;

use crate::{debug, error};

use super::{device::GPUID, framebuffer::Framebuffer};

#[derive(Debug)]
pub struct CRTC {
    id: u32,
    buffer_id: u32,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    mode: drmModeModeInfo,
    gamma_size: i32,
    gpu: GPUID,
}

impl CRTC {
    pub fn new(gpu: GPUID, crtc_id: u32) -> Result<Self, ErrorKind> {
        debug!("Getting crtc. - GPUID: {}, CrtcID: {}", gpu, crtc_id);

        if crtc_id == 0 {
            let err = ErrorKind::CRTC_NOT_FOUND;
            error!("Failed to get crtc. - ErrorKind: {:?}", err);
            return Err(err);
        }

        let crtc_ptr: *mut drm::_drmModeCrtc = unsafe { drmModeGetCrtc(gpu, crtc_id) };

        if crtc_ptr.is_null() {
            let err = ErrorKind::CRTC_FAILED;
            error!("Failed to get crtc. - ErrorKind: {:?}", err);
            return Err(err);
        }

        let crtc = unsafe { crtc_ptr.as_ref().unwrap() };

        let crtc = CRTC {
            id: crtc_id,
            buffer_id: crtc.buffer_id,
            x: crtc.x,
            y: crtc.y,
            width: crtc.width,
            height: crtc.height,
            mode: crtc.mode,
            gamma_size: crtc.gamma_size,
            gpu,
        };

        unsafe { drmModeFreeCrtc(crtc_ptr) };
        Ok(crtc)
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

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn mode(&self) -> drmModeModeInfo {
        self.mode
    }

    pub fn gamma_size(&self) -> i32 {
        self.gamma_size
    }

    pub fn set(
        &mut self,
        connectors: &mut [u32],
        mode: drmModeModeInfoPtr,
        framebuffer: &Framebuffer,
    ) -> Result<(), ErrorKind> {
        unsafe {
            drmModeSetCrtc(
                self.gpu,
                self.id,
                framebuffer.id(),
                0,
                0,
                connectors.as_mut_ptr(),
                connectors.len().try_into().unwrap(),
                mode,
            );
        }
        Ok(())
    }

    pub fn restore(&mut self, connectors: &mut [u32]) {
        unsafe {
            drmModeSetCrtc(self.gpu, self.id, self.buffer_id,
                self.x, self.y, connectors.as_mut_ptr(), 1, &mut self.mode);
        }
    }
}