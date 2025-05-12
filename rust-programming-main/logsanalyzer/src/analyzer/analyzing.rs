use std::error::Error;
use std::fs;
use crate::analyzer::config::Config;
use crate::analyzer::filtering::filter_by_level;
use crate::analyzer::logs::Log;

pub fn analyze(config: Config) -> Result<Vec<Log>, Box<dyn Error>> {
    let content = fs::read_to_string(config.path)?;
    let filtered = filter_by_level(&content, config.level);
    Ok(filtered)
}