use std::fs::{ self, File };
use std::path::{ Path };
use std::io::Write;
use chrono::{ Utc };


#[derive(Debug)]
#[allow(dead_code)]
pub enum LogType {
    Info,
    Warn,
    Err
}

pub struct Logger {
    file_name: String,
}

impl Logger {
    pub fn new(file_name: &str) -> Logger {
        let new_path = Path::new(file_name);
        if !new_path.exists() {
            File::create(new_path).unwrap();
        }
        Logger { file_name: String::from(file_name) }
    }

    pub fn log(&self, log_type: LogType, text: String) {
        let now = Utc::now().format("%Y-%m-%dT%T");
        let log_string = format!("{} [{}] {}\n", now, format!("{:?}", log_type).to_lowercase(), text);

        let mut file = fs::OpenOptions::new().write(true).append(true).open(self.file_name.to_string()).unwrap();
        write!(file, "{}", log_string).unwrap();
    }
}
