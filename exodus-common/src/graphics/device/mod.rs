#![allow(dead_code)]

pub mod native_device;

use std::fs::OpenOptions;
use std::{fs::File, os::fd::AsRawFd};
use std::os::unix::fs::OpenOptionsExt;
use drm::*;
use exodus_errors::ErrorKind;
use crate::*;
use crate::consts::DRI_DIRECTORY;
use crate::enums::{ScreenFlags, Vendor};
use crate::graphics::device::native_device::Device;
use crate::graphics::screen::connector::Connector;
use self::native_device::DeviceRef;

use super::screen::Screen;

pub type GPUID = i32;

#[derive(Debug)]
pub struct GPU {
    card: File,
    vendor: Vendor,
    width: u32,
    height: u32,
    screens: Vec<Screen>,
    device: Option<DeviceRef>,
}

impl GPU {
    
    fn load(path: &str) -> Result<Self, ErrorKind> {
        info!("Loading GPU: \"{}\"", path);

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


        debug!("Getting gpu resources...");
        let resources_ptr: *mut drm::_drmModeRes = unsafe { drmModeGetResources(gpu) };
        if resources_ptr.is_null() {
            let err = ErrorKind::GPU_RESOURCES_FAILED;
            error!("Failed to get gpu resources. - ErrorKind: {:?}", err);
            return Err(err);
        }

        let resources = unsafe{ resources_ptr.as_ref().unwrap() };

        let device = Device::new(gpu)?;


        info!("Detecting screens...");
        let mut screens = Vec::new();
        for i in 0..resources.count_connectors {
            let connector_id = unsafe { *resources.connectors.offset(i as isize).as_ref().unwrap() };
            if let Ok(Some(connector)) = Connector::new(gpu, connector_id) {
                let screen = Screen::new(device.clone(), connector, &[ScreenFlags::TripleBuffered])?;
                info!("Screen - ID: {} Port: {:?} - Resolution: {}x{}", screen.id(), screen.connector_type(), screen.width(), screen.height());
                screens.push(screen);
            }
        }

        unsafe { drmModeFreeResources(resources_ptr) };
        info!("GPU \"{}\" loaded.", path);

        Ok(GPU {
            card,
            screens,
            width: resources.max_width,
            height: resources.max_height,
            device: Some(device),
            vendor,
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

    pub fn search_gpus() -> Result<Vec<GPU>, ErrorKind> {
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
            gpus.push(GPU::load(path)?);
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

    pub fn screens(&self) -> &[Screen] {
        self.screens.as_ref()
    }

    pub fn get_screen(&mut self, id: u32) -> Option<&mut Screen> {
        self.screens.iter_mut().find(|x| x.id() == id)
    }

    pub fn dispose(&mut self) {
        debug!("Disposing GPU: \"{}\"", self.id());
        self.screens.iter_mut().for_each(|screen| {
            screen.dispose();
        });
    }

}

impl Drop for GPU {
    fn drop(&mut self) {
        self.dispose();
    }
}