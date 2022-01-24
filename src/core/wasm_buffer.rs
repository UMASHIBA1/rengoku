struct WasmBuffer {
    position: usize,
    buffer: Vec<u8>,
}

impl WasmBuffer {
    pub fn read_bytes(&mut self, size: usize) -> Vec<u8> {
        let will_read_end_position = self.position + size;
        if will_read_end_position <= self.buffer.len() {
            let reading_bytes = self.buffer[self.position..will_read_end_position];
            self.position += size;
            reading_bytes.to_vec()
        } else {
            Vec::new()
        }
    }
}

pub fn create_wasm_buffer(buffer: Vec<u8>) -> Box<WasmBuffer> {
    Box::new(WasmBuffer {
        position: 0,
        buffer,
    })
}
