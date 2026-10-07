use log_analayzer::{formatter::{filter_by_level, print_statistic}, parser::core::{LogLevel}, reader::file_decompositor};
use std::io;

fn main() {

    let file = loop {
        println!("Пожалуйста введите путь к файлу логов");
        let mut user_input_path = String::new();
        io::stdin().read_line(&mut user_input_path).expect("Incorrect format");
        let user_input_path = user_input_path.trim();

        match file_decompositor(user_input_path) {
            Ok(file) => break file,
            Err(error) => println!("По данному пути не найден файл, проверьте путь и напишите заново. Ошибка {}",error)
            
        };
    };  
    
    loop{
        println!("Введите номер одной из команд\n1.Показать всю статистику\n2.Вывести только ошибки\n3.Вывести только предупреждения\n4.Выход");
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Incorrect format");
        let user_input:u32 = user_input.trim().parse().expect("not a number");
        
        match user_input {
            1 => print_statistic(&file),
            2 => filter_by_level(&file, LogLevel::Error),
            3 => filter_by_level(&file, LogLevel::Warning),
            4 => break,
            _ => println!("Incorrect command, please input correct")
        }

    }
}
