use exodus_errors::ErrorKind;
use gbm::gbm_create_device;
use std::sync::Arc;

use crate::{debug, error};

use super::GPUID;

pub type DeviceRef = Arc<Device>;
pub type NativeBufferRaw = *mut gbm::gbm_bo;
pub type NativeSurfaceRaw = *mut gbm::gbm_surface;


#[derive(Debug)]
pub struct Device {
    id : GPUID,
    device: *mut gbm::gbm_device,
}

impl Device {
    pub fn new(id: GPUID) -> Result<DeviceRef, ErrorKind> {
        debug!("Creating DeviceManager. - GPUID: {}", id);

        let device = unsafe {
            gbm_create_device(id)
        };

        if device.is_null() {
            error!("Failed to create gbm_device. - GPUID: {}", id);
            return Err(ErrorKind::DEVICE_MANAGER_CREATE_FAILED);
        }

        Ok(DeviceRef::new(Device {
            device,
            id,
        }))
    }

    pub(crate) fn as_ptr(&self) -> *mut gbm::gbm_device {
        self.device
    }

    pub fn id(&self) -> i32 {
        self.id
    }
}

unsafe impl Send for Device {}
unsafe impl Sync for Device {}