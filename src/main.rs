use std::{env, process};

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing args: {}", err);
        process::exit(1)
    });

    println!("searching for {} in {}", config.query, config.file_path);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application Error: {}", e);
        process::exit(1)
    }
}
