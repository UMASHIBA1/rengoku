use crate::core::wasm_buffer::{WasmBuffer, WasmBufferError};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Module {
    magic: Option<Vec<u8>>,
    version: Option<Vec<u8>>,
}

impl Module {

    pub fn new(magic: Option<Vec<u8>>, version: Option<Vec<u8>>) -> Module {
        Module {
            magic,
            version,
        }
    }


    pub fn load(&mut self, buffer: &mut WasmBuffer) {
        let will_magic_bytes = buffer.read_bytes(4);
        match will_magic_bytes {
            Ok(bytes) => self.magic = Some(bytes),
            Err(err) => {
                match err {
                    WasmBufferError::ReadByteThatDoesNotExistError => {
                        panic!("Err! you try to read byte that doesn't exist, when reading simple_module's magic bytes");
                    }
                }
            }
        };

        let will_version_bytes = buffer.read_bytes(4);
        match will_version_bytes {
            Ok(bytes) => self.version = Some(bytes),
            Err(err) => {
                match err {
                    WasmBufferError::ReadByteThatDoesNotExistError => {
                        panic!("Err! you try to read byte that doesn't exist, when reading simple_module's version bytes");
                    }
                }
            }
        };

    }

}

#[cfg(test)]
mod tests {
    use crate::utils::wat_to_wasm::wat_to_wasm;
    use crate::core::wasm_buffer::create_wasm_buffer;
    use crate::loader::module::module::Module;
    use std::fs::{read};

    const TEST_DATA_BASE_URL: &str = "./src/loader/module/test_data/";

    fn load_and_run_test(file_path: String, expected_module: Module) {
        let wasm = match read(&file_path) {
            Ok(file) => file,
            Err(err) => panic!("can't read file: {}", &file_path)
        };
        let mut buffer = create_wasm_buffer(wasm);
        let mut module = Module::new(None, None);
        module.load(&mut buffer);

        assert_eq!(module, expected_module);
    }

    fn create_expected_module(magic: Vec<u8>, version: Vec<u8>) -> Module {
        Module::new(Some(magic), Some(version))
    }

    #[test]
    fn test_module_load() {
        let output_path = format!("{}{}", TEST_DATA_BASE_URL, "simple_module/simple_module.wasm");
        wat_to_wasm(format!("{}{}", TEST_DATA_BASE_URL, "simple_module/simple_module.wat"), output_path.clone());

        // FYI: https://webassembly.github.io/spec/core/binary/modules.html#binary-module
        let expected_module = create_expected_module(vec![0, 97, 115, 109], vec![1, 0, 0, 0]);

        load_and_run_test(output_path, expected_module);

    }

}