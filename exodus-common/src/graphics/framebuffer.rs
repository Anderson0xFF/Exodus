use drm::{drmModeAddFB, drmModeRmFB};
use exodus_errors::ErrorKind;

use super::{buffer::Buffer, device::GPUID};


#[derive(Debug)]
pub struct Framebuffer {
    id: u32,
    gpu_id: i32,
}

impl Framebuffer {
    pub fn new(gpu: GPUID, buffer: &Buffer) -> Result<Self, ErrorKind> {
        unsafe {
            let mut id = 0;
            let width = buffer.width();
            let height = buffer.height();
            let depth = 24;
            let bpp = buffer.bpp() as u8;
            let pitch = buffer.stride();
            let handle = buffer.handle();
            let result = drmModeAddFB(gpu, width, height, depth, bpp, pitch, handle, &mut id);

            if result != 0 {
                return Err(ErrorKind::FRAMEBUFFER_CREATE_FAILED);
            }

            let framebuffer = Framebuffer {
                id,
                gpu_id: gpu,
            };
            Ok(framebuffer)
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        unsafe {
            drmModeRmFB(self.gpu_id, self.id);
        }
    }
}
