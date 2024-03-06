use std::fs::File;
use std::path::Path;

pub trait ILogger {
    fn log(&self, msg: &str);
    fn logln(&self, msg: &str) {
        self.log(msg);
        self.log("\n");
    }
}

pub struct ConsoleLogger {

}

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

pub struct FileLogger {
    file: Option<File>
}

impl FileLogger {
    fn open_new(path: &str) -> Self {
        let file = match File::open(&Path::new(path)) {
            Err(e) => panic!("FileLogger open error: {}", e),
            Ok(file) => file
        };
        FileLogger {
            file: Some(file)
        }
    }

    fn new() -> Self {
        FileLogger {
            file: None
        }
    }

    fn open(mut self, path: &str) {
        if self.file.is_none() {
            self.file = Some(match File::open(&Path::new(path)) {
                Err(e) => panic!("FileLogger open error: {}", e),
                Ok(file) => file
            });
        }
    }

    fn close(mut self) {
        todo!()
    }

}

impl ILogger for FileLogger {
    fn log(&self, msg: &str) {
        todo!()
    }
}
