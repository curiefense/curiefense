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

#[derive(Debug, Clone, Serialize, PartialEq, Eq, PartialOrd, Ord, Copy)]
#[serde(rename_all = "lowercase")]
#[repr(u8)]
// mapped to nginx log levels
pub enum LogLevel {
    Debug = 0,
    Info = 1,
    Warning = 2,
    Error = 3,
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

impl std::str::FromStr for LogLevel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "debug" => Ok(LogLevel::Debug),
            "info" => Ok(LogLevel::Info),
            "warning" => Ok(LogLevel::Warning),
            "error" => Ok(LogLevel::Error),
            _ => Err(format!("unknown loglevel {}", s)),
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

// this trait is for things that are cheap to pass, and that can be used to build a string
// its main use is to avoid directly passing a possibly expensive format macro to a logging function
pub trait CheapString {
    fn c_to_string(self) -> String;
}

impl CheapString for &str {
    fn c_to_string(self) -> String {
        self.to_string()
    }
}

impl<F> CheapString for F
where
    F: FnOnce() -> String,
{
    fn c_to_string(self) -> String {
        self()
    }
}

impl Logs {
    pub fn new(lvl: LogLevel) -> Self {
        Logs {
            start: Instant::now(),
            level: lvl,
            logs: Vec::new(),
        }
    }

    pub fn log<S: CheapString>(&mut self, level: LogLevel, message: S) {
        if level < self.level {
            return;
        }
        self.logs.push(Log {
            elapsed_micros: self.start.elapsed().as_micros() as u64,
            message: message.c_to_string(),
            level,
        })
    }

    pub fn debug<S: CheapString>(&mut self, message: S) {
        self.log(LogLevel::Debug, message);
    }
    pub fn info<S: CheapString>(&mut self, message: S) {
        self.log(LogLevel::Info, message);
    }
    pub fn warning<S: CheapString>(&mut self, message: S) {
        self.log(LogLevel::Warning, message);
    }
    pub fn error<S: CheapString>(&mut self, message: S) {
        self.log(LogLevel::Error, message);
    }

    pub fn to_stringvec(&self) -> Vec<String> {
        self.logs.iter().map(|l| l.to_string()).collect()
    }

    pub fn extend(&mut self, other: Logs) {
        self.logs.extend(other.logs);
    }

    pub fn to_json(&self) -> serde_json::Value {
        serde_json::to_value(&self.logs).unwrap_or_else(|rr| serde_json::Value::String(rr.to_string()))
    }
}

impl Serialize for Logs {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_seq(self.logs.iter().map(|l| l.to_string()))
    }
}
