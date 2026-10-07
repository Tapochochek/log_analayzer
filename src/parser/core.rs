#[derive(Debug, PartialEq, Eq)]
pub enum LogLevel{
    Info,
    Warning,
    Error
}

pub struct LogEntry{
    pub timestamp:String,
    pub level:LogLevel,
    pub message:String
}

impl LogEntry{
    pub fn new(raw_string:&String)->Self{
        let  bytes = raw_string.as_bytes();
        let mut words = Vec::new();

        let mut low_counter = 0;
        let mut word_count = 0;

        for (i, &item) in bytes.iter().enumerate(){
            if item == b']'{
                words.push(&raw_string[low_counter..i+1]);
                low_counter = i+2;
                word_count += 1;                
                if word_count == 2{
                    words.push(&raw_string[low_counter..]);
                }
            }

        }

        match words[1]{
            "[WARNING]" => Self { timestamp: words[0].to_string(), level: LogLevel::Warning, message: words[2].to_string() },
            "[ERROR]" => Self { timestamp: words[0].to_string(), level: LogLevel::Error, message: words[2].to_string() },
            _ => Self { timestamp: words[0].to_string(), level: LogLevel::Info, message: words[2].to_string() }
        }
        
        
    }
}