use core::error;
use std::fs::{self, TryLockError::Error};

use crate::parser::core::LogEntry;

pub fn file_decompositor(file_path:&str)-> Result<Vec<LogEntry>, std::io::Error>{
    let file: Result<String, std::io::Error> = fs::read_to_string(file_path);
    let check_file = match file{
        Ok(check_file) => check_file,
        Err(error) => return Err(error) 
    };

    let mut vector_struct = Vec::new();
    for line in check_file.lines(){
        vector_struct.push(LogEntry::new(line));
    }
    Ok(vector_struct)
}