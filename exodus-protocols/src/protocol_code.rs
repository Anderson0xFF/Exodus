
pub const PROTOCOL_VERSION_1_0_0: u32 = 100;

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum ProtocolCode {
    ProtocolError = -1,
    /// None protocol.
    ProtocolNone = 0x0,

    /// Register entity in display server.
    /// 
    /// Post: `ProtocolEntityRegister`
    /// 
    /// ### Arguments
    /// 
    /// * `class` - String utf8 of 128 bytes, the class of entity. 
    /// 
    ///       Example: "Mozilla Firefox"
    /// 
    /// * `title` - String utf16 of 128 bytes, the title of entity.
    /// 
    ///       Example: "Mozilla Firefox - Web Browser"
    /// 
    /// * `version` - Number of 32 bits, the version of protocol. 
    /// 
    ///       Example: 1.0.0 = 100.
    /// 
    /// * `author` - String utf8 of 128 bytes, the author of entity.
    /// 
    ///       Example: "Mozilla Foundation"
    /// 
    /// * `description` - String utf8 of 128 bytes, the description of entity.
    /// 
    ///       Example: "Mozilla Firefox is a free and open-source web browser developed by the Mozilla Foundation and its subsidiary, the Mozilla Corporation."
    ///
    /// ### Returns
    /// 
    /// No returns.
    ProtocolEntityRegister,

    /// List all GPUs found in the system.
    /// 
    /// Post: `ProtocolEnumerateGPUS`
    /// 
    /// ### Arguments
    /// 
    /// No arguments.
    /// 
    /// ### Returns
    /// 
    /// 
    /// * `gpu_count` - Number of 32 bits, the count of GPUs.
    /// 
    ///       Example: 2
    /// 
    /// * `gpus` - List of 32 bits, the id of GPUs.
    /// 
    ///       Example: [5, 9]
    /// 
    ProtocolEnumerateGPUS,

    /// Get information about GPU.
    /// 
    /// Post: `ProtocolGPUInfo`
    /// 
    /// ### Arguments
    /// 
    /// * `gpu` - Number of 32 bits, the id of GPU.
    /// 
    ///       Example: 5.
    /// 
    /// ### Returns
    /// 
    /// * `gpu` - Number of 32 bits, the id of GPU.
    /// 
    /// * `vendor` - Number of 32 bits, the vendor id of GPU.
    /// 
    ///       Example: 0x10DE
    /// 
    /// * `vendor_name` - String utf8 of 128 bytes, the vendor of GPU.
    /// 
    ///       Example: "NVIDIA Corporation"
    /// 
    /// * `model` - Number of 32 bits, the model id of GPU.
    /// 
    ///       Example: 0x1F02
    /// 
    ProtocolGPUInfo,

    /// List all screens found in the system.
    /// 
    /// Post: `ProtocolEnumerateScreens`
    /// 
    /// ### Arguments
    /// 
    /// * `gpu` - Number of 32 bits, the id of GPU.
    /// 
    ///       Example: 5.
    /// 
    /// ### Returns
    /// 
    /// * `screen_count` - Number of 32 bits, the count of screens.
    /// 
    ///       Example: 2
    /// 
    /// * `screens` - List of 32 bits, the id of screens.
    /// 
    ///       Example: [1, 2]
    /// 
    ProtocolEnumerateScreens,

    /// Get information about screen.
    /// 
    /// Post: `ProtocolScreenInfo`
    /// 
    /// ### Arguments
    /// 
    /// * `gpu` - Number of 32 bits, the id of GPU.
    /// 
    ///       Example: 5.
    /// 
    /// * `screen` - Number of 32 bits, the id of screen.
    /// 
    ///       Example: 2
    /// 
    /// ### Returns
    /// 
    /// * `screen` - Number of 32 bits, the id of screen.
    /// 
    ///       Example: 2
    /// 
    /// * `width` - Number of 32 bits, the width of screen in pixels.
    /// 
    ///       Example: 1920
    /// 
    /// * `height` - Number of 32 bits, the height of screen in pixels.
    /// 
    ///       Example: 1080
    /// 
    /// * `refresh` - Number of 32 bits, the refresh rate of screen in hertz.
    /// 
    ///       Example: 60
    /// 
    /// * `subpixel` - Number of 32 bits, the subpixel of screen.
    /// 
    ///       Example: 1
    /// 
    /// * `connector_type` - Number of 32 bits, the connector type of screen.
    /// 
    ///       Example: 2
    /// 
    /// * `mm_width` - Number of 32 bits, the width of screen in millimeters.
    /// 
    ///       Example: 309
    /// 
    /// * `mm_height` - Number of 32 bits, the height of screen in millimeters.
    /// 
    ///       Example: 174
    /// 
    /// * `buffer_count` - Number of 32 bits, the count of buffers.
    /// 
    ///       Example: 2
    /// 
    ProtocolScreenInfo,
}