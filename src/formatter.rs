
use crate::parser::core::{LogEntry, LogLevel};

pub fn print_all_log_in_file(log_vector:&[LogEntry]){
    for element in log_vector{
        println!("{}, {:?}, {}",element.timestamp,element.level,element.message);
    }
}

pub fn print_statistic(log_vector:&[LogEntry]){

    let mut error_count = 0;
    let mut warning_count = 0;
    let mut info_count = 0;

    for element in log_vector{
        match element.level {
            LogLevel::Error => error_count+=1,
            LogLevel::Warning => warning_count+=1,
            _=> info_count += 1,
        }      
    }
    println!("Всего логов обработано: {}. Ошибок: {}, Предупреждений: {}, Инфо: {}",log_vector.len(),error_count,warning_count,info_count);

}
pub fn filter_by_level(log_vector:&[LogEntry], log_level:LogLevel){
    for element in log_vector.iter().filter(|vec| vec.level == log_level){
        println!("{}, {:?}, {}", element.timestamp, element.level, element.message);
    }
}