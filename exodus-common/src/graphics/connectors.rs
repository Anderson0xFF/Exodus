#![allow(dead_code)]

use drm::{drmModeConnection::DRM_MODE_CONNECTED, *};
use gbm::{gbm_bo, gbm_bo_create, gbm_bo_flags::*, gbm_bo_format::GBM_BO_FORMAT_XRGB8888, gbm_device};
use exodus_errors::ErrorKind;
use std::rc::Rc;

use crate::{debug, error, graphics::encoders::Encoder};

use super::{encoders::EncoderRef, crtcs::Crtc, device::Device};

pub type Encoders = Vec<EncoderRef>;
pub type Modes = Vec<drmModeModeInfo>;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum ConnectorType {
    Unknown,
    HDMIA,
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
impl ConnectorType {
    pub fn from_u32(connector_type: u32) -> Self {
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
    encoder: EncoderRef,
    encoders: Encoders,
    crtc: Crtc,
    buffer: *mut gbm_bo,
}

impl Screen {
    pub fn new(
        connector_id: u32,
        device: Device,
        native_device: *mut gbm_device,
    ) -> Result<Self, ErrorKind> 
    {
        
        debug!("Getting screen...");
        unsafe {
            let connector_ptr = drmModeGetConnector(device, connector_id);

            if connector_ptr.is_null() {
                let err = ErrorKind::EXODUS_CONNECTOR_FAILED;
                error!("Failed to get connector. - ErrorKind: {:?}", err);
                return Err(err);
            }

            let connector = connector_ptr.as_ref().unwrap();

            if connector.connection != DRM_MODE_CONNECTED {
                drmModeFreeConnector(connector_ptr);
                return Err(ErrorKind::EXODUS_CONNECTOR_NOT_CONNECTED);
            } else if connector.count_modes == 0 {
                drmModeFreeConnector(connector_ptr);
                return Err(ErrorKind::EXODUS_CONNECTOR_NO_MODES);
            }

            debug!("Getting modes...");
            let mut modes = Vec::new();
            for i in 0..connector.count_modes {
                let mode = connector.modes.offset(i as isize).as_ref().unwrap().clone();
                modes.push(mode);
            }

            if modes.is_empty() {
                drmModeFreeConnector(connector_ptr);
                return Err(ErrorKind::EXODUS_CONNECTOR_NO_MODES);
            }

            let mode = modes[0];
            
            debug!("Getting encoders...");
            let mut encoders = Vec::new();
            for i in 0..connector.count_encoders 
            {
                let id = connector
                    .encoders
                    .offset(i as isize)
                    .as_ref()
                    .unwrap()
                    .clone();

                let encoder = Rc::new(Encoder::new(id, device)?);
                encoders.push(encoder);
            }

            let encoder = encoders
                .iter()
                .find(|&encoder| encoder.id() == connector.encoder_id)
                .cloned()
                .unwrap();

            debug!("Getting crtc...");
            let crtc = Crtc::new(encoder.crtc_id(), device)?;

            debug!("Creating monitor buffer...");
            let monitor_buffer = gbm_bo_create(
                native_device,
                mode.hdisplay as u32,
                mode.vdisplay as u32,
                GBM_BO_FORMAT_XRGB8888,
                GBM_BO_USE_SCANOUT | GBM_BO_USE_RENDERING,
            );

            let screen = Screen {
                id: connector_id,
                connector_type: ConnectorType::from_u32(connector.connector_type),
                mmWidth: connector.mmWidth,
                mmHeight: connector.mmHeight,
                subpixel: connector.subpixel,
                mode,
                modes,
                encoder,
                encoders,
                crtc,
                buffer: monitor_buffer,
            };

            debug!("Screen detected. Screen: {:?}", screen);
            drmModeFreeConnector(connector_ptr);
            Ok(screen)
        }
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

    pub fn buffer(&self) -> *mut gbm_bo {
        self.buffer
    }
}

impl Drop for Screen {
    fn drop(&mut self) {
        unsafe {
            gbm::gbm_bo_destroy(self.buffer);
        }
    }
}