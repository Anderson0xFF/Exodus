use drm::*;
use gbm::gbm_bo_format::*;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScreenFlags {
    DoubleBuffered = 1,
    TripleBuffered = 2,
}


#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum ConnectorType {
    Unknown,
    HDMIA = 1,
    HDMIB,
    TV,
    DVII,
    DVID,
    DVIA,
    VGA,
    DISPLAY_PORT,
    eDP,
    VIRTUAL,
    DSI,
    DPI,
    WRITEBACK,
    SPI,
    LVDS,
    COMPOSITE,
    SVIDEO,
    COMPONENT,
    NINE_PIN_DIN,
    USB,
}

#[allow(non_upper_case_globals)]
impl From<u32> for ConnectorType {
    fn from(connector_type: u32) -> Self {
        match connector_type {
            DRM_MODE_CONNECTOR_HDMIA => ConnectorType::HDMIA,
            DRM_MODE_CONNECTOR_HDMIB => ConnectorType::HDMIB,
            DRM_MODE_CONNECTOR_TV => ConnectorType::TV,
            DRM_MODE_CONNECTOR_DVII => ConnectorType::DVII,
            DRM_MODE_CONNECTOR_DVID => ConnectorType::DVID,
            DRM_MODE_CONNECTOR_DVIA => ConnectorType::DVIA,
            DRM_MODE_CONNECTOR_VGA => ConnectorType::VGA,
            DRM_MODE_CONNECTOR_DisplayPort => ConnectorType::DISPLAY_PORT,
            DRM_MODE_CONNECTOR_eDP => ConnectorType::eDP,
            DRM_MODE_CONNECTOR_VIRTUAL => ConnectorType::VIRTUAL,
            DRM_MODE_CONNECTOR_DSI => ConnectorType::DSI,
            DRM_MODE_CONNECTOR_DPI => ConnectorType::DPI,
            DRM_MODE_CONNECTOR_WRITEBACK => ConnectorType::WRITEBACK,
            DRM_MODE_CONNECTOR_SPI => ConnectorType::SPI,
            DRM_MODE_CONNECTOR_LVDS => ConnectorType::LVDS,
            DRM_MODE_CONNECTOR_Composite => ConnectorType::COMPOSITE,
            DRM_MODE_CONNECTOR_SVIDEO => ConnectorType::SVIDEO,
            DRM_MODE_CONNECTOR_Component => ConnectorType::COMPONENT,
            DRM_MODE_CONNECTOR_9PinDIN => ConnectorType::NINE_PIN_DIN,
            DRM_MODE_CONNECTOR_USB => ConnectorType::USB,
            _ => ConnectorType::Unknown,
        }
    }
}

#[allow(non_upper_case_globals)]
impl From<i32> for ConnectorType {
    fn from(connector_type: i32) -> Self {
        match connector_type as u32 {
            DRM_MODE_CONNECTOR_HDMIA => ConnectorType::HDMIA,
            DRM_MODE_CONNECTOR_HDMIB => ConnectorType::HDMIB,
            DRM_MODE_CONNECTOR_TV => ConnectorType::TV,
            DRM_MODE_CONNECTOR_DVII => ConnectorType::DVII,
            DRM_MODE_CONNECTOR_DVID => ConnectorType::DVID,
            DRM_MODE_CONNECTOR_DVIA => ConnectorType::DVIA,
            DRM_MODE_CONNECTOR_VGA => ConnectorType::VGA,
            DRM_MODE_CONNECTOR_DisplayPort => ConnectorType::DISPLAY_PORT,
            DRM_MODE_CONNECTOR_eDP => ConnectorType::eDP,
            DRM_MODE_CONNECTOR_VIRTUAL => ConnectorType::VIRTUAL,
            DRM_MODE_CONNECTOR_DSI => ConnectorType::DSI,
            DRM_MODE_CONNECTOR_DPI => ConnectorType::DPI,
            DRM_MODE_CONNECTOR_WRITEBACK => ConnectorType::WRITEBACK,
            DRM_MODE_CONNECTOR_SPI => ConnectorType::SPI,
            DRM_MODE_CONNECTOR_LVDS => ConnectorType::LVDS,
            DRM_MODE_CONNECTOR_Composite => ConnectorType::COMPOSITE,
            DRM_MODE_CONNECTOR_SVIDEO => ConnectorType::SVIDEO,
            DRM_MODE_CONNECTOR_Component => ConnectorType::COMPONENT,
            DRM_MODE_CONNECTOR_9PinDIN => ConnectorType::NINE_PIN_DIN,
            DRM_MODE_CONNECTOR_USB => ConnectorType::USB,
            _ => ConnectorType::Unknown,
        }
    }
}


#[derive(Debug, Copy, Clone)]
pub enum PixelFormat {
    XRGB8888 = 0,
    ARGB8888,
}

impl PixelFormat {
    pub fn bpp(&self) -> u32 {
        match self {
            PixelFormat::XRGB8888 => 32,
            PixelFormat::ARGB8888 => 32,
            
        }
    }

