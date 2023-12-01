use exodus_errors::ErrorKind;

use crate::consts::DEFAULT_BUFFER_SIZE;

#[derive(Debug)]
pub struct NetworkMessage {
    index: usize,
    buffer: Vec<u8>
}

impl Default for NetworkMessage {
    fn default() -> Self {
        Self { index: 3, buffer: Vec::with_capacity(DEFAULT_BUFFER_SIZE) }
    }
}

impl NetworkMessage {
    #[inline]
    pub fn get_index(&self) -> usize { self.index }

    #[inline]
    pub fn get_buffer(&mut self) -> &mut Vec<u8> { &mut self.buffer }

    pub fn write_protocol(&mut self, protocol: u32) {
        let bytes = protocol.to_le_bytes();
        self.buffer.insert(0, bytes[0]);
        self.buffer.insert(1, bytes[1]);
        self.buffer.insert(2, bytes[2]);
        self.buffer.insert(3, bytes[3]);
    }

    pub fn get_protocol(&self) -> Result<u32, ErrorKind> {
        if self.buffer.len() >= 4 {
            let value = u32::from_le_bytes([self.buffer[0], self.buffer[1], self.buffer[2], self.buffer[3]]);
            return Ok(value);
        } 
        Err(ErrorKind::EXODUS_NETWORK_MESSAGE_OVERFLOW)
    }

    pub fn read_u8(&mut self) -> Result<u8, ErrorKind> {
        if self.buffer.is_empty() {
            return Err(ErrorKind::EXODUS_NETWORK_MESSAGE_EMPTY);
        }

        if self.index < self.buffer.len() {
            let value = self.buffer[self.index];
            self.index += 1;
            return Ok(value);
        }
        Err(ErrorKind::EXODUS_NETWORK_MESSAGE_OVERFLOW)
    }

    pub fn read_u16(&mut self) -> Result<u16, ErrorKind> {
        if self.buffer.is_empty() {
            return Err(ErrorKind::EXODUS_NETWORK_MESSAGE_EMPTY);
        }

        if self.index + 1 < self.buffer.len() {
            let value = u16::from_le_bytes([self.buffer[self.index], self.buffer[self.index + 1]]);
            self.index += 2;
            return Ok(value);
        } 
        Err(ErrorKind::EXODUS_NETWORK_MESSAGE_OVERFLOW)
    }

    pub fn read_u32(&mut self) -> Result<u32, ErrorKind> {
        if self.buffer.is_empty() {
            return Err(ErrorKind::EXODUS_NETWORK_MESSAGE_EMPTY);
        }

        if self.index + 3 < self.buffer.len() {
            let value = u32::from_le_bytes([self.buffer[self.index], self.buffer[self.index + 1], self.buffer[self.index + 2], self.buffer[self.index + 3]]);
            self.index += 4;
            return Ok(value);
        } 
        Err(ErrorKind::EXODUS_NETWORK_MESSAGE_OVERFLOW)
    }

    pub fn read_u64(&mut self) -> Result<u64, ErrorKind> {
        if self.buffer.is_empty() {
            return Err(ErrorKind::EXODUS_NETWORK_MESSAGE_EMPTY);
        }

        if self.index + 7 < self.buffer.len() {
            let value = u64::from_le_bytes([self.buffer[self.index], self.buffer[self.index + 1], self.buffer[self.index + 2], self.buffer[self.index + 3], self.buffer[self.index + 4], self.buffer[self.index + 5], self.buffer[self.index + 6], self.buffer[self.index + 7]]);
            self.index += 8;
            return Ok(value);
        } 
        Err(ErrorKind::EXODUS_NETWORK_MESSAGE_OVERFLOW)
    }

    pub fn read_i8(&mut self) -> Result<i8, ErrorKind> {
        if self.buffer.is_empty() {
            return Err(ErrorKind::EXODUS_NETWORK_MESSAGE_EMPTY);
        }

        if self.index < self.buffer.len() {
            let value = self.buffer[self.index] as i8;
            self.index += 1;
            return Ok(value);
        } 
        Err(ErrorKind::EXODUS_NETWORK_MESSAGE_OVERFLOW)
    }

    pub fn read_i16(&mut self) -> Result<i16, ErrorKind> {
        if self.buffer.is_empty() {
            return Err(ErrorKind::EXODUS_NETWORK_MESSAGE_EMPTY);
        }

        if self.index + 1 < self.buffer.len() {
            let value = i16::from_le_bytes([self.buffer[self.index], self.buffer[self.index + 1]]);
            self.index += 2;
            return Ok(value);
        } 
        Err(ErrorKind::EXODUS_NETWORK_MESSAGE_OVERFLOW)
    }

    pub fn read_i32(&mut self) -> Result<i32, ErrorKind> {
        if self.buffer.is_empty() {
            return Err(ErrorKind::EXODUS_NETWORK_MESSAGE_EMPTY);
        }

        if self.index + 3 < self.buffer.len() {
            let value = i32::from_le_bytes([self.buffer[self.index], self.buffer[self.index + 1], self.buffer[self.index + 2], self.buffer[self.index + 3]]);
            self.index += 4;
            return Ok(value);
        } 
        Err(ErrorKind::EXODUS_NETWORK_MESSAGE_OVERFLOW)
    }

