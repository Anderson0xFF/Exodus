#![allow(non_upper_case_globals)]
use std::rc::Rc;

use drm::{drmModeConnection::DRM_MODE_CONNECTED, *};

use crate::{card::Card, debug, error, errors::ErrorKind};

pub struct Display {
    card: Rc<Card>,
    connectors: Vec<Connector>,
    resources: *mut drmModeRes,
}

impl Display {
    pub fn new(card: &Rc<Card>) -> Result<Self, ErrorKind> {
        let mut connectors = Vec::new();

        debug!("Search connectors...");
        let resources = unsafe { drmModeGetResources(card.fd()) };

        if resources.is_null() {
            let err = ErrorKind::RAMEN_DISPLAY_RESOURCES_FAILED;
            error!("Failed to search connectors. - ErrorKind: {:?}", err);
            return Err(err);
        }

        unsafe {
            let resources = *resources;

            for i in 0..resources.count_connectors {
                let connector_ptr = drmModeGetConnector(card.fd(), *resources.connectors.offset(i as isize));

                if connector_ptr.is_null() {
                    continue;
                }

                let connector = connector_ptr.as_ref().unwrap();
                if connector.connection == DRM_MODE_CONNECTED {
                    let width = connector.modes.offset(0).as_ref().unwrap().hdisplay;
                    let height = connector.modes.offset(0).as_ref().unwrap().vdisplay;

                    let connector_type = Self::connector_type_to_str(connector.connector_type);
                    debug!("Connector {{ Id: {} Type: {} Resolution: {}x{} }}", (*connector).connector_id, connector_type, width, height);
                    
                    connectors.push(Connector::new(card.clone(), connector_ptr)?);
                    continue;
                }

                drmModeFreeConnector(connector_ptr);
            }

            if connectors.is_empty() {
                let err = ErrorKind::RAMEN_DISPLAY_CONNECTORS_NOT_FOUND;
                error!("No connected connectors found. - ErrorKind: {:?}", err);
                return Err(err);
            }
        }

        Ok(Display {
            card: Rc::clone(card),
            connectors,
            resources,
        })
    }

    fn connector_type_to_str(connector_type: u32) -> &'static str {
        match connector_type {
            DRM_MODE_CONNECTOR_HDMIA => "HDMI-A",
            DRM_MODE_CONNECTOR_HDMIB => "HDMI-B",
            DRM_MODE_CONNECTOR_TV => "TV",
            DRM_MODE_CONNECTOR_DVII => "DVI-I",
            DRM_MODE_CONNECTOR_DVID => "DVI-D",
            DRM_MODE_CONNECTOR_DVIA => "DVI-A",
            DRM_MODE_CONNECTOR_VGA => "VGA",
            DRM_MODE_CONNECTOR_DisplayPort => "DisplayPort",
            DRM_MODE_CONNECTOR_eDP => "eDP",
            DRM_MODE_CONNECTOR_VIRTUAL => "Virtual",
            DRM_MODE_CONNECTOR_DSI => "DSI",
            DRM_MODE_CONNECTOR_DPI => "DPI",
            DRM_MODE_CONNECTOR_WRITEBACK => "Writeback",
            DRM_MODE_CONNECTOR_SPI => "SPI",
            DRM_MODE_CONNECTOR_LVDS => "LVDS",
            DRM_MODE_CONNECTOR_Composite => "Composite",
            DRM_MODE_CONNECTOR_SVIDEO => "S-Video",
            DRM_MODE_CONNECTOR_Component => "Component",
            DRM_MODE_CONNECTOR_9PinDIN => "9-Pin DIN",
            DRM_MODE_CONNECTOR_USB => "USB",
            _=> "Unknown",
        }
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        unsafe {
            drmModeFreeResources(&mut *self.resources);
        }
    }
}

pub type ConnectorPtr = *mut drmModeConnector;
pub type ResourcesPtr = *mut drmModeRes;

pub struct Connector {
    card: Rc<Card>,
    connector: *mut drmModeConnector,
    encoder: *mut drmModeEncoder,
    crtc: *mut drmModeCrtc,
}

impl Connector {
    pub fn new(card: Rc<Card>, connector: ConnectorPtr) -> Result<Self, ErrorKind> {
        unsafe {
            if (*connector).encoder_id == 0 {
                let err = ErrorKind::RAMEN_CONNECTOR_ENCODER_NOT_FOUND;
                error!("No encoder found. - ErrorKind: {:?}", err);
                return Err(err);
            }

            let encoder = drmModeGetEncoder(card.fd(), (*connector).encoder_id);

            if encoder.is_null() {
                let err = ErrorKind::RAMEN_CONNECTOR_ENCODER_FAILED;
                error!("Failed to get encoder. - ErrorKind: {:?}", err);
                return Err(err);
            }
            else if (*encoder).crtc_id == 0 {
                let err = ErrorKind::RAMEN_CONNECTOR_ENCODER_CRTC_NOT_FOUND;
                error!("No CRTC found. - ErrorKind: {:?}", err);
                return Err(err);
            }

            let crtc = drmModeGetCrtc(card.fd(), (*encoder).crtc_id);

            if crtc.is_null() {
                let err = ErrorKind::RAMEN_CONNECTOR_ENCODER_CRTC_FAILED;
                error!("Failed to get CRTC. - ErrorKind: {:?}", err);
                return Err(err);
            }

            Ok(Connector {
                card,
                connector,
                encoder,
                crtc,
            })
        }
    }
}

impl Drop for Connector {
    fn drop(&mut self) {
        if !self.connector.is_null() {
            unsafe {
                drmModeFreeConnector(self.connector);
            }
        }

        if !self.encoder.is_null() {
            unsafe {
                drmModeFreeEncoder(self.encoder);
            }
        }

        if !self.crtc.is_null() {
            unsafe {
                drmModeFreeCrtc(self.crtc);
            }
        }
    }
}
