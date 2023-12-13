#![allow(unused_imports)]
#![allow(dead_code)]

use exodus_errors::ErrorKind;
use gbm::{
    gbm_bo_flags::{GBM_BO_USE_RENDERING, GBM_BO_USE_SCANOUT, GBM_BO_USE_CURSOR, GBM_BO_USE_PROTECTED, GBM_BO_USE_LINEAR},
    gbm_bo_format::{GBM_BO_FORMAT_ARGB8888, GBM_BO_FORMAT_XRGB8888}, gbm_bo_transfer_flags::{GBM_BO_TRANSFER_READ, GBM_BO_TRANSFER_WRITE}, gbm_bo_map, gbm_bo_unmap, gbm_surface_create, gbm_surface_lock_front_buffer,
};
use libc::c_void;

use crate::{enums::{BufferFlag, PixelFormat}, verbose, error, debug};
use super::device::{Device, DeviceRef};

#[derive(Debug)]
pub struct Surface {
    width: u32,
    height: u32,
    format: PixelFormat,
    surface: *mut gbm::gbm_surface,
    device: DeviceRef,
}

impl Surface {
    pub fn new(width: u32, height: u32, format: PixelFormat, surface_flags: &[BufferFlag], device: DeviceRef) -> Result<Self, ErrorKind> {
        debug!("Creating surface. - Width: {}, Height: {}, Format: {:?}, Flags: {:?}", width, height, format, surface_flags);
        
        let mut flags = 0;

        for flag in surface_flags {
            match flag {
                BufferFlag::Cursor      => flags |= GBM_BO_USE_CURSOR,
                BufferFlag::Linear      => flags |= GBM_BO_USE_LINEAR,
                BufferFlag::Protected   => flags |= GBM_BO_USE_PROTECTED,
                BufferFlag::Rendering   => flags |= GBM_BO_USE_RENDERING,
                BufferFlag::Scanout     => flags |= GBM_BO_USE_SCANOUT,
            }
        }

        let surface = unsafe { gbm_surface_create(device.as_ptr(), width, height, format as u32, flags) };
        if surface.is_null() {
            error!("Failed to create surface. - ErrorKind: {:?}", ErrorKind::SURFACE_CREATE_FAILED);
            return Err(ErrorKind::SURFACE_CREATE_FAILED);
        }
        
        let surface = Surface { width, height, format, surface, device };
        Ok(surface)
    }

    pub fn get_data(&self) -> *mut gbm::gbm_surface {
        self.surface
    }

    pub fn get_format(&self) -> PixelFormat {
        self.format
    }

    pub fn lock(&self) -> Result<SurfaceLock, ErrorKind> {
        verbose!("Locking surface.");

        let data = unsafe { gbm_surface_lock_front_buffer(self.surface) };

        if data.is_null() {
            error!("Failed to lock surface. - ErrorKind: {:?}", ErrorKind::SURFACE_LOCK_FAILED);
            return Err(ErrorKind::SURFACE_LOCK_FAILED);
        }

        let surface_lock = SurfaceLock { width: self.width, height: self.height, buffer: data };
        Ok(surface_lock)
    }

    pub fn as_ptr(&self) -> *mut gbm::gbm_surface {
        self.surface
    }
}

pub struct SurfaceLock {
    width: u32,
    height: u32,
    buffer: *mut gbm::gbm_bo,
}

impl SurfaceLock {

    #[inline]
    pub fn handle(&self) -> u32 {
        unsafe { gbm::gbm_bo_get_handle(self.buffer).u32_ }
    }

    #[inline]
    pub fn stride(&self) -> u32 {
        unsafe { gbm::gbm_bo_get_stride(self.buffer) }
    }

    #[inline]
    pub fn width(&self) -> u32 {
        self.width
    }

    #[inline]
    pub fn height(&self) -> u32 {
        self.height
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

        let mut map_data = std::ptr::null_mut();
        let bo = self.buffer;
        let mut stride = self.stride();

        let src = pixels.as_ptr() as *const c_void;
        let dst = unsafe { gbm_bo_map(bo, x, y, width, height, GBM_BO_TRANSFER_WRITE, &mut stride, &mut map_data) };
        let count = (width * height * 4) as usize;

        if dst == libc::MAP_FAILED {
            error!("Failed to map buffer. - ErrorKind: {:?}", ErrorKind::BUFFER_MAPPING_FAILED);
            return Err(ErrorKind::BUFFER_MAPPING_FAILED);
        }

        unsafe { 
            std::ptr::copy_nonoverlapping(src, dst, count);
            gbm_bo_unmap(bo, map_data);
        };
        Ok(())
    }

    /// Read pixels from the buffer.
    pub fn read(&mut self, x: u32, y: u32, width: u32, height: u32) -> Result<Vec<u32>, ErrorKind> {
        verbose!("Reading buffer. - X: {}, Y: {}, Width: {}, Height: {}", x, y, width, height);

        if x + width > self.width() || y + height > self.height() {
            error!("Buffer is out of bounds. - ErrorKind: {:?}", ErrorKind::BUFFER_OUT_OF_BOUNDS);
            return Err(ErrorKind::BUFFER_OUT_OF_BOUNDS);
        }

        let mut map_data = std::ptr::null_mut();
        let bo = self.buffer;
        let mut stride = self.stride();

        let src = unsafe { gbm_bo_map(bo, x, y, width, height, GBM_BO_TRANSFER_READ, &mut stride, &mut map_data) };
        let count = (width * height * 4) as usize;
        let mut dst = Vec::with_capacity(count);

        if src == libc::MAP_FAILED {
            error!("Failed to map buffer. - ErrorKind: {:?}", ErrorKind::BUFFER_MAPPING_FAILED);
            return Err(ErrorKind::BUFFER_MAPPING_FAILED);
        }

        unsafe {
            std::ptr::copy_nonoverlapping(src, dst.as_mut_ptr() as *mut c_void, count);
            gbm_bo_unmap(bo, map_data);
        };
        Ok(dst)
    }

    pub fn pixels(&self) -> Result<Vec<u32>, ErrorKind> {
        let mut stride = self.stride();
        let mut map_data = std::ptr::null_mut();
        let pixels_ptr = unsafe { gbm_bo_map(self.buffer, 0, 0, self.width, self.height, GBM_BO_TRANSFER_READ, &mut stride, &mut map_data) };

        if pixels_ptr == libc::MAP_FAILED {
            return Err(ErrorKind::SURFACE_LOCK_MAPPING_FAILED);
        }

        let pixels = unsafe { std::slice::from_raw_parts(pixels_ptr as *const u32, (self.width * self.height) as usize) };
        unsafe { 
            gbm_bo_unmap(self.buffer, map_data);
        };
        Ok(pixels.to_vec())
    }
}