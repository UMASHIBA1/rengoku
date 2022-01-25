use std::path::Path;
use std::fs::{read, write, create_dir_all};


// NOTE: the file path expect path from directory root(e.g. ./src/loader/test_data/simple_module.wat)
pub fn wat_to_wasm(input_path: String, output_path: String) {

    let binary = wat::parse_file(&input_path);

    match binary {
        Ok(binary) => {

            let write_result = write(&output_path, binary);
            if write_result.is_err() {
                panic!("failed to write {}", output_path)
            }

        },
        Err(err) => {
                panic!("failed to compile {} by this error: {}", input_path, err)

        }
    };


}