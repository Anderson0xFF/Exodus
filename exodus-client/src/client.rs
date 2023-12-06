use exodus_common::{net::connection::Connection, consts::EXODUS_DIRECTORY};
use exodus_errors::ErrorKind;
use exodus_protocols::network_message::NetworkMessage;
use exodus_protocols::protocol_code::ProtocolCode::*;

use crate::utils::{Display, GPU, Screen};

pub struct Metadata {
    pub class: String,
    pub title: String,
    pub version: u32,
    pub author: String,
    pub description: String,
}

#[derive(Debug)]
pub struct Entity {
    conn: Connection
}

impl Entity {
    #[inline]
    fn new(conn: Connection) -> Self {
        Self {
            conn,
        }
    }

    pub fn connect(dpy: Option<u32>, metadata: Metadata) -> Result<Self, ErrorKind> {
        if let Some(dpy) = dpy {
            let conn = Connection::connect(&format!("{}/exodus-{}", EXODUS_DIRECTORY, dpy))?;
            let mut entity = Self::new(conn);
            entity.set_metadata(metadata);
            return Ok(entity); 
        }

        let mut entity = Self::new(Connection::connect(&format!("{}/exodus-0", EXODUS_DIRECTORY))?);
        entity.set_metadata(metadata);

        Ok(entity)
    }

    fn request(&mut self, msg: NetworkMessage) -> Result<Option<NetworkMessage>, ErrorKind> {
        self.conn.send(msg);
        self.conn.buffer()
    }

    fn set_metadata(&mut self, metadata: Metadata) {
        let mut msg = NetworkMessage::new(ProtocolEntityInit);
        msg.write_string_utf8(&metadata.class);
        msg.write_string_utf8(&metadata.title);
        msg.write_u32(metadata.version);
        msg.write_string_utf8(&metadata.author);
        msg.write_string_utf8(&metadata.description);

        self.conn.send(msg);
    }

    pub fn get_display(&mut self) -> Result<Display, ErrorKind> {
        let msg = NetworkMessage::new(ProtocolDisplayData);
        let response = self.request(msg)?;
        if let None = response {
            return Err(ErrorKind::DISPLAY_NOT_FOUND);
        }

        let mut response = response.unwrap();
        let id = response.read_i32()?;
        let gpu = response.read_i32()?;
        let gpu_count = response.read_u32()?;
        let mut gpus = Vec::new();
        for _ in 0..gpu_count {
            gpus.push(response.read_i32()?);
        }

        Ok(Display::new(id, gpu, gpu_count, gpus))
    }

    pub fn get_gpu(&mut self, id: i32) -> Result<GPU, ErrorKind> {
        let mut msg = NetworkMessage::new(ProtocolGPUData);
        msg.write_i32(id);

        let response = self.request(msg)?;
        if let None = response {
            return Err(ErrorKind::GPU_NOT_FOUND);
        }

        let mut response = response.unwrap();
        let id = response.read_i32()?;
        let vendor = response.read_i32()?;
        let device = response.read_i32()?;
        let screen_count = response.read_u32()?;
        let mut screens = Vec::new();
        for _ in 0..screen_count {
            screens.push(response.read_i32()?);
        }

        Ok(GPU::new(id, vendor, device, screen_count, screens))
    }

    pub fn get_screen(&mut self, id: i32) -> Result<Screen, ErrorKind> {
        let mut msg = NetworkMessage::new(ProtocolScreenData);
        msg.write_i32(id);

        let response = self.request(msg)?;
        if let None = response {
            return Err(ErrorKind::SCREEN_NOT_FOUND);
        }

        let mut msg = response.unwrap();

        let id = msg.read_i32()?;
        let connector_type = msg.read_i32()?;
        let mm_width = msg.read_u32()?;
        let mm_height = msg.read_u32()?;
        let subpixel = msg.read_u32()?;
        let mode = msg.read_u32()?;
        let modes_count = msg.read_u32()?;
        let mut modes = Vec::new();

        for _ in 0..modes_count {
            modes.push(msg.read_u32()?);
        }

        Ok(Screen::new(id, connector_type.into(), mm_width, mm_height, subpixel.into(), mode, modes_count, modes))
    }
}