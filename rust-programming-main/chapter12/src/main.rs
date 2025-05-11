use std::env;
use std::process;
use chapter12::{run, Config};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    eprintln!("Searching for {}", config.query);
    eprintln!("filename: {}", config.filename);
    if let Err(e) = run(config){
        eprintln!("Application error: {}", e);
        process::exit(1);
    }

}


