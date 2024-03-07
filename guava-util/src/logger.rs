use std::fs::File;
use std::fs::OpenOptions;
use std::io::Seek;
use std::path::Path;

pub trait ILogger {
    fn log(&self, msg: &str);
    fn logln(&self, msg: &str) {
        self.log(msg);
        self.log("\n");
    }
}

pub struct ConsoleLogger;

impl ConsoleLogger {
    fn new() -> Self {
        ConsoleLogger {}
    }
}

impl ILogger for ConsoleLogger {
    fn log(&self, msg: &str) {
        println!("{}", msg);
    }
}
