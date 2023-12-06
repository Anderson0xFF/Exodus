#![allow(dead_code)]

use drm::{drmModeConnection::DRM_MODE_CONNECTED, *};
use exodus_errors::ErrorKind;
use crate::{debug, error, graphics::encoders::Encoder};
use super::{crtcs::Crtc, buffer::{Buffer, PixelFormat}, device::{DeviceID, native_device::NativeDeviceRef}};

pub type Modes = Vec<drmModeModeInfo>;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum ConnectorType {
    Unknown,
    HDMIA = 1,
    HDMIB,
    TV,
    DVII,
    DVID,
    DVIA,
    VGA,
    DISPLAY_PORT,
    eDP,
    VIRTUAL,
    DSI,
    DPI,
    WRITEBACK,
    SPI,
    LVDS,
    COMPOSITE,
    SVIDEO,
    COMPONENT,
    NINE_PIN_DIN,
    USB,
}

#[allow(non_upper_case_globals)]
impl From<u32> for ConnectorType {
    fn from(connector_type: u32) -> Self {
        match connector_type {
            DRM_MODE_CONNECTOR_HDMIA => ConnectorType::HDMIA,
            DRM_MODE_CONNECTOR_HDMIB => ConnectorType::HDMIB,
            DRM_MODE_CONNECTOR_TV => ConnectorType::TV,
            DRM_MODE_CONNECTOR_DVII => ConnectorType::DVII,
            DRM_MODE_CONNECTOR_DVID => ConnectorType::DVID,
            DRM_MODE_CONNECTOR_DVIA => ConnectorType::DVIA,
            DRM_MODE_CONNECTOR_VGA => ConnectorType::VGA,
            DRM_MODE_CONNECTOR_DisplayPort => ConnectorType::DISPLAY_PORT,
            DRM_MODE_CONNECTOR_eDP => ConnectorType::eDP,
            DRM_MODE_CONNECTOR_VIRTUAL => ConnectorType::VIRTUAL,
            DRM_MODE_CONNECTOR_DSI => ConnectorType::DSI,
            DRM_MODE_CONNECTOR_DPI => ConnectorType::DPI,
            DRM_MODE_CONNECTOR_WRITEBACK => ConnectorType::WRITEBACK,
            DRM_MODE_CONNECTOR_SPI => ConnectorType::SPI,
            DRM_MODE_CONNECTOR_LVDS => ConnectorType::LVDS,
            DRM_MODE_CONNECTOR_Composite => ConnectorType::COMPOSITE,
            DRM_MODE_CONNECTOR_SVIDEO => ConnectorType::SVIDEO,
            DRM_MODE_CONNECTOR_Component => ConnectorType::COMPONENT,
            DRM_MODE_CONNECTOR_9PinDIN => ConnectorType::NINE_PIN_DIN,
            DRM_MODE_CONNECTOR_USB => ConnectorType::USB,
            _ => ConnectorType::Unknown,
        }
    }
}

#[allow(non_upper_case_globals)]
impl From<i32> for ConnectorType {
    fn from(connector_type: i32) -> Self {
        match connector_type as u32 {
            DRM_MODE_CONNECTOR_HDMIA => ConnectorType::HDMIA,
            DRM_MODE_CONNECTOR_HDMIB => ConnectorType::HDMIB,
            DRM_MODE_CONNECTOR_TV => ConnectorType::TV,
            DRM_MODE_CONNECTOR_DVII => ConnectorType::DVII,
            DRM_MODE_CONNECTOR_DVID => ConnectorType::DVID,
            DRM_MODE_CONNECTOR_DVIA => ConnectorType::DVIA,
            DRM_MODE_CONNECTOR_VGA => ConnectorType::VGA,
            DRM_MODE_CONNECTOR_DisplayPort => ConnectorType::DISPLAY_PORT,
            DRM_MODE_CONNECTOR_eDP => ConnectorType::eDP,
            DRM_MODE_CONNECTOR_VIRTUAL => ConnectorType::VIRTUAL,
            DRM_MODE_CONNECTOR_DSI => ConnectorType::DSI,
            DRM_MODE_CONNECTOR_DPI => ConnectorType::DPI,
            DRM_MODE_CONNECTOR_WRITEBACK => ConnectorType::WRITEBACK,
            DRM_MODE_CONNECTOR_SPI => ConnectorType::SPI,
            DRM_MODE_CONNECTOR_LVDS => ConnectorType::LVDS,
            DRM_MODE_CONNECTOR_Composite => ConnectorType::COMPOSITE,
            DRM_MODE_CONNECTOR_SVIDEO => ConnectorType::SVIDEO,
            DRM_MODE_CONNECTOR_Component => ConnectorType::COMPONENT,
            DRM_MODE_CONNECTOR_9PinDIN => ConnectorType::NINE_PIN_DIN,
            DRM_MODE_CONNECTOR_USB => ConnectorType::USB,
            _ => ConnectorType::Unknown,
        }
    }
}


