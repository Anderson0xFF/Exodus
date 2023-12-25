use drm::*;
use exodus_common::{enums::*, *, graphics::device::GPUID};
use exodus_errors::ErrorKind;
use super::encoders::Encoder;


pub type Modes = Vec<*mut _drmModeModeInfo>;

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct Connector {
    id: u32,
    connector_type: ConnectorType,
    mmWidth: u32,
    mmHeight: u32,
    subpixel: u32,
    modes: Modes,
    connector: *mut _drmModeConnector,
    encoder: Encoder
}

impl Connector {
    pub(crate) fn new(gpu: GPUID, connector_id: u32) -> Result<Option<Self>, ErrorKind> 
    {
        debug!("Checking connector.  - GPUID: {} - ConnectorID: {}", gpu, connector_id);
        let connector = unsafe { Self::get_connector(gpu, connector_id)? };
        if connector.is_none() {
            return Ok(None);
        }

        let connector = connector.unwrap();
        let modes = unsafe { Self::get_modes(connector) };
        let encoder = unsafe { (*connector).encoder_id };
        let encoder = Encoder::new(gpu, encoder)?;

        unsafe {
            Ok(Some(Self {
                id: (*connector).connector_id,
                connector_type: (*connector).connector_type.into(),
                mmWidth: (*connector).mmWidth,
                mmHeight: (*connector).mmHeight,
                subpixel: (*connector).subpixel,
                modes,
                connector,
                encoder,
            }))
        }
    }

    unsafe fn get_connector(gpu: GPUID, connector_id: u32) -> Result<Option<*mut _drmModeConnector>, ErrorKind> 
    {
        debug!("Getting connector. - GPUID: {}, ConnectorID: {}", gpu, connector_id);
        let connector_ptr = drmModeGetConnector(gpu, connector_id);

        if connector_ptr.is_null() {
            let err = ErrorKind::CONNECTOR_FAILED;
            error!("Failed to get connector data. - ErrorKind: {:?}", err);
            return Err(err);
        }

        let connector = connector_ptr.as_ref().unwrap();

        if connector.connection != drmModeConnection::DRM_MODE_CONNECTED {
            debug!("Connector {} - {:?} not connected.", connector.connector_id, ConnectorType::from(connector.connector_type));
            return Ok(None);
        }

        else if connector.count_modes == 0 {
            let err = ErrorKind::CONNECTOR_MODE_FAILED;
            error!("Failed to get connector. - ErrorKind: {:?}", err);
            return Err(err);
        }

        Ok(Some(connector_ptr))
    }

    unsafe fn get_modes(connector: *mut _drmModeConnector) -> Vec<*mut _drmModeModeInfo> 
    {
        debug!("Getting modes for connector: {:?}", connector.as_ref().unwrap().connector_id);

        let mut modes: Vec<*mut _drmModeModeInfo> = Vec::new();
        for i in 0..(*connector).count_modes {
            let mode: *mut _drmModeModeInfo = (*connector).modes.offset(i as isize);
            modes.push(mode);
        }

        modes
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

    pub fn get_mode(&self, id: u32) -> Option<drmModeModeInfoPtr> {
        if id as usize >= self.modes.len() {
            return None;
        }

        Some(self.modes[id as usize])
    }

    pub fn modes(&self) -> &[*mut drm::_drmModeModeInfo] {
        self.modes.as_slice()
    }

    pub fn encoder(&self) -> Encoder {
        self.encoder
    }
}

impl Drop for Connector {
    fn drop(&mut self) {
        unsafe {
            drmModeFreeConnector(self.connector);
        }
    }
}