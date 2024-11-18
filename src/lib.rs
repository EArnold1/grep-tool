use std::{env, error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{line}")
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut lines = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            lines.push(line);
        }
    }

    lines
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut lines = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            lines.push(line);
        }
    }

    lines
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments passed in");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let case_sensitive = env::var("CASE_SENSITIVE").is_err();

        Ok(Config {
            query,
            file_path,
            case_sensitive,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";

        let content = "\
Rust:
safe, fast and productive.
pick three.
    ";

        assert_eq!(vec!["safe, fast and productive."], search(query, content))
    }

    #[test]
    fn one_result() {
        let query = "RusT";

        let content = "\
    Rust:
    safe, fast and productive.
    pick three.
        ";

        assert_eq!(vec!["Rust:"], search_case_insensitive(query, content))
    }
}
