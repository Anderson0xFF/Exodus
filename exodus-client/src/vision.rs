#![allow(dead_code)]
use exodus_common::types::NativeDevice;

#[derive(Debug)]
pub struct Screen {
    id: u32,
    width: u32,
    height: u32,
}

impl Screen {
    pub fn new(id: u32, width: u32, height: u32) -> Self { Self { id, width, height } }
}

#[derive(Debug)]
pub struct NativeVision {
    id: i32,
    native_device: NativeDevice,
}

impl NativeVision {
    #[inline]
    pub(crate) fn new(id: i32, device: i32) -> Self {
        Self {
            id,
            native_device: unsafe {
                gbm::gbm_create_device(device)
            },
        }
    }

    #[inline]
    pub fn get_id(&self) -> i32 {
        self.id
    }

    #[inline]
    pub fn get_native_device(&self) -> NativeDevice {
        self.native_device
    }
}

impl Drop for NativeVision {
    fn drop(&mut self) {
        unsafe {
            gbm::gbm_device_destroy(self.native_device);
        }
    }
}