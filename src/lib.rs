use regex::{Captures, Regex};
use std::{error::Error, fs};

fn format_color<'a>(str: &'a str, pattern: &'a str) -> std::borrow::Cow<'a, str> {
    use colored::Colorize;
    let re = Regex::new(pattern).unwrap();

    re.replace_all(str, |cap: &Captures| format!("{}", &cap[0].yellow()))
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = search(&config.query, &contents);

    for (line, content) in results {
        println!("[{line}]: {}", format_color(content, &config.query))
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    contents
        .lines()
        .enumerate()
        .filter(|(_, content)| {
            let q_regex = Regex::new(query).unwrap();
            q_regex.is_match(content)
        })
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
    pub case_sensitive: Option<bool>,
}

impl Config {
    pub fn new(args: Config) -> Result<Config, String> {
        // validate search query (regex)
        let mut pattern = args.query;

        let case_sensitive = args.case_sensitive.unwrap_or(false);

        // Create regex
        if Regex::new(&pattern).is_err() {
            return Err("Invalid regex pattern".to_string());
        }

        let case_insensitive = r"(?i)";

        if case_sensitive && !pattern.contains(case_insensitive) {
            pattern = format!("{}{}", case_insensitive, pattern);
        }

        // validate file_path/dir
        let file_path = args.file_path;

        if fs::metadata(&file_path).is_err() {
            return Err(format!("invalid file path {}", &file_path));
        }

        Ok(Config {
            query: pattern,
            file_path,
            case_sensitive: None,
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
