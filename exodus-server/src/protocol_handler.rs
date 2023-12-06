use exodus_common::net::network_message::NetworkMessage;
use exodus_errors::ErrorKind;
use exodus_protocols::protocol_code::ProtocolCode;
use crate::{client::Entity, display::Display};

pub type Handler = fn(&mut Display, &mut Entity, NetworkMessage) -> Result<(), ErrorKind>;

#[derive(Debug)]
pub struct ProtocolHandler {
    dpy: Display,
    recv_entity_metadata:   Handler,
    send_display_data:      Handler,
    send_gpu_data:          Handler,
}

impl ProtocolHandler {
    pub fn new(dpy: Display) -> Self {
        Self {
            dpy,
            recv_entity_metadata:   Self::recv_entity_metadata,
            send_display_data:      Self::send_display_data,
            send_gpu_data:          Self::send_gpu_data,
        }
    }

    pub fn set_protocol_handler(&mut self, code: ProtocolCode, callback: Handler) -> Result<(), ErrorKind> {
        match code {
            ProtocolCode::ProtocolEntityInit        => self.recv_entity_metadata = callback,
            ProtocolCode::ProtocolDisplayData       => self.send_display_data = callback,
            ProtocolCode::ProtocolGPUData           => self.send_gpu_data = callback,
            ProtocolCode::ProtocolScreenData        => todo!(),
            ProtocolCode::ProtocolScreenModeData    => todo!(),
            ProtocolCode::ProtocolGPURendering      => todo!(),
            _ => todo!(),
        };

        Ok(())
    }

    pub fn process_request(&mut self, entity: &mut Entity) -> Result<(), ErrorKind> {
        let msg = entity.get_request()?;

        if msg.is_none() {
            return Ok(());
        }

        let msg = msg.unwrap();
        let code = msg.code()?;
        let code = ProtocolCode::from(code);

        match code {
            ProtocolCode::ProtocolEntityInit
            => {
                let recv_entity_metadata: Handler = self.recv_entity_metadata;
                recv_entity_metadata(&mut self.dpy, entity, msg)?;
            }
            ProtocolCode::ProtocolDisplayData
            => {
                let send_display_data: Handler = self.send_display_data;
                send_display_data(&mut self.dpy, entity, msg)?;
            }
            ProtocolCode::ProtocolGPUData
            => {
                let send_gpu_data: Handler = self.send_gpu_data;
                send_gpu_data(&mut self.dpy, entity, msg)?;
            }
            ProtocolCode::ProtocolScreenData        => todo!(),
            ProtocolCode::ProtocolScreenModeData    => todo!(),
            ProtocolCode::ProtocolGPURendering      => todo!(),
            _ => todo!(),
        };

        Ok(())
    }

    pub fn recv_entity_metadata(_: &mut Display, entity: &mut Entity, mut msg: NetworkMessage) -> Result<(), ErrorKind> {
        let class       = msg.read_string_utf8()?;
        let version        = msg.read_u32()?;
        let author      = msg.read_string_utf8()?;
        let description = msg.read_string_utf8()?;

        entity.set_class(class);
        entity.set_version(version);
        entity.set_author(author);
        entity.set_description(description);
        Ok(())
    }

    pub fn send_display_data(dpy: &mut Display, entity: &mut Entity, _: NetworkMessage) -> Result<(), ErrorKind> {
        let mut msg = NetworkMessage::new(ProtocolCode::ProtocolDisplayData);
        msg.write_i32(dpy.id());
        
        let gpus = dpy.gpus();
        if gpus.is_empty() {
            msg.write_i32(-1);
            msg.write_u32(0);
            return Ok(());
        }

        let gpu = &gpus[0];
        msg.write_i32(gpu.id());
        msg.write_u32(gpus.len().try_into().unwrap());

        for gpu in gpus {
            msg.write_i32(gpu.id());
        }

        entity.send(msg);
        Ok(())
    }

    pub fn send_gpu_data(dpy: &mut Display, entity: &mut Entity, mut msg: NetworkMessage) -> Result<(), ErrorKind> {
        let id = msg.read_i32()?;
        
        if let Some(gpu) = dpy.gpu(id) {
            let mut msg = NetworkMessage::new(ProtocolCode::ProtocolGPUData);
            msg.write_i32(gpu.id());
            msg.write_i32(-1);
            msg.write_i32(-1);
            msg.write_u32(gpu.screens().len().try_into().unwrap());

            for screen in gpu.screens() {
                msg.write_u32(screen.id());
            }

            entity.send(msg);
        }

        Ok(())
    }

    pub fn send_screen_data(dpy: &mut Display, entity: &mut Entity, mut input: NetworkMessage) -> Result<(), ErrorKind> {
        let gpu = input.read_i32()?;
        let gpu = dpy.gpu(gpu).unwrap();
        let screen = input.read_u32()?;
        let screen = gpu.get_screen_from_id(screen).unwrap();

        let mut msg = NetworkMessage::new(ProtocolCode::ProtocolScreenData);
        msg.write_u32(screen.id());
        msg.write_u32(screen.connector_type() as u32);
        msg.write_u32(screen.mmWidth());
        msg.write_u32(screen.mmHeight());
        msg.write_u32(screen.subpixel());

        // Mode
        msg.write_u32(0);

        // Modes count
        let modes = screen.modes();
        msg.write_u32(modes.len().try_into().unwrap());

        // Modes
        for i in 0..modes.len() {
            msg.write_u32(i.try_into().unwrap());
        }

        entity.send(msg);
        Ok(())
    }
}
