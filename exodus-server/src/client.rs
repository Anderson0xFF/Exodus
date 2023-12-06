use exodus_common::net::connection::Connection;
use exodus_errors::ErrorKind;
use exodus_protocols::network_message::NetworkMessage;

#[derive(Debug, Default)]
pub struct Entity {
    conn: Option<Connection>,
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
            conn: Some(conn), ..Default::default()
        }
    }
    pub fn id(&self) -> u32 {
        self.conn.as_ref().unwrap().id()
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

    pub fn title(&self) -> &str {
        &self.title
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

    pub fn get_request(&mut self) -> Result<Option<NetworkMessage>, ErrorKind> {
        self.conn.as_mut().unwrap().buffer()
    }

    pub fn send(&mut self, msg: NetworkMessage) {
        self.conn.as_mut().unwrap().send(msg)
    }
}
