use std::fmt::Display;

use drm::{*, drmModeConnection::DRM_MODE_CONNECTED};
use crate::{errors::ErrorKind, error, graphics::Size, types::ModeInfo, debug};

use super::enconders::Encoder;

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
            _=> ConnectorType::Unknown,
        }
    }
}


#[derive(Debug)]
pub struct Connector {
    id : u32,
    connector_type: ConnectorType,
    size: Size<u32>,
    subpixel: u32,
    current_mode: Option<ModeInfo>,
    modes: Vec<ModeInfo>,
    current_encoder: Option<Encoder>,
    encoders: Vec<Encoder>,
}

impl Connector {
    pub fn new(device: i32, id: u32) -> Result<Self, ErrorKind> {
        unsafe {
            let connector_ptr = drmModeGetConnector(device, id);

            if connector_ptr.is_null() {
                let err = ErrorKind::RAMEN_CONNECTOR_FAILED;
                error!("Failed to get connector. - ErrorKind: {:?}", err);
                return Err(err);
            }

            let connector = connector_ptr.as_ref().unwrap();
            if connector.connection != DRM_MODE_CONNECTED {
                drmModeFreeConnector(connector_ptr);
                return Err(ErrorKind::RAMEN_CONNECTOR_NOT_CONNECTED);
            }

            let size = Size {
                width: connector.mmWidth,
                height: connector.mmHeight,
            };

            let mut modes = Vec::new();
            for i in 0..connector.count_modes {
                let mode = connector.modes.offset(i as isize).as_ref().unwrap().clone();
                modes.push(mode);
            }

            let mut current_mode = None;
            if !modes.is_empty() {
                current_mode = Some(modes[0]);
            }

            let mut encoders = Vec::new();
            for i in 0..connector.count_encoders {
                let id = connector.encoders.offset(i as isize).as_ref().unwrap().clone();
                let encoder = Encoder::new(device, id)?;
                encoders.push(encoder);
            }

            let current_encoder = encoders.iter().find(|&encoder| encoder.id() == connector.encoder_id).cloned();

            drmModeFreeConnector(connector_ptr);

            let connector = Connector {
                id,
                connector_type: ConnectorType::from_u32(connector.connector_type),
                size,
                subpixel: connector.subpixel,
                current_mode,
                modes,
                current_encoder,
                encoders,
            };
            debug!("Found connector: {}", connector);

            Ok(connector)
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn connector_type(&self) -> ConnectorType {
        self.connector_type
    }

    pub fn size(&self) -> &Size<u32> {
        &self.size
    }

    pub fn subpixel(&self) -> u32 {
        self.subpixel
    }

    pub fn modes(&self) -> &[_drmModeModeInfo] {
        self.modes.as_ref()
    }

    pub fn current_mode(&self) -> Option<_drmModeModeInfo> {
        self.current_mode
    }
}

impl Display for Connector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Connector {{ id: {}, connector_type: {:?}, resolution: {:?}, subpixel: {} }}", self.id, self.connector_type, self.size, self.subpixel)
    }
}