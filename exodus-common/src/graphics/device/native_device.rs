use exodus_errors::ErrorKind;
use gbm::gbm_create_device;
use std::sync::Arc;

pub type NativeDeviceRef = Arc<NativeDevice>;
pub type NativeBufferRaw = *mut gbm::gbm_bo;
pub type NativeSurfaceRaw = *mut gbm::gbm_surface;


#[derive(Debug)]
pub struct NativeDevice {
    device: *mut gbm::gbm_device,
}

impl NativeDevice {
    pub fn new(id: i32) -> Result<NativeDeviceRef, ErrorKind> {
        let device = unsafe {
            gbm_create_device(id)
        };

        if device.is_null() {
            return Err(ErrorKind::NATIVE_DEVICE_CREATE_FAILED);
        }

        Ok(NativeDeviceRef::new(NativeDevice {
            device,
        }))
    }

    pub(crate) fn as_ptr(&self) -> *mut gbm::gbm_device {
        self.device
    }
}

unsafe impl Send for NativeDevice {}
unsafe impl Sync for NativeDevice {}