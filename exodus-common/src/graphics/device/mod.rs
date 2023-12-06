#![allow(dead_code)]

pub mod native_device;

use std::fs::OpenOptions;
use std::{fs::File, os::fd::AsRawFd};
use std::os::unix::fs::OpenOptionsExt;
use drm::{drmModeGetResources, drmModeFreeResources};
use exodus_errors::ErrorKind;
use crate::*;
use crate::consts::DRI_DIRECTORY;
use crate::graphics::device::native_device::NativeDevice;
use self::native_device::NativeDeviceRef;

use super::screen::Screen;

pub type GPUID = i32;

#[derive(Debug)]
pub struct GPU {
    file: File,
    width: u32,
    height: u32,
    screens: Vec<Screen>,
    native_device: NativeDeviceRef,
}

impl GPU {
    
    fn load(path: &str) -> Result<Self, ErrorKind> {
        info!("Loading gpu: \"{}\"", path);

        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .custom_flags(libc::O_RDWR | libc::O_CLOEXEC)
            .open(path);

        if let Err(_) = file {
            let err = ErrorKind::GPU_LOAD_FAILED;
            error!("Failed to load gpu: \"{}\" - ErrorKind: {:?}", path, err);
            return Err(err);
        }

        let file = file.unwrap();
        
        debug!("Getting gpu resources...");
        let resources_ptr = unsafe { drmModeGetResources(file.as_raw_fd()) };
        if resources_ptr.is_null() {
            let err = ErrorKind::GPU_RESOURCES_FAILED;
            error!("Failed to get gpu resources. - ErrorKind: {:?}", err);
            return Err(err);
        }

        let resources = unsafe{ resources_ptr.as_ref().unwrap() };

        debug!("Creating NativeDevice...");
        let native_device = NativeDevice::new(file.as_raw_fd())?;

        info!("Detecting screens...");
        let mut screens = Vec::new();
        for i in 0..resources.count_connectors {
            let connector_id = unsafe { *resources.connectors.offset(i as isize).as_ref().unwrap() };
            if let Ok(screen) = Screen::new(connector_id, file.as_raw_fd(), native_device.clone()) {
                info!("Screen {}x{}", screen.width(), screen.height());
                screens.push(screen);
            }
        }

        unsafe { drmModeFreeResources(resources_ptr) };
        info!("Graphic device \"{}\" loaded.", path);

        Ok(GPU {
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

        let graphics_devices_paths = dri_directory.unwrap()
            .map(|res| res.map(|e| e.path().to_str().unwrap().to_string()))
            .collect::<Result<Vec<_>, std::io::Error>>().unwrap();

        let mut gpus = Vec::new();
        for path in graphics_devices_paths.iter().filter(|x| x.contains("card")) {
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

    pub fn native_device(&self) -> NativeDeviceRef {
        self.native_device.clone()
    }

    pub fn screens(&self) -> &[Screen] {
        self.screens.as_ref()
    }

    pub fn get_screen_from_id(&mut self, id: u32) -> Option<&mut Screen> {
        self.screens.iter_mut().find(|x| x.id() == id)
    }

    pub fn get_screen_from_index(&mut self, index: usize) -> Option<&mut Screen> {
        self.screens.get_mut(index)
    }

}

impl Drop for GPU {
    fn drop(&mut self) {
        for screen in self.screens.iter_mut() {
            screen.shutdown();
        }
    }
}