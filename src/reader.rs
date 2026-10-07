use std::fs;

use crate::parser::core::LogEntry;

pub fn file_decompositor(file_path:&String)-> Vec<LogEntry>{
    let file: Result<String, std::io::Error> = fs::read_to_string(file_path);
    let check_file = match file{
        Ok(check_file) => check_file,
        Err(_) => panic!("Can't read file")
    };
    let mut vector_struct = Vec::new();
    for line in check_file.lines(){
        vector_struct.push(LogEntry::new(&line.to_string()));
    }
    vector_struct
}