use exodus_errors::ErrorKind;

use crate::protocol_code::ProtocolCode;

pub const DEFAULT_BUFFER_SIZE: usize          = 0x200;

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

    pub fn new(protocol: ProtocolCode) -> Self {
        let mut msg = Self::default();
        msg.write_protocol(protocol as i32);
        msg
    }

    #[inline]
    pub fn get_index(&self) -> usize { self.index }

    #[inline]
    pub fn get_buffer(&mut self) -> &mut Vec<u8> { &mut self.buffer }

    pub fn write_protocol(&mut self, protocol: i32) {
        let bytes = protocol.to_le_bytes();
        self.buffer.insert(0, bytes[0]);
        self.buffer.insert(1, bytes[1]);
        self.buffer.insert(2, bytes[2]);
        self.buffer.insert(3, bytes[3]);
    }

    pub fn code(&self) -> Result<i32, ErrorKind> {
        if self.buffer.len() >= 4 {
            let value = i32::from_le_bytes([self.buffer[0], self.buffer[1], self.buffer[2], self.buffer[3]]);
            return Ok(value);
        }

        Err(ErrorKind::NETWORKMESSAGE_FAILED)
    }

    pub fn read_u8(&mut self) -> Result<u8, ErrorKind> {
        if self.buffer.is_empty() {
            return Err(ErrorKind::NETWORKMESSAGE_EMPTY);
        }

        let value = self.buffer[self.index];
        self.index += 1;
        Ok(value)
    }

    pub fn read_u16(&mut self) -> Result<u16, ErrorKind> {
        if self.buffer.is_empty() || self.index + 1 >= self.buffer.len() {
            return Err(ErrorKind::NETWORKMESSAGE_EMPTY);
        }

        let value = u16::from_le_bytes([self.buffer[self.index], self.buffer[self.index + 1]]);
        self.index += 2;
        Ok(value)
    }

    pub fn read_u32(&mut self) -> Result<u32, ErrorKind> {
        if self.buffer.is_empty() || self.index + 3 >= self.buffer.len() {
            return Err(ErrorKind::NETWORKMESSAGE_EMPTY);
        }

        let value = u32::from_le_bytes([self.buffer[self.index], self.buffer[self.index + 1], self.buffer[self.index + 2], self.buffer[self.index + 3]]);
        self.index += 4;
        Ok(value)
    }

    pub fn read_u64(&mut self) -> Result<u64, ErrorKind> {
        if self.buffer.is_empty() || self.index + 7 >= self.buffer.len() {
            return Err(ErrorKind::NETWORKMESSAGE_EMPTY);
        }

        let value = u64::from_le_bytes([self.buffer[self.index], self.buffer[self.index + 1], self.buffer[self.index + 2], self.buffer[self.index + 3], self.buffer[self.index + 4], self.buffer[self.index + 5], self.buffer[self.index + 6], self.buffer[self.index + 7]]);
        self.index += 8;
        Ok(value)
    }

    pub fn read_i8(&mut self) -> Result<i8, ErrorKind> {
        if self.buffer.is_empty() {
            return Err(ErrorKind::NETWORKMESSAGE_EMPTY);
        }

        let value = i8::from_le_bytes([self.buffer[self.index]]);
        self.index += 1;
        Ok(value)
    }

    pub fn read_i16(&mut self) -> Result<i16, ErrorKind> {
        if self.buffer.is_empty() || self.index + 1 >= self.buffer.len() {
            return Err(ErrorKind::NETWORKMESSAGE_EMPTY);
        }

        let value = i16::from_le_bytes([self.buffer[self.index], self.buffer[self.index + 1]]);
        self.index += 2;
        Ok(value)
    }

    pub fn read_i32(&mut self) -> Result<i32, ErrorKind> {
        if self.buffer.is_empty() || self.index + 3 >= self.buffer.len() {
            return Err(ErrorKind::NETWORKMESSAGE_EMPTY);
        }

        let value = i32::from_le_bytes([self.buffer[self.index], self.buffer[self.index + 1], self.buffer[self.index + 2], self.buffer[self.index + 3]]);
        self.index += 4;
        Ok(value)
    }

    pub fn read_i64(&mut self) -> Result<i64, ErrorKind> {
        if self.buffer.is_empty() || self.index + 7 >= self.buffer.len() {
            return Err(ErrorKind::NETWORKMESSAGE_EMPTY);
        }

        let value = i64::from_le_bytes([self.buffer[self.index], self.buffer[self.index + 1], self.buffer[self.index + 2], self.buffer[self.index + 3], self.buffer[self.index + 4], self.buffer[self.index + 5], self.buffer[self.index + 6], self.buffer[self.index + 7]]);
        self.index += 8;
        Ok(value)
    }

    pub fn read_f32(&mut self) -> Result<f32, ErrorKind> {
        if self.buffer.is_empty() || self.index + 3 >= self.buffer.len() {
            return Err(ErrorKind::NETWORKMESSAGE_EMPTY);
        }

        let value = f32::from_le_bytes([self.buffer[self.index], self.buffer[self.index + 1], self.buffer[self.index + 2], self.buffer[self.index + 3]]);
        self.index += 4;
        Ok(value)
    }

    pub fn read_f64(&mut self) -> Result<f64, ErrorKind> {
        if self.buffer.is_empty() || self.index + 7 >= self.buffer.len() {
            return Err(ErrorKind::NETWORKMESSAGE_EMPTY);
        }

        let value = f64::from_le_bytes([self.buffer[self.index], self.buffer[self.index + 1], self.buffer[self.index + 2], self.buffer[self.index + 3], self.buffer[self.index + 4], self.buffer[self.index + 5], self.buffer[self.index + 6], self.buffer[self.index + 7]]);
        self.index += 8;
        Ok(value)
    }
    
    pub fn read_bytes(&mut self, length: usize) -> Result<Vec<u8>, ErrorKind> {
        if self.buffer.is_empty() || self.index + length >= self.buffer.len() {
            return Err(ErrorKind::NETWORKMESSAGE_EMPTY);
        }

        let bytes = Vec::from(&self.buffer[self.index..self.index + length]);
        Ok(bytes)
    }

    pub fn read_string_utf16(&mut self) -> Result<String, ErrorKind> {
        let length = self.read_u32()? as usize;
        let mut bytes = Vec::with_capacity(length);
        for _ in 0..length {
            bytes.push(self.read_u16()?);
        }
        let string = String::from_utf16(&bytes).unwrap();
        Ok(string)
    }

    pub fn read_string_utf8(&mut self) -> Result<String, ErrorKind> {
        let length = self.read_u32()? as usize;
        let mut bytes = Vec::with_capacity(length);
        for _ in 0..length {
            bytes.push(self.read_u8()?);
        }
        let string = String::from_utf8(bytes).unwrap();
        Ok(string)
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
        let bytes = string.encode_utf16().collect::<Vec<u16>>();
        self.write_u32(bytes.len() as u32);
        for byte in bytes {
            self.write_u16(byte);
        }
    }

    pub fn write_string_utf8(&mut self, string: &str) {
        let bytes = string.as_bytes();
        self.write_u32(bytes.len() as u32);
        for byte in bytes {
            self.write_u8(*byte);
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
