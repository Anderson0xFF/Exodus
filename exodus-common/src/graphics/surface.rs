#![allow(unused_imports)]
#![allow(dead_code)]

use exodus_errors::ErrorKind;
use gbm::{
    gbm_bo_flags::{GBM_BO_USE_RENDERING, GBM_BO_USE_SCANOUT, GBM_BO_USE_CURSOR},
    gbm_bo_format::{GBM_BO_FORMAT_ARGB8888, GBM_BO_FORMAT_XRGB8888}, gbm_bo_transfer_flags::GBM_BO_TRANSFER_READ,
};

use crate::{error, types::NativeDevice};
use super::connectors::Screen;

#[derive(Debug)]
pub struct Surface {
    width: u32,
    height: u32,
    format: SurfaceFormat,
    surface_type: SurfaceType,
    data: *mut gbm::gbm_surface,
}

impl Surface {
    pub fn new(
        native_device: NativeDevice,
        width: u32,
        height: u32,
        format: SurfaceFormat,
        surface_type: SurfaceType,
        screen: &Screen,
    ) -> Result<Self, ErrorKind> {
        let format_u32 = match format {
            SurfaceFormat::XRGB8888 => GBM_BO_FORMAT_XRGB8888,
            SurfaceFormat::ARGB8888 => GBM_BO_FORMAT_ARGB8888,
        };

        let flags = match surface_type {
            SurfaceType::Window     => GBM_BO_USE_SCANOUT       | GBM_BO_USE_RENDERING,
            SurfaceType::Cursor     => GBM_BO_USE_SCANOUT       | GBM_BO_USE_CURSOR     | GBM_BO_USE_RENDERING,
            SurfaceType::Overlay    => GBM_BO_USE_SCANOUT       | GBM_BO_USE_RENDERING,
            SurfaceType::Background => GBM_BO_USE_SCANOUT       | GBM_BO_USE_RENDERING,
        };

        let data: *mut gbm::gbm_surface = unsafe {
            gbm::gbm_surface_create(
                native_device,
                screen.width(),
                screen.height(),
                format_u32,
                flags,
            )
        };

        let surface = Surface { width, height, format, surface_type, data};
        Ok(surface)
    }

    pub fn get_data(&self) -> *mut gbm::gbm_surface {
        self.data
    }

    pub fn get_format(&self) -> SurfaceFormat {
        self.format
    }

    pub fn get_type(&self) -> SurfaceType {
        self.surface_type
    }

    pub fn lock(&self) -> Result<SurfaceLock, ErrorKind> {
        let data = unsafe { gbm::gbm_surface_lock_front_buffer(self.data) };
        if data.is_null() {
            return Err(ErrorKind::EXODUS_SURFACE_LOCK_FAILED);
        }

        let surface_lock = SurfaceLock { width: self.width, height: self.height, data };
        Ok(surface_lock)
    }
}

pub struct SurfaceLock {
    width: u32,
    height: u32,
    data: *mut gbm::gbm_bo,
}

impl SurfaceLock {

    #[inline]
    pub fn handle(&self) -> u32 {
        unsafe { gbm::gbm_bo_get_handle(self.data).u32_ }
    }

    #[inline]
    pub fn stride(&self) -> u32 {
        unsafe { gbm::gbm_bo_get_stride(self.data) }
    }

    #[inline]
    pub fn width(&self) -> u32 {
        self.width
    }

    #[inline]
    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn format(&self) -> SurfaceFormat {
        let format = unsafe { gbm::gbm_bo_get_format(self.data) };
        let format = SurfaceFormat::from_u32(format).unwrap();
        format
    }

    pub fn pixels(&self) -> Result<Vec<u32>, ErrorKind> {
        let width = self.width();
        let height = self.height();
        let mut stride = self.stride();

        let mut pixels = Vec::new();
        let mut map_data = std::ptr::null_mut();
        let addr = unsafe { gbm::gbm_bo_map(self.data, 0, 0, width, height, GBM_BO_TRANSFER_READ, &mut stride, &mut map_data) };

        if addr == libc::MAP_FAILED {
            return Err(ErrorKind::EXODUS_SURFACE_MAP_FAILED);
        }

        let pixel_ptr = addr as *const u32;
        let pixel_slice = unsafe { std::slice::from_raw_parts(pixel_ptr, (width * height) as usize) };
        pixels.extend_from_slice(pixel_slice);

        unsafe { gbm::gbm_bo_unmap(self.data, map_data) };

        Ok(pixels)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum SurfaceFormat {
    XRGB8888 = GBM_BO_FORMAT_XRGB8888 as isize,
    ARGB8888 = GBM_BO_FORMAT_ARGB8888 as isize,
}

impl SurfaceFormat {
    pub fn from_u32(format: u32) -> Result<Self, ErrorKind> {
        match format {
            GBM_BO_FORMAT_XRGB8888 => Ok(SurfaceFormat::XRGB8888),
            GBM_BO_FORMAT_ARGB8888 => Ok(SurfaceFormat::ARGB8888),
            _ => Err(ErrorKind::EXODUS_SURFACE_FORMAT_UNKNOWN),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum SurfaceType {
    Window = 1,
    Cursor,
    Overlay,
    Background,
}

#[derive(Debug, Copy, Clone)]
pub enum SurfaceTransform {
    Normal = 0,
    Rotate90,
    Rotate180,
    Rotate270,
    FlipHorizontal,
    FlipVertical,
    Rotate90FlipHorizontal,
    Rotate90FlipVertical,
}
