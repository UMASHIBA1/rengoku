use std::path::Path;
use std::fs::{read, write, create_dir_all};

pub fn wat_to_wasm(input_path: String, output_path: String) {

    let wat = match read(&input_path){
        Ok(wat) => wat,
        Err(err) => {
            panic!("failed to load {} file", input_path)
        }
    };


    let binary = wat::parse_file(wat);

    match binary {
        Ok(binary) => {

            let create_dir_result = create_dir_all(&output_path);
            if create_dir_result.is_err() {
                panic!("failed to create dirs {}", output_path)
            }

            let write_result = write(&output_path, binary);
            if write_result.is_err() {
                panic!("failed to write {}", output_path)
            }

        },
        Err(err) => {
                panic!("failed to compile {}", input_path)

        }
    };


}