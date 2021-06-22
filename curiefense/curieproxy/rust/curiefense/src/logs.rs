use serde::Serialize;
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct Logs {
    pub level: LogLevel,
    pub start: Instant,
    pub logs: Vec<Log>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Log {
    pub elapsed_micros: u64,
    pub level: LogLevel,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

impl LogLevel {
    fn short(&self) -> char {
        match self {
            LogLevel::Debug => 'D',
            LogLevel::Info => 'I',
            LogLevel::Warning => 'W',
            LogLevel::Error => 'E',
        }
    }
}

impl std::fmt::Display for Log {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}µs {}", self.level.short(), self.elapsed_micros, self.message)
    }
}

impl Default for Logs {
    fn default() -> Self {
        Logs {
            start: Instant::now(),
            level: LogLevel::Debug,
            logs: Vec::new(),
        }
    }
}

impl Logs {
    pub fn log<S: ToString>(&mut self, level: LogLevel, message: S) {
        let now = Instant::now();
        self.logs.push(Log {
            elapsed_micros: now.duration_since(self.start).as_micros() as u64,
            message: message.to_string(),
            level,
        })
    }

    pub fn debug<S: ToString>(&mut self, message: S) {
        self.log(LogLevel::Debug, message);
    }
    pub fn info<S: ToString>(&mut self, message: S) {
        self.log(LogLevel::Info, message);
    }
    pub fn warning<S: ToString>(&mut self, message: S) {
        self.log(LogLevel::Warning, message);
    }
    pub fn error<S: ToString>(&mut self, message: S) {
        self.log(LogLevel::Error, message);
    }

    pub fn to_stringvec(&self) -> Vec<String> {
        self.logs.iter().map(|l| l.to_string()).collect()
    }
}
