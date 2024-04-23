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
    stack: HashMap<u32, *mut c_void>,
}

impl Allocator {
    pub fn new() -> Self {
        Self {
            stack: HashMap::new()
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            stack: HashMap::with_capacity(capacity)
        }
    }

    pub fn alloc<T>(&mut self, key: u32, value: T) {
        let ptr = Box::into_raw(Box::new(value));
        self.stack.insert(key, ptr as *mut c_void);
    }

    pub unsafe fn set(&mut self, key: u32, ptr: *mut c_void) {
        self.stack.insert(key, ptr);
    }

    pub fn free(&mut self, key: u32) {
        self.stack.remove(&key);
    }

    pub fn get(&mut self, key: u32) -> Option<&mut *mut c_void>{
        self.stack.get_mut(&key)
    }
}