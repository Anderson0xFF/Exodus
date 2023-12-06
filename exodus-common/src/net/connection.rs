use std::{os::unix::net::UnixStream, io::{Read, Write}};
use exodus_errors::ErrorKind;
use exodus_protocols::network_message::NetworkMessage;
use crate::if_;


#[derive(Debug)]
pub struct Connection {
    id: u32,
    socket: UnixStream,
}

impl Connection {
    
    pub fn new(socket: UnixStream) -> Self { 
        static mut ID: u32 = 0;

        Self { 
            id: unsafe { ID += 1; ID },
            socket,
        } 
    }

    pub fn connect(path: &str) -> Result<Self, ErrorKind> {
        if let Ok(socket) = UnixStream::connect(path) {
            return Ok(Self::new(socket));
        }

        Err(ErrorKind::CONNECTION_FAILED)
    }

    pub fn set_nonblocking(&mut self, nonblocking: bool) {
        self.socket.set_nonblocking(nonblocking).unwrap_or_default();
    }

    #[inline]
    pub fn id(&self) -> u32 { self.id }

    pub fn buffer(&mut self) -> Result<Option<NetworkMessage>, ErrorKind> {
        let mut msg = NetworkMessage::default();

        match self.socket.read(msg.get_buffer()) {
            Ok(size) => if_!(size > 0, return Ok(Some(msg))),
            Err(_) => return Err(ErrorKind::CONNECTION_CLOSED),
        }

        Ok(None)
    }

    pub fn send(&mut self, mut msg: NetworkMessage) {
        self.socket.write(msg.get_buffer()).unwrap_or_default();
    }
}