    pub fn size(&self) -> usize {
        match self {
            PixelFormat::XRGB8888 => 4,
            PixelFormat::ARGB8888 => 4,
        }
    }
}

impl From<u32> for PixelFormat {
    fn from(format: u32) -> Self {
        match format {
            GBM_BO_FORMAT_XRGB8888 => PixelFormat::XRGB8888,
            GBM_BO_FORMAT_ARGB8888 => PixelFormat::ARGB8888,
            _ => PixelFormat::XRGB8888,
        }
    }
}

impl From<i32> for PixelFormat {
    fn from(format: i32) -> Self {
        match format as u32 {
            GBM_BO_FORMAT_XRGB8888 => PixelFormat::XRGB8888,
            GBM_BO_FORMAT_ARGB8888 => PixelFormat::ARGB8888,
            _ => PixelFormat::XRGB8888,
        }
    }
}

#[derive(Debug, Clone)]
pub enum SubPixel {
    Unknown = 1,
    HorizontalRGB,
    HorizontalBGR,
    VerticalRGB,
    VerticalBGR,
    None,
}

impl From<u32> for SubPixel {
    fn from(subpixel: u32) -> Self {
        match subpixel {
            1 => SubPixel::Unknown,
            2 => SubPixel::HorizontalRGB,
            3 => SubPixel::HorizontalBGR,
            4 => SubPixel::VerticalRGB,
            5 => SubPixel::VerticalBGR,
            6 => SubPixel::None,
            _ => SubPixel::Unknown,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Vendor {
    Unknown = 0,
    AMD = 0x1002,
    Intel = 0x8086,
    Nvidia = 0x10DE,
    ARM = 0x13B5,
    Qualcomm = 0x5143,
    Broadcom = 0x1166,
    Vmware = 0x15AD,
    Google = 0x1AE0,
    Apple = 0x106B,
    Samsung = 0x144D,
    Microsoft = 0x1414,
    ZTE = 0x1CF2,
}

impl From<u32> for Vendor {
    fn from(vendor: u32) -> Self {
        match vendor {
            0x1002 => Vendor::AMD,
            0x8086 => Vendor::Intel,
            0x10DE => Vendor::Nvidia,
            0x13B5 => Vendor::ARM,
            0x5143 => Vendor::Qualcomm,
            0x1166 => Vendor::Broadcom,
            0x15AD => Vendor::Vmware,
            0x1AE0 => Vendor::Google,
            0x106B => Vendor::Apple,
            0x144D => Vendor::Samsung,
            0x1414 => Vendor::Microsoft,
            0x1CF2 => Vendor::ZTE,
            _ => Vendor::Unknown,
        }
    }
}

impl From<u16> for Vendor {
    fn from(vendor: u16) -> Self {
        match vendor {
            0x1002 => Vendor::AMD,
            0x8086 => Vendor::Intel,
            0x10DE => Vendor::Nvidia,
            0x13B5 => Vendor::ARM,
            0x5143 => Vendor::Qualcomm,
            0x1166 => Vendor::Broadcom,
            0x15AD => Vendor::Vmware,
            0x1AE0 => Vendor::Google,
            0x106B => Vendor::Apple,
            0x144D => Vendor::Samsung,
            0x1414 => Vendor::Microsoft,
            0x1CF2 => Vendor::ZTE,
            _ => Vendor::Unknown,
        }
    }
}


#[derive(Debug, Copy, Clone)]
pub enum BufferFlag {
    /// Buffer is going to be used as cursor
    Cursor,
    /// Buffer is linear, i.e. not tiled.
    Linear,
    /// Buffer is protected, i.e. encrypted and not readable by CPU or 
    /// any other non-secure / non-trusted components nor by non-trusted OpenGL, OpenCL, and Vulkan applications.
    Protected,
    ///	Buffer is to be used for rendering - for example it is going to be used as the storage for a color buffer
    Rendering,
    /// Buffer is going to be used for scanout - for example it is going to be used to display something on the screen
    Scanout
}


pub enum Planes {

    /// Primary plane
    /// This is buffer is used to display the primary plane, background.
    Background = 1,
    
    /// Cursor plane
    /// This is buffer used to display the cursor.
    /// This plane is always visible.
    Cursor,

    /// Overlay plane
    /// This is buffer used to display the overlay plane.
    /// This plane is always visible, but does not overlap the cursor.
    Overlay,

    /// Normal plane
    /// This is buffer used to display the normal plane.
    /// This plane can be overlapped by another plane and is not always visible.
    Normal,
}

impl From<i32> for Planes {
    fn from(value: i32) -> Self {
        match value {
            1 => Self::Background,
            2 => Self::Cursor,
            3 => Self::Overlay,
            4 => Self::Normal,
            _ => panic!("Invalid plane id"),
        }
    }
}


#[derive(Debug, Copy, Clone)]
pub enum SurfaceTransform {
    Normal = 0,
    Rotate90,
    Rotate180,
    Rotate270,
    FlipHorizontal,
    FlipVertical,
    Rotate90FlipHorizontal,
    Rotate90FlipVertical,
}