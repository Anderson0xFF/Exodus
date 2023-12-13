use std::{os::unix::net::UnixStream, io::{Read, Write}};
use exodus_errors::ErrorKind;
use crate::if_;
use super::network_message::NetworkMessage;


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

    pub fn disconnect(&mut self) {
        self.socket.shutdown(std::net::Shutdown::Both).unwrap_or_default();
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

