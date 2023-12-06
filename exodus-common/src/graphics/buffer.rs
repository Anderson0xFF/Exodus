use exodus_errors::ErrorKind;
use gbm::gbm_bo_transfer_flags::GBM_BO_TRANSFER_WRITE;
use gbm::{gbm_bo_format::*, gbm_bo_flags::*};
use gbm::*;
use libc::c_void;


use super::device::native_device::NativeDevice;

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
    pub fn create_native_buffer(width: u32, height: u32, format: PixelFormat, native_device: &NativeDevice) -> Result<Self, ErrorKind> {
        let pixel_format = match format {
            PixelFormat::XRGB8888 => GBM_BO_FORMAT_XRGB8888,
            PixelFormat::ARGB8888 => GBM_BO_FORMAT_ARGB8888,
        };

        let buffer = unsafe {
            gbm_bo_create(native_device.as_ptr(), width, height, pixel_format, GBM_BO_USE_SCANOUT | GBM_BO_USE_RENDERING)
        };

        if buffer.is_null() {
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

    pub fn width(&self) -> u32 {
        match self {
            Self::Legacy { width, .. } => *width,
            Self::Native { width, .. } => *width,
        }
    }

    pub fn height(&self) -> u32 {
        match self {
            Self::Legacy { height, .. } => *height,
            Self::Native { height, .. } => *height,
        }
    }

    pub fn handle(&self) -> u32 {
        match self {
            Self::Legacy { handle, .. } => *handle,
            Self::Native { handle, .. } => *handle,
        }
    }

    pub fn stride(&self) -> u32 {
        match self {
            Self::Legacy { stride, .. } => *stride,
            Self::Native { stride, .. } => *stride,
        }
    }

    pub fn bpp(&self) -> u32 {
        match self {
            Self::Legacy { bpp, .. } => *bpp,
            Self::Native { bpp, .. } => *bpp,
        }
    }

    pub fn format(&self) -> PixelFormat {
        match self {
            Self::Legacy { format, .. } => *format,
            Self::Native { format, .. } => *format,
        }
    }

    pub fn write(&mut self, x: u32, y: u32, width: u32, height: u32, data: &[u32]) -> Result<(), ErrorKind> {
        match self {
            Self::Legacy { .. } => todo!(),
            Self::Native { .. } => self.write_native(x, y, width, height, data),
        }
    }

    fn write_native(&self, x: u32, y: u32, width: u32, height: u32, data: &[u32]) -> Result<(), ErrorKind> {
        let mut stride = self.stride();
        let mut map_data = std::ptr::null_mut();
        let bo = self.buffer() as *mut gbm_bo;
        let pixels = unsafe { gbm_bo_map(bo, x, y, width, height, GBM_BO_TRANSFER_WRITE, &mut stride, &mut map_data) };

        if pixels == libc::MAP_FAILED {
            return Err(ErrorKind::BUFFER_MAPPING_FAILED);
        }

        unsafe { 
            std::ptr::copy_nonoverlapping(data.as_ptr() as *const c_void, pixels, data.len());
            gbm_bo_unmap(bo, map_data);
        };
        Ok(())
    }

    pub fn read(&self, x: u32, y: u32, width: u32, height: u32) -> Result<Vec<u32>, ErrorKind> {
        match self {
            Self::Legacy { .. } => todo!(),
            Self::Native { .. } => self.read_native(x, y, width, height),
        }
    }

    fn read_native(&self, x: u32, y: u32, width: u32, height: u32) -> Result<Vec<u32>, ErrorKind> {
        let mut stride = self.stride();
        let mut map_data = std::ptr::null_mut();
        let bo = self.buffer() as *mut gbm_bo;
        let pixels = unsafe { gbm_bo_map(bo, x, y, width, height, GBM_BO_TRANSFER_WRITE, &mut stride, &mut map_data) };

        if pixels == libc::MAP_FAILED {
            return Err(ErrorKind::BUFFER_MAPPING_FAILED);
        }

        let mut data = Vec::new();
        unsafe {
            data.set_len(width as usize * height as usize);
            std::ptr::copy_nonoverlapping(pixels, data.as_mut_ptr() as *mut c_void, data.len());
            gbm_bo_unmap(bo, map_data);
        };
        Ok(data)
    }

    fn buffer(&self) -> *mut c_void {
        match self {
            Self::Legacy { buffer, .. } => *buffer,
            Self::Native { buffer, .. } => *buffer as *mut c_void,
        }
    }
    
}

#[derive(Debug, Copy, Clone)]
pub enum PixelFormat {
    XRGB8888,
    ARGB8888,
}

impl From<u32> for PixelFormat {
    fn from(format: u32) -> Self {
        match format {
            GBM_BO_FORMAT_XRGB8888 => PixelFormat::XRGB8888,
            GBM_BO_FORMAT_ARGB8888 => PixelFormat::ARGB8888,
            _ => PixelFormat::XRGB8888,
        }
    }
}

impl From<i32> for PixelFormat {
    fn from(format: i32) -> Self {
        match format as u32 {
            GBM_BO_FORMAT_XRGB8888 => PixelFormat::XRGB8888,
            GBM_BO_FORMAT_ARGB8888 => PixelFormat::ARGB8888,
            _ => PixelFormat::XRGB8888,
        }
    }
}

#[derive(Debug, Clone)]
pub enum SubPixel {
    Unknown = 1,
    HorizontalRGB,
    HorizontalBGR,
    VerticalRGB,
    VerticalBGR,
    None,
}

impl From<u32> for SubPixel {
    fn from(subpixel: u32) -> Self {
        match subpixel {
            1 => SubPixel::Unknown,
            2 => SubPixel::HorizontalRGB,
            3 => SubPixel::HorizontalBGR,
            4 => SubPixel::VerticalRGB,
            5 => SubPixel::VerticalBGR,
            6 => SubPixel::None,
            _ => SubPixel::Unknown,
        }
    }
}
