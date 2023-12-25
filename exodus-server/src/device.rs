#![allow(dead_code)]

use std::fs::OpenOptions;
use std::{fs::File, os::fd::AsRawFd};
use std::os::unix::fs::OpenOptionsExt;
use drm::*;
use exodus_common::consts::DRI_DIRECTORY;
use exodus_common::enums::{Vendor, ScreenFlags};
use exodus_common::graphics::device::{DeviceRef, Device, GPUID};
use exodus_common::*;
use exodus_errors::ErrorKind;
use crate::*;
use crate::screen::Screen;


#[derive(Debug)]
pub struct GPU {
    card:       File,
    vendor:     Vendor,
    model:      u32,
    width:      u32,
    height:     u32,
    device:     Option<DeviceRef>,
    screens:    Vec<Screen>,
}

impl GPU {
    
    fn load(path: &str) -> Result<Self, ErrorKind> {
        info!("Loading gpu: \"{}\"", path);

        let card = OpenOptions::new()
            .read(true)
            .write(true)
            .custom_flags(libc::O_RDWR | libc::O_CLOEXEC | libc::O_NONBLOCK)
            .open(path);

        if let Err(_) = card {
            let err = ErrorKind::GPU_LOAD_FAILED;
            error!("Failed to load gpu: \"{}\" - ErrorKind: {:?}", path, err);
            return Err(err);
        }

        let card = card.unwrap();
        let gpu = card.as_raw_fd();
        let vendor = unsafe { Self::get_card_vendor(gpu) };
        let vendor = Vendor::from(vendor);
        let model = unsafe { Self::get_card_model(gpu) };

        debug!("Getting gpu resources...");
        let resources_ptr: *mut drm::_drmModeRes = unsafe { drmModeGetResources(gpu) };
        if resources_ptr.is_null() {
            let err = ErrorKind::GPU_RESOURCES_FAILED;
            error!("Failed to get gpu resources. - ErrorKind: {:?}", err);
            return Err(err);
        }

        let device = Device::new(gpu)?;
        let resources = unsafe { resources_ptr.as_ref().unwrap() };
        let screens = Screen::enumerate_screens(&device, resources,&[ScreenFlags::DoubleBuffered])?;

        unsafe { drmModeFreeResources(resources_ptr) };

        debug!("GPU loaded successfully. - GPUID: {} - Vendor: {:?} ", gpu, vendor);
        
        Ok(GPU {
            card,
            width: resources.max_width,
            height: resources.max_height,
            device: Some(device),
            vendor,
            screens,
            model,
        })
    }

    #[inline]
    pub fn id(&self) -> i32 {
        self.card.as_raw_fd()
    }

    unsafe fn get_card_vendor(gpu: GPUID) -> u16 {
        let mut device_ptr = std::ptr::null_mut();
        drmGetDevice(gpu, &mut device_ptr);

        let drm_device = device_ptr.as_ref().unwrap() ;
        let vendor_id = drm_device.deviceinfo.pci.as_ref().unwrap().vendor_id;

        drmFreeDevice(&mut device_ptr);
        return vendor_id;
    }

    unsafe fn get_card_model(gpu: GPUID) -> u32 {
        let mut device_ptr = std::ptr::null_mut();
        drmGetDevice(gpu, &mut device_ptr);

        let drm_device = device_ptr.as_ref().unwrap() ;
        let device_id = drm_device.deviceinfo.pci.as_ref().unwrap().device_id;

        drmFreeDevice(&mut device_ptr);
        return device_id as u32;
    }

    pub fn enumerate_gpus() -> Result<Vec<GPU>, ErrorKind> {
        info!("Detecting GPU...");
       
        let dri_directory = std::fs::read_dir(DRI_DIRECTORY);

        if let Err(e) = dri_directory {
            const ERROR: ErrorKind = ErrorKind::GPUS_LIST_FAILED;

            if let std::io::ErrorKind::NotFound = e.kind() {
                error!("Failed to list GPU in \"{}\", directory not found - ErrorKind: {:?}", DRI_DIRECTORY, ERROR);
                return Err(ERROR);
            }
            else if let std::io::ErrorKind::PermissionDenied = e.kind() {
                error!("Failed to list GPU in \"{}\", permission denied - ErrorKind: {:?}", DRI_DIRECTORY, ERROR);
                return Err(ERROR);
            }

            error!("Failed to list GPU in \"{}\" - ErrorKind: {:?}", DRI_DIRECTORY, ERROR);
            return Err(ERROR);
        }

        let cards = dri_directory.unwrap()
            .map(|res| res.map(|e| e.path().to_str().unwrap().to_string()))
            .collect::<Result<Vec<_>, std::io::Error>>().unwrap();

        let mut gpus = Vec::new();
        for path in cards.iter().filter(|x| x.contains("card")) {
            let gpu = GPU::load(path)?;
            info!("GPU detected. - GPUID: {} - Vendor: {:?}", gpu.id(), gpu.vendor);
            gpus.push(gpu);
        }

        if gpus.is_empty() {
            const ERROR: ErrorKind = ErrorKind::GPU_NOT_FOUND;
            error!("No gpus found. - ErrorKind: {:?}", ERROR);
            return Err(ERROR);
        }

        info!("GPUs found: {:?}", gpus.len());
        Ok(gpus)
    }

    pub fn device(&self) -> Option<DeviceRef> {
        self.device.clone()
    }

    pub fn vendor(&self) -> Vendor {
        self.vendor
    }

    pub fn get_screen(&self, id: u32) -> Option<&Screen> {
        self.screens.iter().find(|x| x.id() == id)
    }

    pub fn get_screen_mut(&mut self, id: u32) -> Option<&mut  Screen> {
        self.screens.iter_mut().find(|x| x.id() == id)
    }

    pub fn screens(&self) -> &[Screen] {
        self.screens.as_ref()
    }

    pub fn screens_mut(&mut self) -> &mut Vec<Screen> {
        &mut self.screens
    }

    pub fn model(&self) -> u32 {
        self.model
    }

    pub(crate) fn dispose(&mut self) {
        debug!("Disposing gpu...");
        self.screens.iter_mut().for_each(|screen| screen.dispose());
        debug!("GPU disposed.");
    }
}

impl Drop for GPU {
    fn drop(&mut self) {
        self.dispose();
    }
}