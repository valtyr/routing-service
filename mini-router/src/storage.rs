use std::fs::File;

use crate::types::TOSMFile;

pub fn read_file(path: String) -> TOSMFile {
    let in_file = File::open(path).unwrap();
    let file: TOSMFile = bincode::deserialize_from(in_file).unwrap();
    file
}

pub fn write_file(path: String, file: &TOSMFile) {
    let out_file = File::create(path).unwrap();
    bincode::serialize_into(out_file, &file).unwrap();
}
