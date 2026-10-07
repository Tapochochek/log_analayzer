
use crate::parser::core::{LogEntry, LogLevel};

pub fn all_file(log_vector:&Vec<LogEntry>){
    for element in log_vector{
        println!("{}, {:?}, {}",element.timestamp,element.level,element.message);
    }
}

pub fn statistic(log_vector:&Vec<LogEntry>){

    let mut all_log_entry = 0;
    let mut error_count = 0;
    let mut warning_count = 0;
    let mut info_count = 0;

    for element in log_vector{
        all_log_entry += 1;
        match element.level {
            LogLevel::Error => error_count+=1,
            LogLevel::Warning => warning_count+=1,
            _=> info_count += 1,
        }
        
    }

    println!("Всего логов обработано: {}. Ошибок: {}, Предупреждений: {}, Инфо: {}",all_log_entry,error_count,warning_count,info_count);
}
pub fn sorting(log_vector:&Vec<LogEntry>, log_level:LogLevel){
    for element in log_vector.iter().filter(|vec| vec.level == log_level){
        println!("{}, {:?}, {}", element.timestamp, element.level, element.message);
    }
}