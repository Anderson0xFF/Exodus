use std::collections::HashMap;
use libc::c_void;

#[macro_export]
macro_rules! ZeroMemory {
    ($x:expr) => {
        unsafe { std::mem::zeroed::<$x>() }
    };
}


#[derive(Debug)]
pub struct Allocator {
    stack: Option<HashMap<u32, *mut c_void>>,
}

impl Allocator {
    pub const fn new() -> Self {
        Self {
            stack: None
        }
    }

    pub fn set(&mut self, key: u32, ptr: *mut c_void) {
        if let Some(stack) = self.stack.as_mut() {
            stack.insert(key, ptr);
        }
    }

    pub fn free(&mut self, key: u32) {
        if let Some(stack) = self.stack.as_mut() {
            stack.remove(&key);
        }
    }

    pub fn get(&mut self, key: u32) -> Option<*mut c_void> {
        if let Some(stack) = self.stack.as_mut() {
            return stack.get(&key).copied();
        }
        None
    }
}