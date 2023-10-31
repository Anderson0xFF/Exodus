use std::fs::OpenOptions;
use std::rc::Rc;
use std::{fs::File, os::fd::AsRawFd};
use std::os::unix::fs::OpenOptionsExt;
use gbm::gbm_create_device;
use libc;
use drm::{drmModeGetResources, drmModeFreeResources};

use crate::info;
use crate::types::GbmDevicePtr;
use crate::{errors::ErrorKind, debug, error, graphics::Size};

use super::connectors::Connector;
use super::crtcs::Crtc;
use super::enconders::Encoder;


#[derive(Debug)]
pub struct GraphicDevice {
    file: File,
    connectors: Vec<Connector>,
    crtcs: Vec<Crtc>,
    encoders: Vec<Encoder>,
    min_resolution: Size<u32>,
    max_resolution: Size<u32>,
    gbm_device: GbmDevicePtr,
}

impl GraphicDevice {
    const DRI_DIRECTORY: &'static str = "/dev/dri/";

    fn load(path: &str) -> Result<Self, ErrorKind> {
        info!("Loading graphic device: \"{}\"", path);

        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .custom_flags(libc::O_RDWR | libc::O_NONBLOCK)
            .open(path);

        if let Err(_) = file {
            let err = ErrorKind::RAMEN_GRAPHIC_DEVICE_LOAD_FAILED;
            error!("Failed to load graphic device: \"{}\" - ErrorKind: {:?}", path, err);
            return Err(err);
        }

        let file = file.unwrap();
        

        debug!("Getting resources...");
        let resources_ptr = unsafe { drmModeGetResources(file.as_raw_fd()) };
        if resources_ptr.is_null() {
            let err = ErrorKind::RAMEN_GRAPHIC_DEVICE_RESOURCES_FAILED;
            error!("Failed to get resources. - ErrorKind: {:?}", err);
            return Err(err);
        }

        let resources = unsafe{ resources_ptr.as_ref().unwrap() };

        debug!("Getting connectors...");
        let mut connectors = Vec::new();
        for i in 0..resources.count_connectors {
            let id = unsafe { *resources.connectors.offset(i as isize).as_ref().unwrap() };
            if let Ok(connector) = Connector::new(file.as_raw_fd(), id) {
                connectors.push(connector);
            }
        }

        debug!("Getting crtcs...");
        let mut crtcs = Vec::new();
        for i in 0..resources.count_crtcs {
            let id = unsafe { *resources.crtcs.offset(i as isize).as_ref().unwrap() };
            if let Ok(crtc) = Crtc::new(file.as_raw_fd(), id) {
                crtcs.push(crtc);
            }
        }

        debug!("Getting encoders...");
        let mut encoders = Vec::new();
        for i in 0..resources.count_encoders {
            let id = unsafe { *resources.encoders.offset(i as isize).as_ref().unwrap() };
            if let Ok(encoder) = Encoder::new(file.as_raw_fd(), id) {
                encoders.push(encoder);
            }
        }

        let min_resolution = Size {
            width: resources.min_width,
            height: resources.min_height,
        };

        let max_resolution = Size {
            width: resources.max_width,
            height: resources.max_height,
        };

        debug!("Creating GBM device...");
        let gbm_device = unsafe { gbm_create_device(file.as_raw_fd()) };

        if gbm_device.is_null() {
            let err = ErrorKind::RAMEN_DISPLAY_GBM_DEVICE_FAILED;
            error!("Failed to create GBM device. - ErrorKind: {:?}", err);
            return Err(err);
        }

        unsafe { drmModeFreeResources(resources_ptr) };
        info!("Graphic device \"{}\" loaded. [OK]", path);

        Ok(GraphicDevice {
            file,
            connectors,
            crtcs,
            encoders,
            min_resolution,
            max_resolution,
            gbm_device,
        })   
    }

    #[inline]
    pub fn id(&self) -> i32 {
        self.file.as_raw_fd()
    }

    pub fn detect_graphics_devices() -> Result<Vec<Rc<GraphicDevice>>, ErrorKind> {
        info!("Detecting graphics devices...");
       
        let dri_directory = std::fs::read_dir(Self::DRI_DIRECTORY);

        if let Err(e) = dri_directory {
            const ERROR: ErrorKind = ErrorKind::RAMEN_GRAPHICS_DEVICES_LIST_FAILED;

            if let std::io::ErrorKind::NotFound = e.kind() {
                error!("Failed to list graphics devices in \"{}\", directory not found - ErrorKind: {:?}", Self::DRI_DIRECTORY, ERROR);
                return Err(ERROR);
            }
            else if let std::io::ErrorKind::PermissionDenied = e.kind() {
                error!("Failed to list graphics devices in \"{}\", permission denied - ErrorKind: {:?}", Self::DRI_DIRECTORY, ERROR);
                return Err(ERROR);
            }

            error!("Failed to list graphics devices in \"{}\" - ErrorKind: {:?}", Self::DRI_DIRECTORY, ERROR);
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
            const ERROR: ErrorKind = ErrorKind::RAMEN_GRAPHICS_DEVICES_NOT_FOUND;
            error!("No graphics devices found. - ErrorKind: {:?}", ERROR);
            return Err(ERROR);
        }

        info!("Graphics devices found: {:?}", graphics_devices.len());
        Ok(graphics_devices)
    }
}