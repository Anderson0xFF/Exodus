use exodus_errors::ErrorKind;
use gbm::gbm_bo_transfer_flags::{GBM_BO_TRANSFER_WRITE, GBM_BO_TRANSFER_READ};
use gbm::gbm_bo_flags::*;
use gbm::*;
use libc::c_void;


use crate::enums::{PixelFormat, BufferFlag};
use crate::{error, debug, verbose};

use super::device::Device;


#[derive(Debug)]
pub enum Buffer {
    Legacy {
        width: u32,
        height: u32,
        handle: u32,
        stride: u32,
        bpp: u32,
        format: PixelFormat,
        buffer: *mut c_void,
    },
    Native {
        width: u32,
        height: u32,
        handle: u32,
        stride: u32,
        bpp: u32,
        format: PixelFormat,
        buffer: *mut gbm_bo,
    },
}

impl Buffer {
    pub fn new(device: &Device, width: u32, height: u32, format: PixelFormat, buffer_flags: &[BufferFlag]) -> Result<Self, ErrorKind> {
        debug!("Creating buffer. - Width: {}, Height: {}, Format: {:?}, Flags: {:?}", width, height, format, buffer_flags);

        let mut flags = 0;

        for flag in buffer_flags {
            match flag {
                BufferFlag::Cursor      => flags |= GBM_BO_USE_CURSOR,
                BufferFlag::Linear      => flags |= GBM_BO_USE_LINEAR,
                BufferFlag::Protected   => flags |= GBM_BO_USE_PROTECTED,
                BufferFlag::Rendering   => flags |= GBM_BO_USE_RENDERING,
                BufferFlag::Scanout     => flags |= GBM_BO_USE_SCANOUT,
            }
        }

        let buffer = unsafe {
            gbm_bo_create(device.as_ptr(), width, height, format as u32, flags)
        };

        if buffer.is_null() {
            error!("Failed to create buffer. - ErrorKind: {:?}", ErrorKind::BUFFER_CREATE_FAILED);
            return Err(ErrorKind::BUFFER_CREATE_FAILED);
        }

        let handle = unsafe { gbm_bo_get_handle(buffer).u32_ };
        let stride = unsafe { gbm_bo_get_stride(buffer) };
        let bpp = unsafe { gbm_bo_get_bpp(buffer) };
        
        Ok(Self::Native {
            width,
            height,
            handle,
            stride,
            bpp,
            format,
            buffer,
        })
    }

    #[allow(dead_code)]
    #[allow(unused_variables)]
    fn create_legacy_buffer(width: u32, height: u32, format: PixelFormat) -> Result<Self, ErrorKind> {
        todo!()
    }

    /// Get the width of the buffer.
    pub fn width(&self) -> u32 {
        match self {
            Self::Legacy { width, .. } => *width,
            Self::Native { width, .. } => *width,
        }
    }

    /// Get the height of the buffer.
    pub fn height(&self) -> u32 {
        match self {
            Self::Legacy { height, .. } => *height,
            Self::Native { height, .. } => *height,
        }
    }

    /// Get the handle of the buffer.
    pub fn handle(&self) -> u32 {
        match self {
            Self::Legacy { handle, .. } => *handle,
            Self::Native { handle, .. } => *handle,
        }
    }

    /// Get the stride of the buffer.
    pub fn stride(&self) -> u32 {
        match self {
            Self::Legacy { stride, .. } => *stride,
            Self::Native { stride, .. } => *stride,
        }
    }

    /// Get the bits per pixel of the buffer.
    pub fn bpp(&self) -> u32 {
        match self {
            Self::Legacy { bpp, .. } => *bpp,
            Self::Native { bpp, .. } => *bpp,
        }
    }

    /// Get the pixel format of the buffer.
    pub fn format(&self) -> PixelFormat {
        match self {
            Self::Legacy { format, .. } => *format,
            Self::Native { format, .. } => *format,
        }
    }