#[allow(non_snake_case)]
#[derive(Debug)]
pub struct Screen {
    id: u32,
    connector_type: ConnectorType,
    mmWidth: u32,
    mmHeight: u32,
    subpixel: u32,
    mode: drmModeModeInfo,
    modes: Modes,
    encoder: Encoder,
    crtc: Crtc,
    buffers: [Buffer; 2],
    buffer_index: usize,
    connector: *mut _drmModeConnector,
    device: DeviceID,
}

impl Screen {
    pub(crate) fn new(connector_id: u32, device: DeviceID, native_device: NativeDeviceRef) -> Result<Self, ErrorKind> 
    {   
        let connector: *mut _drmModeConnector = unsafe { Self::get_connector(device, connector_id)? };
        let modes: Vec<_drmModeModeInfo> = unsafe { Self::get_modes(connector)? };
        let mode = modes[0];
        let encoder = unsafe { Encoder::new((*connector).encoder_id, device)? };
        let crtc = Crtc::new(encoder.crtc_id(), device)?;
        let pixel_format = PixelFormat::XRGB8888;
        let buffers = [
            Buffer::create_native_buffer(mode.hdisplay as u32, mode.vdisplay as u32, pixel_format, &native_device)?,
            Buffer::create_native_buffer(mode.hdisplay as u32, mode.vdisplay as u32, pixel_format, &native_device)?,
        ];
        
        unsafe {
            Ok(Self {
                id: (*connector).connector_id,
                connector_type: (*connector).connector_type.into(),
                mmWidth: (*connector).mmWidth,
                mmHeight: (*connector).mmHeight,
                subpixel: (*connector).subpixel,
                mode,
                modes,
                encoder,
                crtc,
                buffers,
                buffer_index: 0,
                connector,
                device,
            })
        }
    }

    unsafe fn get_connector(device: DeviceID, connector_id: u32) -> Result<*mut _drmModeConnector, ErrorKind> {
        debug!("Getting connector");
        let connector_ptr = drmModeGetConnector(device, connector_id);

        if connector_ptr.is_null() {
            let err = ErrorKind::SCREEN_CONNECTOR_FAILED;
            debug!("Failed to get connector, connector is null. - DeviceID: {}, ConnectorID: {}", device, connector_id);
            error!("Failed to get connector. - ErrorKind: {:?}", err);
            return Err(err);
        }

        let connector = connector_ptr.as_ref().unwrap();

        if connector.connection != DRM_MODE_CONNECTED {
            let err = ErrorKind::SCREEN_CONNECTOR_FAILED;
            debug!("Failed to get connector, connector is not connected. - DeviceID: {}, ConnectorID: {}", device, connector_id);
            error!("Failed to get connector. - ErrorKind: {:?}", err);
            return Err(err);
        } else if connector.count_modes == 0 {
            let err = ErrorKind::SCREEN_CONNECTOR_MODE_FAILED;
            debug!("Failed to get connector, connector has no modes. - DeviceID: {}, ConnectorID: {}", device, connector_id);
            error!("Failed to get connector. - ErrorKind: {:?}", err);
        }

        Ok(connector_ptr)
    }

    unsafe fn get_modes(connector: *mut _drmModeConnector) -> Result<Vec<_drmModeModeInfo>, ErrorKind> {
        let mut modes: Vec<_drmModeModeInfo> = Vec::new();
        for i in 0..(*connector).count_modes {
            let mode = (*connector).modes.offset(i as isize).as_ref().unwrap().clone();
            modes.push(mode);
        }
        Ok(modes)
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn connector_type(&self) -> ConnectorType {
        self.connector_type
    }

    #[allow(non_snake_case)]
    pub fn mmWidth(&self) -> u32 {
        self.mmWidth
    }

    #[allow(non_snake_case)]
    pub fn mmHeight(&self) -> u32 {
        self.mmHeight
    }

    pub fn subpixel(&self) -> u32 {
        self.subpixel
    }

    pub fn modes(&self) -> &[_drmModeModeInfo] {
        self.modes.as_ref()
    }

    pub fn mode(&self) -> _drmModeModeInfo {
        self.mode
    }

    pub fn width(&self) -> u32 {
        self.mode.hdisplay as u32
    }

    pub fn height(&self) -> u32 {
        self.mode.vdisplay as u32
    }

    pub fn write(&mut self, x: u32, y: u32, width: u32, height: u32, pixels: &[u32]) -> Result<(), ErrorKind> {
        self.buffers[self.buffer_index].write(x, y, width, height, pixels)
    }

    pub(crate) fn swap_buffers(&mut self) -> Result<(), ErrorKind> {
        let buffer = &self.buffers[self.buffer_index];
        self.crtc.set_buffer(self.device, self.id, buffer)
    }
}
