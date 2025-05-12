mod analyzer;

use std::env;
use analyzer::analyzing::analyze;
use crate::analyzer::config::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });
    let results = analyze(config);
    if let Ok(logs) = results {
        for log in logs {
            eprintln!("{}", log);
        }
    } else {
        eprintln!("Failed to process logs");
    }
    
}
