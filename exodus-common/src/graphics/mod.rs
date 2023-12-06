pub mod device;
pub mod screen;
pub mod crtcs;
pub mod encoders;
pub mod surface;
pub mod buffer;
pub mod framebuffer;

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