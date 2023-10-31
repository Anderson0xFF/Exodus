#![allow(non_upper_case_globals)]
use std::rc::Rc;

use crate::{errors::ErrorKind, info};

use super::devices::graphics_devices::GraphicDevice;

pub struct Display {
    current_graphic_device: Rc<GraphicDevice>,
    graphics_devices: Vec<Rc<GraphicDevice>>,
}

impl Display {
    pub fn new() -> Result<Self, ErrorKind> {
        info!("Creating display...");
        let graphics_devices = GraphicDevice::detect_graphics_devices()?;
        let current_graphic_device = graphics_devices[0].clone();

        Ok(Self {
            current_graphic_device,
            graphics_devices,
        })
    }

    
}