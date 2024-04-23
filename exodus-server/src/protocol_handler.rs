use exodus_common::net::network_message::NetworkMessage;
use exodus_errors::ErrorKind;
use exodus_protocols::protocol_code::ProtocolCode;
use crate::{client::Entity, display::Display};

pub type Handler = fn(&mut Display, &mut Entity, NetworkMessage) -> Result<(), ErrorKind>;

#[derive(Debug)]
pub struct ProtocolHandler {
    proto_register_entity:      Handler,
    proto_enumerate_gpus:       Handler,
    proto_gpuinfo:              Handler,
    proto_enumerate_screen:     Handler,
    proto_screeninfo:           Handler,
}

impl ProtocolHandler {
    pub fn new() -> Self {
        Self {
            proto_register_entity:      Self::protocol_register_entity,
            proto_enumerate_gpus:       Self::protocol_enumerate_gpus,
            proto_gpuinfo:              Self::protocol_gpuinfo,
            proto_enumerate_screen:     Self::protocol_enumerate_screen,
            proto_screeninfo:           Self::protocol_screeninfo,
        }
    }

    /// Sets the protocol handler for the given protocol code.
    pub fn set_protocol_handler(&mut self, code: ProtocolCode, callback: Handler) -> Result<(), ErrorKind> {
        match code {
            ProtocolCode::ProtocolEntityRegister        => self.proto_register_entity   = callback,
            ProtocolCode::ProtocolEnumerateGPUS         => self.proto_enumerate_gpus    = callback,
            ProtocolCode::ProtocolGPUInfo               => self.proto_gpuinfo           = callback,
            ProtocolCode::ProtocolEnumerateScreens      => self.proto_enumerate_screen  = callback,
            ProtocolCode::ProtocolScreenInfo            => self.proto_screeninfo        = callback,
            _ => todo!(),
        };

        Ok(())
    }

    /// Handles the given protocol code.
    /// 
    /// # Arguments
    /// 
    /// * `code` - The protocol code to handle.
    /// * `message` - The message to handle.
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(())` if the protocol code was handled successfully.
    /// Returns `Err(ErrorKind)` if the protocol code was not handled successfully.
    /// 
    /// # Examples
    
    pub fn handle(&mut self, display: &mut Display, entity: &mut Entity, code: ProtocolCode) -> Result<(), ErrorKind> {
        let message = entity.recv_message()?;

        if message.is_none() {
            return Ok(());
        }

        let message = message.unwrap();

        match code {
            ProtocolCode::ProtocolEntityRegister    => (self.proto_register_entity)(display, entity, message),
            ProtocolCode::ProtocolEnumerateGPUS     => (self.proto_enumerate_gpus)(display, entity, message),
            ProtocolCode::ProtocolGPUInfo           => (self.proto_gpuinfo)(display, entity, message),
            ProtocolCode::ProtocolEnumerateScreens  => (self.proto_enumerate_screen)(display, entity, message),
            _ => todo!(),
        }
    }

    pub fn protocol_register_entity(_: &mut Display, entity: &mut Entity, mut message: NetworkMessage) -> Result<(), ErrorKind> {
        let class = message.read_string_utf8()?;
        let title = message.read_string_utf16()?;
        let version = message.read_u32()?;
        let author = message.read_string_utf8()?;
        let description = message.read_string_utf8()?;

        entity.set_class(class);
        entity.set_title(title);
        entity.set_version(version);
        entity.set_author(author);
        entity.set_description(description);

        Ok(())
    }

    pub fn protocol_enumerate_gpus(display: &mut Display, entity: &mut Entity, _: NetworkMessage) -> Result<(), ErrorKind> {
        let mut message = NetworkMessage::new(ProtocolCode::ProtocolEnumerateGPUS);
        let gpus = display.gpus();

        message.write_u32(gpus.len() as u32);

        for gpu in gpus {
            message.write_i32(gpu.id());
        }

        entity.send(message);

        Ok(())
    }

    pub fn protocol_gpuinfo(display: &mut Display, entity: &mut Entity, mut message: NetworkMessage) -> Result<(), ErrorKind> {
        let gpu_id = message.read_i32()?;
        
        if let Some(gpu) = display.get_gpu(gpu_id) {
            let mut message = NetworkMessage::new(ProtocolCode::ProtocolGPUInfo);
            message.write_i32(gpu.id());
            message.write_u32(gpu.vendor() as u32);
            message.write_string_utf8(gpu.vendor().to_string().as_str());
            message.write_u32(gpu.model());
            entity.send(message);

            return Ok(());
        }

        let mut message = NetworkMessage::new(ProtocolCode::ProtocolError);
        message.write_string_utf8("Failed to get GPU info.");
        entity.send(message);

        Ok(())
    }

    pub fn protocol_enumerate_screen(display: &mut Display, entity: &mut Entity, _: NetworkMessage) -> Result<(), ErrorKind> {
        let mut message = NetworkMessage::new(ProtocolCode::ProtocolEnumerateScreens);
        let gpu = display.get_gpu(message.read_i32()?);

        if gpu.is_none() {
            let mut message = NetworkMessage::new(ProtocolCode::ProtocolError);
            message.write_string_utf8("GPU not found.");
            entity.send(message);
            return Ok(());
        }

        let gpu = gpu.unwrap();
        let screens = gpu.screens();
    
        message.write_u32(screens.len() as u32);

        for screen in screens {
            message.write_u32(screen.id());
        }

        entity.send(message);

        Ok(())
    }

    pub fn protocol_screeninfo(display: &mut Display, entity: &mut Entity, mut message: NetworkMessage) -> Result<(), ErrorKind> {
        let gpu = display.get_gpu(message.read_i32()?);

        if gpu.is_none() {
            let mut message = NetworkMessage::new(ProtocolCode::ProtocolError);
            message.write_string_utf8("GPU not found.");
            entity.send(message);
            return Ok(());
        }

        let gpu = gpu.unwrap();
        let screen = gpu.get_screen(message.read_u32()?);

        if screen.is_none() {
            let mut message = NetworkMessage::new(ProtocolCode::ProtocolError);
            message.write_string_utf8("Screen not found.");
            entity.send(message);
            return Ok(());
        }

        let screen = screen.unwrap();
        let mut message = NetworkMessage::new(ProtocolCode::ProtocolScreenInfo);
        message.write_u32(screen.id());
        message.write_u32(screen.width());
        message.write_u32(screen.height());
        message.write_u32(screen.refresh());
        message.write_u32(screen.subpixel() as u32);
        message.write_u32(screen.connector_type() as u32);
        message.write_u32(screen.mmWidth());
        message.write_u32(screen.mmHeight());
        message.write_u32(screen.buffer_count() as u32);
        entity.send(message);

        Ok(())
    }
}
