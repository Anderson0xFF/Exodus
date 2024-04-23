use exodus_common::{net::{connection::Connection, network_message::NetworkMessage}, debug};
use exodus_errors::ErrorKind;

#[derive(Debug)]
pub struct Entity {
    conn: Connection,
    class: String,
    title: String,
    version: u32,
    author: String,
    description: String,
}

impl Entity {
    #[inline]
    pub(crate) fn new(conn: Connection) -> Self {
        Self {
            conn,
            class: String::new(),
            title: String::new(),
            version: 0,
            author: String::new(),
            description: String::new()
        }
    }
    pub fn id(&self) -> u32 {
        self.conn.id()
    }

    pub(crate) fn set_class(&mut self, class: String) {
        self.class = class;
    }

    pub(crate) fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub(crate) fn set_version(&mut self, version: u32) {
        self.version = version;
    }

    pub(crate) fn set_author(&mut self, author: String) {
        self.author = author;
    }

    pub(crate) fn set_description(&mut self, description: String) {
        self.description = description;
    }

    pub fn class(&self) -> &str {
        &self.class
    }

    pub fn version(&self) -> u32 {
        self.version
    }

    pub fn author(&self) -> &str {
        &self.author
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub(crate) fn recv_message(&mut self) -> Result<Option<NetworkMessage>, ErrorKind> {
        self.conn.buffer()
    }

    pub fn send(&mut self, msg: NetworkMessage) {
        self.conn.send(msg)
    }
}

impl Drop for Entity {
    fn drop(&mut self) {
        debug!("Entity dropped. - ID: {}", self.id());
        self.conn.disconnect();
    }
}