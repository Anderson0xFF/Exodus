#![allow(dead_code)]

use exodus_errors::ErrorKind;
use crate::{debug, enums::*};
use self::connector::Connector;
use super::{device::native_device::DeviceRef, buffer::Buffer, framebuffer::Framebuffer, crtcs::CRTC};

pub mod connector;

#[derive(Debug)]
pub struct Screen {
    device: DeviceRef,
    mode: u32,
    index: usize,
    buffers: Vec<Buffer>,
    framebuffers: Vec<Framebuffer>,
    connector: Connector,
    crtc: CRTC,
}

impl Screen {
    pub(crate) fn new(device: DeviceRef, connector: Connector, flags: &[ScreenFlags]) -> Result<Self, ErrorKind> {
        debug!("Initializing screen. - ConnectorID: {} - GPUID: {} Flags: {:?}", connector.id(), device.id(), flags);

        let crtc_id = connector.encoder().crtc_id();
        let crtc = CRTC::new(device.id(), crtc_id)?;

        let mut width = 0;
        let mut height = 0;
        let mut refresh = 0;
        let mut mode_id = 0;

        debug!("Detecting optimal resolution...");
        for (id, mode) in connector.modes().iter().enumerate() {
            let mode = unsafe { mode.as_ref().unwrap() };
            if mode.hdisplay as u32 > width && mode.vdisplay as u32 > height {
                width = mode.hdisplay  as u32;
                height = mode.vdisplay  as u32;
                refresh = mode.vrefresh;
                mode_id = id;
            }
        }

        let mut buffers: Vec<Buffer> = Vec::new();
        let mut framebuffers: Vec<Framebuffer> = Vec::new();
        let mut buffer_count = 1;

        for flag in flags {
            match flag {
                ScreenFlags::DoubleBuffered => buffer_count = 2,
                ScreenFlags::TripleBuffered => buffer_count = 3,
            }
        }

        debug!("Creating buffers...");
        const FLAGS: [BufferFlag; 2] = [BufferFlag::Scanout, BufferFlag::Rendering];
        for _ in 0..buffer_count {
            let buffer = Buffer::new(&device, width, height, PixelFormat::ARGB8888, &FLAGS)?;
            let framebuffer = Framebuffer::new(device.id(), &buffer)?;
            buffers.push(buffer);
            framebuffers.push(framebuffer);
        }

        debug!("Screen initialized. - Id: {} - GPUID: {} - Width: {} - Height: {} - Refresh: {} ", connector.id(), device.id(), width, height, refresh);

        Ok(Self {
            device,
            index: 0,
            buffers,
            framebuffers,
            connector,
            mode: mode_id as u32,
            crtc,
        })
    }

    pub fn id(&self) -> u32 {
        self.connector.id()
    }

    pub fn connector_type(&self) -> ConnectorType {
        self.connector.connector_type()
    }

    #[allow(non_snake_case)]
    pub fn mmWidth(&self) -> u32 {
        self.connector.mmWidth()
    }

    #[allow(non_snake_case)]
    pub fn mmHeight(&self) -> u32 {
        self.connector.mmHeight()
    }

    pub fn subpixel(&self) -> u32 {
        self.connector.subpixel()
    }

    pub fn width(&self) -> u32 {
        let mode = unsafe { self.connector.get_mode(self.mode).unwrap().as_ref().unwrap() };
        mode.hdisplay as u32
    }

    pub fn height(&self) -> u32 {
        let mode = unsafe { self.connector.get_mode(self.mode).unwrap().as_ref().unwrap() };
        mode.vdisplay as u32
    }

    pub fn write(&mut self, x: u32, y: u32, width: u32, height: u32, pixels: &[u32]) -> Result<(), ErrorKind> {
        let index = (self.index + 1) % self.framebuffers.len();
        self.buffers[index].write(x, y, width, height, pixels)
    }

    pub fn swap_buffers(&mut self) -> Result<(), ErrorKind> {
        let framebuffer = &self.framebuffers[self.index];
        let mode = self.connector.get_mode(self.mode).unwrap();

        self.crtc.set(&mut [self.id()], mode, framebuffer)?;
        self.index = (self.index + 1) % self.framebuffers.len();
        Ok(())
    }

    pub(crate) fn dispose(&mut self) {
        debug!("Disposing screen. - ConnectorID: {} - GPUID: {}", self.connector.id(), self.device.id());
        self.crtc.restore(&mut [self.id()])
    }

}

impl Drop for Screen {
    fn drop(&mut self) {
        debug!("Dropping screen. - ConnectorID: {} - GPUID: {}", self.connector.id(), self.device.id());
        self.crtc.restore(&mut [self.id()])
    }
}