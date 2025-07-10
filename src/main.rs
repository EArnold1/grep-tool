mod utils;

use std::process;

use minigrep::Config;

use utils::args::parse_args;

fn main() {
    let args = parse_args().unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1)
    });

    let is_case_insensitive = args.case_insensitive;

    let config = Config::new(args.query, args.file_path)
        .unwrap_or_else(|err| {
            eprintln!("{err}");
            process::exit(1)
        })
        .case_insensitive(is_case_insensitive)
        .build();

    println!(
        "searching for {} in {}...",
        config.pattern, config.file_path
    );

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application Error: {}", e);
        process::exit(1)
    }
}
