use crate::logger::logger::Logger;

pub struct ConsoleLogger {}

impl ConsoleLogger {
    pub fn new() -> Self {
        ConsoleLogger {}
    }
}

impl Logger for ConsoleLogger {
    fn log(&self, msg: &str) {
        println!("{}", msg)
    }
} 