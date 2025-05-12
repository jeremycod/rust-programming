use crate::analyzer::logs::Log;
use crate::analyzer::config::LogLevel;

pub fn filter_by_level(content: &str, log_level: LogLevel) -> Vec<Log> {
    let level: String = log_level.to_string();
    content
        .lines()
        .filter(|line| line.contains(level.as_str()))
        .filter_map(|line| Log::from_line(&line.to_string()))
        .collect()
}