use std::{env, error::Error, fs};

const CASE_SENSITIVE: &str = "CASE_SENSITIVE";

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for (line, content) in results {
        println!("[{line}]: {content}")
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    contents
        .lines()
        .enumerate()
        .filter(|(_, content)| content.contains(query))
        .map(|(line, content)| (line + 1, content))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    contents
        .lines()
        .enumerate()
        .filter(|(_, content)| content.to_lowercase().contains(&query.to_lowercase()))
        .map(|(line, content)| (line + 1, content))
        .collect()
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(q) => q,
            None => return Err("Provide a query"),
        };
        let file_path = match args.next() {
            Some(p) => p,
            None => return Err("Provide a path"),
        };

        let case_sensitive = env::var(CASE_SENSITIVE).is_err();

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

        assert_eq!(
            vec![(2, "safe, fast and productive.")],
            search(query, content)
        )
    }

    #[test]
    fn one_result() {
        let query = "RusT";

        let content = "\
    Rust:
    safe, fast and productive.
    pick three.
        ";

        assert_eq!(vec![(1, "Rust:")], search_case_insensitive(query, content))
    }

    #[test]
    fn case_insensitive() {
        let query = "Arnold";

        let contents = "\
Hello There,
I am Arnold,
A software engineer.
My username is arnold.
        ";

        assert_eq!(
            vec![(2, "I am Arnold,"), (4, "My username is arnold.")],
            search_case_insensitive(query, contents)
        )
    }
}
