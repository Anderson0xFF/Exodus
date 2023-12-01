use exodus_common::{net::{connection::Connection, network_message::NetworkMessage}, consts::{EXODUS_DISPLAY, EXODUS_DIRECTORY}};
use exodus_errors::ErrorKind;

#[derive(Debug)]
pub struct VisionActivity {
    conn: Connection
}

impl VisionActivity {
    #[inline]
    fn new(conn: Connection) -> Self {
        Self {
            conn,
        }
    }

    pub fn connect(dpy: Option<u32>, nonblocking: bool) -> Result<Self, ErrorKind> {
        if let Some(dpy) = dpy {
            let mut conn = Connection::connect(&format!("{}/exodus-{}", EXODUS_DIRECTORY, dpy))?;
            conn.set_nonblocking(nonblocking);
            return Ok(Self::new(conn));
        }

        let dpy = std::env::var(EXODUS_DISPLAY);
        if dpy.is_err() {
            return Err(ErrorKind::EXODUS_NATIVE_VISION_NOT_RUNNING);
        }

        let dpy = dpy.unwrap();
        if let Ok(mut conn) = Connection::connect(&format!("{}/exodus-{}", EXODUS_DIRECTORY, dpy)) {
            conn.set_nonblocking(nonblocking);
            return Ok(VisionActivity::new(conn));
        }

        return Err(ErrorKind::EXODUS_VISION_ACTIVITY_CONNECT_FAILED);
    }

    pub fn send(&mut self, msg: NetworkMessage) {
        self.conn.send(msg)
    }

    pub fn get_msg(&mut self) -> Result<Option<NetworkMessage>, ErrorKind> {
        self.conn.buffer()
    }

}