    /// Write pixels to the buffer.
    pub fn write(&mut self, x: u32, y: u32, width: u32, height: u32, pixels: &[u32]) -> Result<(), ErrorKind> {
        verbose!("Writing buffer. - X: {}, Y: {}, Width: {}, Height: {}", x, y, width, height);

        if pixels.len() != (width * height) as usize {
            error!("Invalid pixel length. - ErrorKind: {:?}", ErrorKind::BUFFER_INVALID_PIXELS);
            return Err(ErrorKind::BUFFER_INVALID_PIXELS);
        }

        else if x + width > self.width() || y + height > self.height() {
            error!("Buffer is out of bounds. - ErrorKind: {:?}", ErrorKind::BUFFER_OUT_OF_BOUNDS);
            return Err(ErrorKind::BUFFER_OUT_OF_BOUNDS);
        }

        match self {
            Self::Legacy { .. } => todo!(),
            Self::Native { .. } => self.write_buffer(x, y, width, height, pixels)?,
        }
        Ok(())
    }

    fn write_buffer(&self, x: u32, y: u32, width: u32, height: u32, pixels: &[u32]) -> Result<(), ErrorKind> {
        let mut map_data = std::ptr::null_mut();
        let bo = self.buffer() as *mut gbm_bo;
        let mut stride = self.stride();

        let dst = unsafe { gbm_bo_map(bo, x, y, width, height, GBM_BO_TRANSFER_WRITE, &mut stride, &mut map_data) };
        if dst == libc::MAP_FAILED {
            error!("Failed to map buffer. - ErrorKind: {:?}", ErrorKind::BUFFER_MAPPING_FAILED);
            return Err(ErrorKind::BUFFER_MAPPING_FAILED);
        }
        let dst = dst as *mut u32;
        
        unsafe {
            for w in 0..width {
                for h in 0..height {
                    let index = (w + h * width) as usize;
                    let pixel = pixels[index];
                    dst.offset((h * (stride / 4) + w).try_into().unwrap()).write(pixel);
                }
            }
            gbm_bo_unmap(bo, map_data);
        }
        Ok(())
    }

    /// Read pixels from the buffer.
    pub fn read(&self, x: u32, y: u32, width: u32, height: u32) -> Result<Vec<u32>, ErrorKind> {
        verbose!("Reading buffer. - X: {}, Y: {}, Width: {}, Height: {}", x, y, width, height);

        if x + width > self.width() || y + height > self.height() {
            error!("Buffer is out of bounds. - ErrorKind: {:?}", ErrorKind::BUFFER_OUT_OF_BOUNDS);
            return Err(ErrorKind::BUFFER_OUT_OF_BOUNDS);
        }

        match self {
            Self::Legacy { .. } => todo!(),
            Self::Native { .. } => self.read_buffer(x, y, width, height),
        }
    }

    fn read_buffer(&self, x: u32, y: u32, width: u32, height: u32) -> Result<Vec<u32>, ErrorKind> {
        let mut map_data = std::ptr::null_mut();
        let bo = self.buffer() as *mut gbm_bo;
        let mut stride = self.stride();

        let src = unsafe { gbm_bo_map(bo, x, y, width, height, GBM_BO_TRANSFER_READ, &mut stride, &mut map_data) };
        let count = (width * height * 4) as usize;
        let mut dst = Vec::with_capacity(count);

        if src == libc::MAP_FAILED {
            error!("Failed to map buffer. - ErrorKind: {:?}", ErrorKind::BUFFER_MAPPING_FAILED);
            return Err(ErrorKind::BUFFER_MAPPING_FAILED);
        }

        unsafe {
            let src = src as *mut u32;
            for w in 0..width {
                for h in 0..height {
                    let index = (w + h * width) as usize;
                    let pixel = src.offset((h * (stride / 4) + w).try_into().unwrap()).read();
                    dst.insert(index, pixel);
                }
            }
            
            gbm_bo_unmap(bo, map_data);
        };
        Ok(dst)
    }

    pub fn clear(&mut self) -> Result<(), ErrorKind> {
        let width = self.width();
        let height = self.height();
        let color = 0x00000000;
        verbose!("Clearing buffer. - Color: {}", color);

        let pixels = vec![color; (width * height) as usize];

        self.write(0, 0, width, height, &pixels)
    }

    fn buffer(&self) -> *mut c_void {
        match self {
            Self::Legacy { buffer, .. } => *buffer,
            Self::Native { buffer, .. } => *buffer as *mut c_void,
        }
    }
    
}
