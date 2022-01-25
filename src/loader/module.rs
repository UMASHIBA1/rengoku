use crate::core::wasm_buffer::{WasmBuffer, WasmBufferError};

pub struct Module {
    magic: Option<Vec<u8>>,
    version: Option<Vec<u8>>,
}

impl Module {

    pub fn load(&mut self, buffer: &mut WasmBuffer) {
        let will_magic_bytes = buffer.read_bytes(4);
        match will_magic_bytes {
            Ok(bytes) => bytes,
            Err(err) => {
                match err {
                    WasmBufferError::ReadByteThatDoesNotExistError => {
                        panic!("Err! you try to read byte that doesn't exist, when reading module's magic bytes");
                    }
                }
            }
        };

        let will_version_bytes = buffer.read_bytes(4);
        match will_version_bytes {
            Ok(bytes) => bytes,
            Err(err) => {
                match err {
                    WasmBufferError::ReadByteThatDoesNotExistError => {
                        panic!("Err! you try to read byte that doesn't exist, when reading module's version bytes");
                    }
                }
            }
        };

    }

}