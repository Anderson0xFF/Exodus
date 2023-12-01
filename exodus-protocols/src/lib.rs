#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum Protocol {
    
    /// ProtocolTypeInit
    ///
    /// This protocol is used to initialize the client and server.
    /// 
    /// ### Header
    /// | Type | Name | Description |
    /// | --- | --- | --- |
    /// | i32 | display_id | The display id. |
    /// | i32 | device_id | The graphic device id. |
    /// | u32 | screen_count | The screen count. |
    /// | u32 | screen_id | The screen id. |
    /// | u32 | width | The screen width. |
    /// | u32 | height | The screen height. |
    /// 
    /// ### Example
    /// ```rust
    /// let mut msg = NetworkMessage::default();
    /// let display_id = display.get_id();
    /// let device = display.get_graphic_device();
    /// let screens = device.screens();
    /// 
    /// msg.write_u32(Protocol::ProtocolTypeInit as u32);
    /// msg.write_i32(display_id);
    /// msg.write_i32(device.id());
    /// msg.write_u32(screens.len() as u32);
    /// 
    /// for screen in screens {
    ///   msg.write_u32(screen.id());
    ///   msg.write_u32(screen.width());
    ///   msg.write_u32(screen.height());
    /// }
    /// ```
    /// 
    ProtocolTypeInit = 0x0001,
    ProtocolTypeTerminate = 0x0002,

    /// ProtocolTypeSurfaceCommit
    /// 
    /// This protocol is used to commit the surface.
    /// 
    /// ### Header
    /// | Type | Name | Description |
    /// | --- | --- | --- |
    /// | u32 | surface_id | The surface id. |
    /// | u32 | surface_type | The surface type. |
    /// | u32 | surface_format | The surface format. |
    /// | u32 | surface_stride | The surface stride. |
    /// 
    /// | u32 | surface_width | The surface width. |
    /// | u32 | surface_height | The surface height. |
    ProtocolTypeSurfaceCommit,
    ProtocolTypeError,
    
    ProtocolTypeNone = 0x0000,
}

impl Protocol {
    pub fn from(opcode: u32) -> Self {
        match opcode {
            0x0001 => Protocol::ProtocolTypeInit,
            0x0002 => Protocol::ProtocolTypeTerminate,
            0x0003 => Protocol::ProtocolTypeSurfaceCommit,
            0x0004 => Protocol::ProtocolTypeError,

            _ => Protocol::ProtocolTypeNone,
        }
    }
}