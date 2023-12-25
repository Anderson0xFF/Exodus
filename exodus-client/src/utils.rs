use exodus_common::enums::*;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Display {
    pub id: i32,
    pub gpu: i32,
    pub gpu_count: u32,
    pub gpus: Vec<i32>,
}

impl Display {
    pub(crate) fn new(id: i32, gpu: i32, gpu_count: u32, gpus: Vec<i32>) -> Self { Self { id, gpu, gpu_count, gpus } }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct GPU {
    pub id: i32,
    pub vendor: i32,
    pub screen_count: u32,
    pub screens: Vec<i32>,
}

impl GPU {
    pub(crate) fn new(id: i32, vendor: i32, screen_count: u32, screens: Vec<i32>) -> Self { Self { id, vendor, screen_count, screens } }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Screen {
    pub id: i32,
    pub connector_type: ConnectorType,
    pub mm_width: u32,
    pub mm_height: u32,
    pub subpixel: SubPixel,
    pub mode: u32,
    pub modes_count: u32,
    pub modes: Vec<u32>,
}

impl Screen {
    pub(crate) fn new(id: i32, connector_type: ConnectorType, mm_width: u32, mm_height: u32, subpixel: SubPixel, mode: u32, modes_count: u32, modes: Vec<u32>) -> Self { Self { id, connector_type, mm_width, mm_height, subpixel, mode, modes_count, modes } }
}