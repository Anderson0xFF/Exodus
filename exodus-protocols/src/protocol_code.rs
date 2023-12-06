
pub const PROTOCOL_VERSION_1_0_0: u32 = 100;

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum ProtocolCode {
    ProtocolError = -1,
    /// None protocol.
    ProtocolNone = 0x0,

    /// Init vision entity.
    /// 
    /// Post: `ProtocolVisionEntityInit`
    /// 
    /// ### Arguments
    /// 
    /// * `class` - String utf8 of 128 bytes, the class of entity. 
    /// 
    ///       Example: "Mozilla Firefox"
    /// 
    /// * `title` - String utf8 of 128 bytes, the title of entity.
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
    ProtocolEntityInit,

    /// Get native vision data.
    /// 
    /// Post: `ProtocolNativeVisionData`
    /// 
    /// ### Arguments
    /// 
    /// No arguments.
    /// 
    /// ### Returns
    /// 
    /// * `id` - Integer of 32 bits, the id of native vision.
    /// 
    ///       Example: 33.
    /// 
    /// * `gpu` - Integer of 32 bits, current gpu id of native vision.
    /// 
    ///       Example: 7.
    /// 
    /// * `gpu_count` - Integer of 32 bits, the gpu count found.
    /// 
    ///       Example: 3.
    /// 
    /// * `gpus` - Array of integer of 32 bits, the gpus ids found.
    /// 
    ///       Example: [1, 7, 15].
    ///
    ProtocolDisplayData,

    /// Get GPU data.
    /// 
    /// Post: `ProtocolGPUData`
    /// 
    /// ### Arguments
    /// 
    /// * `id` - Integer of 32 bits, the id of gpu.
    /// 
    /// ### Returns
    /// 
    /// * `id` - Integer of 32 bits, the gpu id.
    /// 
    /// * `vendor` - Integer of 32 bits, the gpu vendor.
    /// 
    /// * `device` - Integer of 32 bits, the gpu device.
    /// 
    /// * `screen_count` - Integer of 32 bits, the screen count.
    /// 
    /// * `screens` - Array of integer of 32 bits, the screens ids.
    ProtocolGPUData,

    /// Get screen data.
    /// 
    /// Post: `ProtocolScreenData`
    /// 
    /// ### Arguments
    /// 
    /// * `gpu` - Integer of 32 bits, the id of gpu.
    /// 
    /// * `screen` - Integer of 32 bits, the id of screen.
    /// 
    /// ### Returns
    /// 
    /// * `id` - Usigned integer of 32 bits, the screen id.
    /// 
    /// * `connector_type` - Usigned integer of 32 bits, the connector type.
    /// 
    /// * `mm_width` - Integer of 32 bits, the width in millimeters.
    /// 
    /// * `mm_height` - Integer of 32 bits, the height in millimeters.
    /// 
    /// * `subpixel` - Integer of 32 bits, the subpixel.
    /// 
    /// * `mode` - Integer of 32 bits, the mode id.
    /// 
    /// * `modes_count` - Integer of 32 bits, the modes count.
    /// 
    /// * `modes` - Array of integer of 32 bits, the modes ids.
    ProtocolScreenData,

    /// Get screen mode data.
    /// 
    /// Post: `ProtocolScreenModeData`
    /// 
    /// ### Arguments
    /// 
    /// * `screen` - Integer of 32 bits, the id of screen.
    /// 
    /// ### Returns
    ///    * `clock` - Integer of 32 bits, the clock.
    ///    * `hdisplay` - Integer of 16 bits, the horizontal display.
    ///    * `hsync_start` - Integer of 16 bits, the horizontal sync start.
    ///    * `hsync_end` - Integer of 16 bits, the horizontal sync end.
    ///    * `htotal` - Integer of 16 bits, the horizontal total.
    ///    * `hskew` - Integer of 16 bits, the horizontal skew.
    ///    * `vdisplay` - Integer of 16 bits, the vertical display.
    ///    * `vsync_start` - Integer of 16 bits, the vertical sync start.
    ///    * `vsync_end` - Integer of 16 bits, the vertical sync end.
    ///    * `vtotal` - Integer of 16 bits, the vertical total.
    ///    * `vscan` - Integer of 16 bits, the vertical scan.
    ///    * `vrefresh` - Integer of 32 bits, the vertical refresh.
    ///    * `flags` - Integer of 32 bits, the flags.
    ///    * `type` - Integer of 32 bits, the type.
    ///    * `name` - Array of 32 bytes, the name.
    ProtocolScreenModeData,

    /// Drawing pixels in screen.
    /// 
    /// Post: `ProtocolGPURendering`
    /// 
    /// ### Arguments
    /// 
    /// * `gpu` - Integer of 32 bits, the gpu id.
    /// 
    /// * `screen` - Integer of 32 bits, the id of screen.
    /// 
    /// * `plane` - Integer of 32 bits, the plane id.
    ///     * Background = 1
    ///     * Cursor = 2
    ///     * Overlay = 3
    ///     * Normal = 4
    /// 
    /// * `x` - Integer of 32 bits, the x position.
    /// 
    /// * `y` - Integer of 32 bits, the y position.
    /// 
    /// * `width` - Integer of 32 bits, the width.
    /// 
    /// * `height` - Integer of 32 bits, the height.
    /// 
    /// * `subpixel` - Integer of 32 bits, the subpixel.
    ///     * Unknown = 1
    ///     * HorizontalRGB = 2
    ///     * HorizontalBGR = 3
    ///     * VerticalRGB = 4
    ///     * VerticalBGR = 5
    ///     * None = 6
    /// 
    /// * `pixels` - Array of integer of 32 bits, the pixels.
    /// 
    /// ### Returns
    /// 
    /// No returns.
    /// 
    /// ### Example
    ProtocolGPURendering,
}

impl From<i32> for ProtocolCode {
    fn from(code: i32) -> Self {
        match code {
            -1 => ProtocolCode::ProtocolError,
            0x0 => ProtocolCode::ProtocolNone,
            0x1 => ProtocolCode::ProtocolDisplayData,
            0x2 => ProtocolCode::ProtocolGPUData,
            0x3 => ProtocolCode::ProtocolScreenData,
            0x4 => ProtocolCode::ProtocolScreenModeData,
            0x5 => ProtocolCode::ProtocolGPURendering,
            _ => ProtocolCode::ProtocolNone,
        }
    }
}
