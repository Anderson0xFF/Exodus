use exodus_common::net::{connection::Connection, network_message::NetworkMessage};
use exodus_errors::ErrorKind;
use exodus_protocols::Protocol;

use crate::vision::NativeVision;

#[derive(Debug)]
pub struct VisionActivity {
    conn: Connection,
}

impl VisionActivity {
    #[inline]
    pub(crate) fn new(mut conn: Connection, dpy: &NativeVision) -> Self {
        let mut msg = NetworkMessage::default();
        let dpy_id = dpy.id();
        let device = dpy.get_graphic_device();
        let screens = device.screens();

        msg.write_u32(Protocol::ProtocolTypeInit as u32);
        msg.write_i32(dpy_id);
        msg.write_i32(device.id());
        msg.write_u32(screens.len() as u32);

        for screen in screens {
            msg.write_u32(screen.id());
            msg.write_u32(screen.width());
            msg.write_u32(screen.height());
        }

        conn.send(msg);

        Self { conn }
    }

    pub fn get_id(&self) -> u32 {
        self.conn.get_id()
    }

    pub fn get_msg(&mut self) -> Result<Option<NetworkMessage>, ErrorKind> {
        self.conn.buffer()
    }

    pub fn send(&mut self, msg: NetworkMessage) {
        self.conn.send(msg)
    }
}
