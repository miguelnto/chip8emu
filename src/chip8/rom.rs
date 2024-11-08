use std::fs::File;
use std::io::prelude::*;

pub fn read_file(filename: &str) -> [u8; 3584] {
        let mut f = File::open(filename).expect("File not found.");
        let mut buffer = [0u8; 3584];

        let _ = f.read(&mut buffer);
        buffer
}

