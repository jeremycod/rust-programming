use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;

pub struct Config {
    pub path: String,
    pub level: LogLevel

}
#[derive(Debug)]
pub(crate) enum LogLevel {
    INFO,
    WARN,
    DEBUG,
    ERROR,
    ALL
}
impl FromStr for LogLevel {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "debug" => Ok(LogLevel::DEBUG),
            "warn" => Ok(LogLevel::WARN),
            "error" => Ok(LogLevel::ERROR),
            "info" => Ok(LogLevel::INFO),
            _ => Ok(LogLevel::ALL),
        }
    }
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let name = match self {
            LogLevel::INFO => "INFO",
            LogLevel::ERROR => "ERROR",
            LogLevel::WARN => "WARN",
            LogLevel::DEBUG => "DEBUG",
            LogLevel::ALL => "ALL"
        };
        write!(f, "{}", name)
    }
}
impl Config {
    pub fn new(args: &[String]) -> Result< Config, &str> {
        if args.len() < 3 {
            return Err("Missing arguments.")
        }
        let path = args[1].clone();
        let level = LogLevel::from_str(args[2].as_str()).unwrap_or(LogLevel::ALL);
        Ok(Config {
            path,
            level
        })
    }
}
