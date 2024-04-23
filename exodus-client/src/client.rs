use exodus_common::{net::{connection::Connection, network_message::NetworkMessage}, consts::EXODUS_DIRECTORY};
use exodus_errors::ErrorKind;
use exodus_protocols::protocol_code::ProtocolCode::*;


#[derive(Debug)]
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

    pub fn connect(dpy: Option<String>, metadata: Metadata) -> Result<Self, ErrorKind> {
        if let Some(dpy) = dpy {
            let conn = Connection::connect(&format!("{}/{}", EXODUS_DIRECTORY, dpy))?;
            let mut entity = Self::new(conn);
            entity.set_metadata(metadata);
            return Ok(entity); 
        }

        let mut entity = Self::new(Connection::connect(&format!("{}/exodus-0", EXODUS_DIRECTORY))?);
        entity.set_metadata(metadata);

        Ok(entity)
    }

    pub fn disconnect(&mut self) {
        self.conn.disconnect();
    }

    fn request(&mut self, msg: NetworkMessage) -> Result<Option<NetworkMessage>, ErrorKind> {
        self.conn.send(msg);
        self.conn.buffer()
    }

    fn set_metadata(&mut self, metadata: Metadata) {
        let mut msg = NetworkMessage::new(ProtocolEntityRegister);
        msg.write_string_utf8(&metadata.class);
        msg.write_string_utf8(&metadata.title);
        msg.write_u32(metadata.version);
        msg.write_string_utf8(&metadata.author);
        msg.write_string_utf8(&metadata.description);

        self.conn.send(msg);
    }

}