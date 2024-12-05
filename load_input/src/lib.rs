use std::fs::File;
use std::io::Read;
use std::io;


pub extern fn read_file_contents(filename: &str) -> io::Result<String> {
    #![allow(improper_ctypes_definitions)]
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}