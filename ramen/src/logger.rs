static mut LOGGER: Option<Logger> = None;


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Level {
    Debug,
    Info,
    Warn,
    Error,
}

impl std::fmt::Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let level = match self {
            Level::Debug => "Debug",
            Level::Info => "Info",
            Level::Warn => "Warn",
            Level::Error => "Error",
        };
        write!(f, "{}", level)
    }
}


pub struct Logger {
    level: Level,
    file: std::fs::File,
}

impl Logger {
    pub fn init(level: Level, filename: &str) {
        let file = match std::fs::File::options().append(true).create(true).open(filename) {
            Ok(file) => file,
            Err(e) => panic!("Failed to create log file: {}", e),
        };
        unsafe { LOGGER = Some(Logger { level, file }); }
    }

    pub fn instance() -> &'static mut Logger {
        unsafe { LOGGER.as_mut().unwrap() }
    }

    pub fn level(&self) -> &Level {
        &self.level
    }

    pub fn file_mut(&mut self) -> &mut std::fs::File {
        &mut self.file
    }

    pub fn log(&mut self, level: Level, filename: Option<&'static str>, line: u32, message: &str) {
        use std::io::Write;
        if level >= self.level {
            let date = chrono::offset::Local::now().format("%Y-%m-%d %H:%M:%S");
            let filename = if filename.is_some() { format!("[{}:{}]", filename.unwrap(), line) } else { String::new() };

            let mut message_formated = format!("{} [RAMEN] [{}] - {}\n", date, level, message);
            if level == Level::Debug {
                message_formated = format!("{} [RAMEN] [{}] {} - {}\n", date, level, filename, message);
            }

            self.file.write_all(message_formated.as_bytes()).unwrap();
            self.file.sync_all().unwrap();

            let message_formated = match level {
                Level::Debug => format!("{} [RAMEN] [\x1b[34m{}\x1b[0m] {} - {}", date, level, filename, message),
                Level::Info  => format!("{} [RAMEN] [\x1b[32m{}\x1b[0m] - {}", date, level, message),
                Level::Warn  => format!("{} [RAMEN] [\x1b[33m{}\x1b[0m] - {}", date, level, message),
                Level::Error => format!("{} [RAMEN] [\x1b[31m{}\x1b[0m] - {}", date, level, message),
            };

            let mut lock = std::io::stdout().lock();
            writeln!(lock, "{message_formated}").unwrap();
        }
    }
}


#[macro_export]
macro_rules! log {
    ($level:expr, $file:expr, $($arg:tt)*) => ({
        let logger = $crate::logger::Logger::instance();
        logger.log($level, $file, line!(), &format!($($arg)*));
    })
}

/// It should be used to save detailed debugging information and/or a certain system flow.
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => ($crate::log!($crate::logger::Level::Debug, Some(file!()), $($arg)*));
}

/// It should be used for events that are interesting to be observed in the application flow.
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => ($crate::log!($crate::logger::Level::Info, None, $($arg)*));
}

/// It should be used for abnormal behavior in the system that is not necessarily an error.
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => ($crate::log!($crate::logger::Level::Warn, None, $($arg)*));
}

/// It should be used for runtime errors that normally don't need an action at the time they occur but need to be monitored.
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => ($crate::log!($crate::logger::Level::Error, None, $($arg)*));
}



