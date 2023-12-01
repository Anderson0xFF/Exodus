#![allow(dead_code)]

use std::fs::OpenOptions;
use std::rc::Rc;
use std::{fs::File, os::fd::AsRawFd};
use std::os::unix::fs::OpenOptionsExt;
use gbm::gbm_create_device;
use libc;
use drm::{drmModeGetResources, drmModeFreeResources};
use exodus_errors::ErrorKind;
use crate::*;
use crate::consts::DRI_DIRECTORY;
use crate::types::NativeDevice;

use super::connectors::Screen;

pub type GraphicDeviceRef = Rc<GraphicDevice>;
pub type Device = i32;

#[derive(Debug)]
pub struct GraphicDevice {
    file: File,
    width: u32,
    height: u32,
    screens: Vec<Screen>,
    native_device: NativeDevice,
}

impl GraphicDevice {
    
    fn load(path: &str) -> Result<Self, ErrorKind> {
        info!("Loading graphic device: \"{}\"", path);

        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .custom_flags(libc::O_RDWR | libc::O_CLOEXEC)
            .open(path);

        if let Err(_) = file {
            let err = ErrorKind::EXODUS_GRAPHICS_DEVICES_LOAD_FAILED;
            error!("Failed to load graphic device: \"{}\" - ErrorKind: {:?}", path, err);
            return Err(err);
        }

        let file = file.unwrap();
        

        debug!("Getting resources...");
        let resources_ptr = unsafe { drmModeGetResources(file.as_raw_fd()) };
        if resources_ptr.is_null() {
            let err = ErrorKind::EXODUS_GRAPHIC_DEVICE_RESOURCES_FAILED;
            error!("Failed to get resources. - ErrorKind: {:?}", err);
            return Err(err);
        }

        let resources = unsafe{ resources_ptr.as_ref().unwrap() };

        debug!("Creating GBM device...");
        let native_device = unsafe { gbm_create_device(file.as_raw_fd()) };

        if native_device.is_null() {
            let err = ErrorKind::EXODUS_NATIVE_VISION_GBM_DEVICE_FAILED;
            error!("Failed to create GBM device. - ErrorKind: {:?}", err);
            return Err(err);
        }

        debug!("Getting screens...");
        let mut screens = Vec::new();
        for i in 0..resources.count_connectors {
            let connector_id = unsafe { *resources.connectors.offset(i as isize).as_ref().unwrap() };
            if let Ok(connector) = Screen::new(connector_id, file.as_raw_fd(), native_device) {
                screens.push(connector);
            }
        }

        unsafe { drmModeFreeResources(resources_ptr) };
        info!("Graphic device \"{}\" loaded.", path);

        Ok(GraphicDevice {
            file,
            screens,
            width: resources.max_width,
            height: resources.max_height,
            native_device,
        })   
    }

    #[inline]
    pub fn id(&self) -> i32 {
        self.file.as_raw_fd()
    }

    pub fn search_cards() -> Result<Vec<GraphicDeviceRef>, ErrorKind> {
        info!("Detecting graphics devices...");
       
        let dri_directory = std::fs::read_dir(DRI_DIRECTORY);

        if let Err(e) = dri_directory {
            const ERROR: ErrorKind = ErrorKind::EXODUS_GRAPHICS_DEVICES_LIST_FAILED;

            if let std::io::ErrorKind::NotFound = e.kind() {
                error!("Failed to list graphics devices in \"{}\", directory not found - ErrorKind: {:?}", DRI_DIRECTORY, ERROR);
                return Err(ERROR);
            }
            else if let std::io::ErrorKind::PermissionDenied = e.kind() {
                error!("Failed to list graphics devices in \"{}\", permission denied - ErrorKind: {:?}", DRI_DIRECTORY, ERROR);
                return Err(ERROR);
            }

            error!("Failed to list graphics devices in \"{}\" - ErrorKind: {:?}", DRI_DIRECTORY, ERROR);
            return Err(ERROR);
        }

        let graphics_devices_paths = dri_directory.unwrap()
            .map(|res| res.map(|e| e.path().to_str().unwrap().to_string()))
            .collect::<Result<Vec<_>, std::io::Error>>().unwrap();

        let mut graphics_devices = Vec::new();
        for path in graphics_devices_paths.iter().filter(|x| x.contains("card")) {
            graphics_devices.push(Rc::new(GraphicDevice::load(path)?));
        }

        if graphics_devices.is_empty() {
            const ERROR: ErrorKind = ErrorKind::EXODUS_GRAPHICS_DEVICES_NOT_FOUND;
            error!("No graphics devices found. - ErrorKind: {:?}", ERROR);
            return Err(ERROR);
        }

        info!("Graphics devices found: {:?}", graphics_devices.len());
        Ok(graphics_devices)
    }

    pub fn native_device(&self) -> NativeDevice {
        self.native_device
    }

    pub fn screens(&self) -> &[Screen] {
        self.screens.as_ref()
    }
}