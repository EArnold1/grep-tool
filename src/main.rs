mod utils;

use std::process;

use minigrep::Config;

use utils::args::parse_args;

fn main() {
    let args = parse_args().unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1)
    });

    let config = Config::new(Config {
        query: args.query,
        file_path: args.file_path,
        case_sensitive: Some(args.case_sensitive),
    })
    .unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1)
    });

    println!("searching for {} in {}...", config.query, config.file_path);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application Error: {}", e);
        process::exit(1)
    }
}
