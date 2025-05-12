use std::fmt;
use std::str::FromStr;
use crate::analyzer::config::LogLevel;
use regex::Regex;

#[derive(Debug)]
pub struct Log {
    timestamp: String,
    level: LogLevel,
    context: String,
    message: String
}

impl fmt::Display for Log {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {} - {}: {}", self.timestamp, self.level, self.context, self.message)
    }
}
impl Log {
    pub fn from_line(line: &String) -> Option<Log> {
        let re1 = Regex::new(r"(?P<timestamp>[\d\-/:,\s]+)\s+-?\s*(?P<level>[A-Z]+)\s+\[(?P<context>[^\]]+)\]\s*-\s*(?P<message>.+)").ok()?;
        let re2 = Regex::new(r"(?P<timestamp>[\d\-/:,\s]+)\s+(?P<level>[A-Z]+)\s+(?P<context>[^:]+):\s+(?P<message>.+)").ok()?;

        let caps = re1.captures(line).or_else(|| re2.captures(line))?;

        Some(Log {
            timestamp: caps["timestamp"].to_string(),
            level: LogLevel::from_str(&caps["level"]).ok()?,
            context: caps["context"].to_string(),
            message: caps["message"].to_string(),
        })
    }
}