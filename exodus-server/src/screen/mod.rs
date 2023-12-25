#![allow(dead_code)]

mod connector;
mod crtcs;
mod encoders;


use drm::_drmModeRes;
use exodus_common::{graphics::{device::DeviceRef, buffer::Buffer}, enums::*, debug, info};
use exodus_errors::ErrorKind;
use crate::framebuffer::Framebuffer;
use self::{connector::Connector, crtcs::CRTC};

#[derive(Debug)]
pub struct DrawCommand {
    pub x:          u32,
    pub y:          u32,
    pub plane:      u32,
    pub width:      u32,
    pub height:     u32,
    pub pixels:     Vec<u32>,
}

impl DrawCommand {
    pub fn set_x(&mut self, x: u32) {
        self.x = x;
    }

    pub fn y(&self) -> u32 {
        self.y
    }

    pub fn set_plane(&mut self, plane: u32) {
        self.plane = plane;
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
    }

    pub fn set_pixels(&mut self, pixels: Vec<u32>) {
        self.pixels = pixels;
    }
}

#[derive(Debug)]
pub struct Screen {
    device:         DeviceRef,
    mode:           u32,
    index:          usize,
    buffers:        Vec<Buffer>,
    framebuffers:   Vec<Framebuffer>,
    connector:      Connector,
    crtc:           CRTC,
    queue:          Vec<DrawCommand>,
    vsync:          bool,
}

impl Screen {

    pub(crate) fn enumerate_screens(device: &DeviceRef, resources: &_drmModeRes, flags: &[ScreenFlags]) -> Result<Vec<Self>, ErrorKind> 
    {
        debug!("Enumerating screens. - GPU: {} - Flags: {:?}", device.id(), flags);

        let mut screens = Vec::new();

        for i in 0..resources.count_connectors {
            let connector_id = unsafe { *resources.connectors.offset(i as isize).as_ref().unwrap() };
            if let Ok(Some(connector)) = Connector::new(device.id(), connector_id) {
                let screen = Screen::new(device.clone(), connector, &[ScreenFlags::TripleBuffered])?;
                info!("Detected screen. ID: {} - Port: {:?} - Resolution: {}x{}", screen.id(), screen.connector_type(), screen.width(), screen.height());
                screens.push(screen);
            }
        }

        debug!("Screens enumerated successfully. - GPUID: {} - Count: {}", device.id(), screens.len());
        Ok(screens)
    }

    fn new(device: DeviceRef, connector: Connector, flags: &[ScreenFlags]) -> Result<Self, ErrorKind> {
        debug!("Initializing screen. - ConnectorID: {} - GPUID: {} Flags: {:?}", connector.id(), device.id(), flags);

        let crtc_id = connector.encoder().crtc_id();
        let crtc = CRTC::new(device.id(), crtc_id)?;


        let mut mode_id = 0;
        let mut width = 0;
        let mut height = 0;
        let mut refresh = 0;

        if flags.contains(&ScreenFlags::OptimalResolution) 
        {
            debug!("Detecting optimal resolution...");

            for (id, mode) in connector.modes().iter().enumerate() {
                let mode = unsafe { mode.as_ref().unwrap() };
                if mode.hdisplay as u32 > width && mode.vdisplay as u32 > height {
                    width = mode.hdisplay  as u32;
                    height = mode.vdisplay  as u32;
                    mode_id = id as u32;
                    refresh = mode.vrefresh;
                }
            }
        }
        else 
        {
            debug!("Detecting default resolution...");
            let mode = unsafe { connector.get_mode(0).unwrap().as_ref().unwrap() };
            mode_id = 0;
            width = mode.hdisplay as u32;
            height = mode.vdisplay as u32;
            refresh = mode.vrefresh;
        }


        let mut buffer_count = 1;

        for flag in flags {
            match flag {
                ScreenFlags::DoubleBuffered => buffer_count = 2,
                ScreenFlags::TripleBuffered => buffer_count = 3,
                _ => (),
            }
        }


        debug!("Creating buffers...");
        let mut buffers: Vec<Buffer> = Vec::with_capacity(buffer_count);
        let mut framebuffers: Vec<Framebuffer> = Vec::with_capacity(buffer_count);

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
            queue: Vec::with_capacity(128),
            vsync: true,
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

    pub fn refresh(&self) -> u32 {
        let mode = unsafe { self.connector.get_mode(self.mode).unwrap().as_ref().unwrap() };
        mode.vrefresh
    }

    pub fn clear(&mut self) {
        let length = self.buffers.len();
        self.buffers[(self.index + 1) % length].clear().unwrap();
    }

    /// Submit a draw command to the draw queue.
    pub fn submit(&mut self, command: DrawCommand) {
        self.queue.push(command);
    }

    pub fn write(&mut self, x: u32, y: u32, width: u32, height: u32, pixels: &[u32]) -> Result<(), ErrorKind> {
        let length = self.buffers.len();
        let index = (self.index + 1) % length;
        self.buffers[index].write(x, y, width, height, pixels)
    }

    /// Swap the buffers of the screen.
    pub fn swap_buffers(&mut self) -> Result<(), ErrorKind> {
        let index = (self.index + 1) %  self.buffers.len();

        self.queue.sort_by(|x, y| x.plane.cmp(&y.plane));

        for command in self.queue.drain(..) {
            self.buffers[index].write(command.x, command.y, command.width, command.height, &command.pixels)?;
        }

        let framebuffer = &self.framebuffers[self.index];
        let mode = self.connector.get_mode(self.mode).unwrap();

        self.crtc.set(&mut [self.id()], mode, framebuffer)?;

        self.index = (self.index + 1) % self.buffers.len();
        Ok(())
    }

    pub(super) fn dispose(&mut self) {
        debug!("Disposing screen. - ConnectorID: {} - GPUID: {}", self.connector.id(), self.device.id());
        self.crtc.restore(&mut [self.id()])
    }

    pub fn mode(&self) -> u32 {
        self.mode
    }

    pub fn set_vsync(&mut self, vsync: bool) {
        self.vsync = vsync;
    }
}

impl Drop for Screen {
    fn drop(&mut self) {
        self.dispose();
    }
}