    pub fn read_i64(&mut self) -> Result<i64, ErrorKind> {
        if self.buffer.is_empty() {
            return Err(ErrorKind::EXODUS_NETWORK_MESSAGE_EMPTY);
        }

        if self.index + 7 < self.buffer.len() {
            let value = i64::from_le_bytes([self.buffer[self.index], self.buffer[self.index + 1], self.buffer[self.index + 2], self.buffer[self.index + 3], self.buffer[self.index + 4], self.buffer[self.index + 5], self.buffer[self.index + 6], self.buffer[self.index + 7]]);
            self.index += 8;
            return Ok(value);
        } 
        Err(ErrorKind::EXODUS_NETWORK_MESSAGE_OVERFLOW)
    }

    pub fn read_f32(&mut self) -> Result<f32, ErrorKind> {
        if self.buffer.is_empty() {
            return Err(ErrorKind::EXODUS_NETWORK_MESSAGE_EMPTY);
        }

        if self.index + 3 < self.buffer.len() {
            let value = f32::from_le_bytes([self.buffer[self.index], self.buffer[self.index + 1], self.buffer[self.index + 2], self.buffer[self.index + 3]]);
            self.index += 4;
            return Ok(value);
        } 
        Err(ErrorKind::EXODUS_NETWORK_MESSAGE_OVERFLOW)
    }

    pub fn read_f64(&mut self) -> Result<f64, ErrorKind> {
        if self.buffer.is_empty() {
            return Err(ErrorKind::EXODUS_NETWORK_MESSAGE_EMPTY);
        }

        if self.index + 7 < self.buffer.len() {
            let value = f64::from_le_bytes([self.buffer[self.index], self.buffer[self.index + 1], self.buffer[self.index + 2], self.buffer[self.index + 3], self.buffer[self.index + 4], self.buffer[self.index + 5], self.buffer[self.index + 6], self.buffer[self.index + 7]]);
            self.index += 8;
            return Ok(value);
        } 
        Err(ErrorKind::EXODUS_NETWORK_MESSAGE_OVERFLOW)
    }

    pub fn read_string(&mut self, len: usize) -> Result<String, ErrorKind> {
        if self.buffer.is_empty() {
            return Err(ErrorKind::EXODUS_NETWORK_MESSAGE_EMPTY);
        }

        if self.index + len < self.buffer.len() {
            let mut string = String::new();
            for i in 0..len {
                string.push(self.buffer[self.index + i] as char);
            }
            self.index += len;
            return Ok(string);
        } 
        Err(ErrorKind::EXODUS_NETWORK_MESSAGE_OVERFLOW)
    }

    pub fn write_u8(&mut self, value: u8) {
        self.buffer.push(value);
    }

    pub fn write_u16(&mut self, value: u16) {
        let bytes = value.to_le_bytes();
        self.buffer.push(bytes[0]);
        self.buffer.push(bytes[1]);
    }

    pub fn write_u32(&mut self, value: u32) {
        let bytes = value.to_le_bytes();
        self.buffer.push(bytes[0]);
        self.buffer.push(bytes[1]);
        self.buffer.push(bytes[2]);
        self.buffer.push(bytes[3]);
    }

    pub fn write_u64(&mut self, value: u64) {
        let bytes = value.to_le_bytes();
        self.buffer.push(bytes[0]);
        self.buffer.push(bytes[1]);
        self.buffer.push(bytes[2]);
        self.buffer.push(bytes[3]);
        self.buffer.push(bytes[4]);
        self.buffer.push(bytes[5]);
        self.buffer.push(bytes[6]);
        self.buffer.push(bytes[7]);
    }

    pub fn write_i8(&mut self, value: i8) {
        let bytes = value.to_le_bytes();
        self.buffer.push(bytes[0]);
    }

    pub fn write_i16(&mut self, value: i16) {
        let bytes = value.to_le_bytes();
        self.buffer.push(bytes[0]);
        self.buffer.push(bytes[1]);
    }

    pub fn write_i32(&mut self, value: i32) {
        let bytes = value.to_le_bytes();
        self.buffer.push(bytes[0]);
        self.buffer.push(bytes[1]);
        self.buffer.push(bytes[2]);
        self.buffer.push(bytes[3]);
    }

    pub fn write_i64(&mut self, value: i64) {
        let bytes = value.to_le_bytes();
        self.buffer.push(bytes[0]);
        self.buffer.push(bytes[1]);
        self.buffer.push(bytes[2]);
        self.buffer.push(bytes[3]);
        self.buffer.push(bytes[4]);
        self.buffer.push(bytes[5]);
        self.buffer.push(bytes[6]);
        self.buffer.push(bytes[7]);
    }

    pub fn write_f32(&mut self, value: f32) {
        let bytes = value.to_le_bytes();
        self.buffer.push(bytes[0]);
        self.buffer.push(bytes[1]);
        self.buffer.push(bytes[2]);
        self.buffer.push(bytes[3]);
    }

    pub fn write_f64(&mut self, value: f64) {
        let bytes = value.to_le_bytes();
        self.buffer.push(bytes[0]);
        self.buffer.push(bytes[1]);
        self.buffer.push(bytes[2]);
        self.buffer.push(bytes[3]);
        self.buffer.push(bytes[4]);
        self.buffer.push(bytes[5]);
        self.buffer.push(bytes[6]);
        self.buffer.push(bytes[7]);
    }

    pub fn write_string_utf16(&mut self, string: &str) {
        for c in string.chars() {
            self.buffer.push(c as u8);
        }
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.buffer.push(*byte);
        }
    }

    pub fn clear(&mut self) { 
        self.index = 0;
        self.buffer.clear();
    }

    pub fn reset(&mut self) {
        self.index = 0;
    }
}
