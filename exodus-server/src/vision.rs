#![allow(non_upper_case_globals)]
use exodus_common::{
    consts::*,
    graphics::device::{GraphicDevice, GraphicDeviceRef},
    net::connection::Connection,
    *,
};
use exodus_errors::ErrorKind;
use std::{os::unix::net::UnixListener, path};

use crate::client::VisionActivity;

pub struct NativeVision {
    id: i32,
    graphic_device: GraphicDeviceRef,
    graphics_devices: Vec<GraphicDeviceRef>,
    listener: UnixListener,
}

impl NativeVision {
    pub fn new() -> Result<Self, ErrorKind> {
        if !path::Path::new(EXODUS_DIRECTORY).exists() {
            std::fs::create_dir_all(EXODUS_DIRECTORY).unwrap();
            std::fs::create_dir_all(EXODUS_LOG_DIRECTORY).unwrap();
        }

        let (id, dpy) = Self::gen_display();
        if let Err(_) = std::env::var(EXODUS_DISPLAY) {
            std::env::set_var(EXODUS_DISPLAY, id.to_string());
        }

        Self::logger_initialize(id);

        info!("Initializing display...");

        let listener: UnixListener = Self::create_display_listener(&dpy)?;
        let graphics_devices = GraphicDevice::search_cards()?;
        let graphic_device = graphics_devices[0].clone();

        info!("Display initialized successfully.");

        Ok(Self {
            id,
            graphic_device,
            graphics_devices,
            listener,
        })
    }

    fn logger_initialize(id: i32) {
        let logger_dir = format!("{}/exodus-{}.log", EXODUS_LOG_DIRECTORY, id);
        if let Ok(level) = std::env::var(EXODUS_LOG) {
            let level = match level.parse::<i32>() {
                Ok(lvl) => logger::Level::from(lvl),
                Err(_) => logger::Level::Error,
            };

            logger::Logger::init(level, logger_dir.as_str());
        } else {
            logger::Logger::init(logger::Level::Info, logger_dir.as_str());
        }
    }

    pub fn accept(&self) -> Option<VisionActivity> {
        match self.listener.accept() {
            Ok((stream, _)) => {
                stream.set_nonblocking(true).unwrap();
                let conn = Connection::new(stream);
                Some(VisionActivity::new(conn, self))
            }
            Err(_) => None,
        }
    }

    fn create_display_listener(path: &str) -> Result<UnixListener, ErrorKind> {
        if let Ok(listener) = UnixListener::bind(path) {
            listener.set_nonblocking(true).unwrap();
            return Ok(listener);
        }

        let err = ErrorKind::EXODUS_NATIVE_VISION_CREATE_FAILED;
        error!("Failed to creating display. - ErrorKind: {:?}", err);
        Err(err)
    }

    fn gen_display() -> (i32, String) {
        let mut i = 0;
        loop {
            let path = format!("{}/exodus-{i}", EXODUS_DIRECTORY);
            if !path::Path::new(&path).exists() {
                break (i, path);
            }
            i += 1;
        }
    }

    #[inline]
    pub fn id(&self) -> i32 {
        self.id
    }

    #[inline]
    pub fn get_graphic_device(&self) -> GraphicDeviceRef {
        self.graphic_device.clone()
    }
    
    #[inline]
    pub fn graphics_devices(&self) -> &[GraphicDeviceRef] {
        self.graphics_devices.as_ref()
    }
}

impl Drop for NativeVision {
    fn drop(&mut self) {
        if let Ok(id) = std::env::var(EXODUS_DISPLAY) {
            if id == self.id.to_string() {
                std::env::remove_var(EXODUS_DISPLAY);
            }
        }

        let display = format!("{}/exodus-{}", EXODUS_DIRECTORY, self.id);
        if path::Path::new(&display).exists() {
            std::fs::remove_file(&display).unwrap();
        }
    }
}
