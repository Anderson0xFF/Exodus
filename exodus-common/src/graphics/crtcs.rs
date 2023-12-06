use drm::{drmModeFreeCrtc, drmModeGetCrtc, drmModeModeInfo, drmModeSetCrtc, drmModeModeInfoPtr};
use exodus_errors::ErrorKind;

use crate::{debug, error};

use super::{framebuffer::Framebuffer, device::GPUID};

#[derive(Debug)]
pub struct Crtc {
    id: u32,
    buffer_id: u32,
    x: u32,
    y: u32,
    width : u32,
    height: u32,
    mode: drmModeModeInfo,
    connector: u32,
    gamma_size: i32,
    gpu: GPUID,
}

impl Crtc {
    pub fn new(id: u32, gpu: GPUID, connector: u32) -> Result<Self, ErrorKind> {
        debug!("Getting crtc...");

        if id == 0 {
            let err = ErrorKind::CRTC_NOT_FOUND;
            error!("Failed to get crtc. - ErrorKind: {:?}", err);
            return Err(err);
        }

        let crtc_ptr = unsafe { drmModeGetCrtc(gpu, id) };

        if crtc_ptr.is_null() {
            let err = ErrorKind::CRTC_FAILED;
            error!("Failed to get crtc. - ErrorKind: {:?}", err);
            return Err(err);
        }

        let crtc = unsafe { crtc_ptr.as_ref().unwrap().clone() };

        unsafe { drmModeFreeCrtc(crtc_ptr) };

        let crtc = Crtc {
            id,
            buffer_id: crtc.buffer_id,
            x: crtc.x,
            y: crtc.y,
            width: crtc.width,
            height: crtc.height,
            mode: crtc.mode,
            gamma_size: crtc.gamma_size,
            gpu,
            connector,
        };

        debug!("Found Crtc: {:?}", crtc);
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

    pub fn render_framebuffer(&mut self, mode: drmModeModeInfoPtr, framebuffer: &Framebuffer) -> Result<(), ErrorKind> {
        unsafe {
            drmModeSetCrtc(self.gpu, self.id, framebuffer.id(), 0, 0, &mut self.connector, 1, mode);
        }
        Ok(())
    }

    pub fn restore(&mut self) {
        unsafe {
            drmModeSetCrtc(self.gpu, self.id, self.buffer_id, self.x, self.y, &mut self.connector, 1, &mut self.mode);
        }
    }
}

impl Drop for Crtc {
    fn drop(&mut self) {
        self.restore();
    }
}
