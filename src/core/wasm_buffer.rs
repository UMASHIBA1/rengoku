use std::fmt;
use std::error;

#[derive(Debug)]
pub enum WasmBufferError {
    ReadByteThatDoesNotExistError
}

impl fmt::Display for WasmBufferError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WasmBufferError::ReadByteThatDoesNotExistError => write!(f, "read byte that doesn't exist")
        }
    }
}

impl error::Error for WasmBufferError {}

#[derive(Debug)]
pub struct WasmBuffer {
    position: usize,
    buffer: Vec<u8>,
}

impl WasmBuffer {
    pub fn read_bytes(&mut self, size: usize) -> Result<Vec<u8>, WasmBufferError> {
        let will_read_end_position = self.position + size;
        if will_read_end_position <= self.buffer.len() {
            let reading_bytes = self.buffer[self.position..will_read_end_position];
            self.position += size;
            Ok(reading_bytes.to_vec())
        } else {
            Err(WasmBufferError::ReadByteThatDoesNotExistError)
        }
    }
}

pub fn create_wasm_buffer(buffer: Vec<u8>) -> Box<WasmBuffer> {
    Box::new(WasmBuffer {
        position: 0,
        buffer,
    })
}
