use regex::{Captures, Regex};
use std::{error::Error, fs};

fn format_color<'a>(
    str: &'a str,
    pattern: &'a str,
) -> Result<std::borrow::Cow<'a, str>, Box<dyn Error>> {
    use colored::Colorize;
    let re = Regex::new(pattern).map_err(|_| "malformed regex pattern")?;

    Ok(re.replace_all(str, |cap: &Captures| format!("{}", &cap[0].yellow())))
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = search(&config.pattern, &contents);

    for (line, content) in results {
        println!("[{line}]: {}", format_color(content, &config.pattern)?)
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    contents
        .lines()
        .enumerate()
        .filter(|(_, content)| {
            let q_regex = Regex::new(query).expect("pattern should be correct");
            q_regex.is_match(content)
        })
        .map(|(line, content)| (line + 1, content))
        .collect()
}

pub struct ConfigBuilder {
    pub pattern: String,
    pub file_path: String,
    pub case_insensitive: Option<bool>,
}

impl ConfigBuilder {
    pub fn case_insensitive(&mut self, is_case_insensitive: bool) -> &mut Self {
        let case_insensitive_pattern = r"(?i)";

        if is_case_insensitive && !&self.pattern.contains(case_insensitive_pattern) {
            self.pattern = format!("{}{}", case_insensitive_pattern, &self.pattern);
        }

        self
    }

    pub fn build(&self) -> Config {
        Config {
            pattern: self.pattern.clone(),
            file_path: self.file_path.clone(),
        }
    }
}

pub struct Config {
    pub pattern: String,
    pub file_path: String,
}

impl Config {
    pub fn new(pattern: String, file_path: String) -> Result<ConfigBuilder, String> {
        // validate search query (regex)
        if Regex::new(&pattern).is_err() {
            return Err(format!("invalid regex pattern: {}", pattern));
        }

        if fs::metadata(&file_path).is_err() {
            return Err(format!("invalid file path: {}", &file_path));
        }

        Ok(ConfigBuilder {
            case_insensitive: None,
            file_path,
            pattern,
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
        let query = "(?i)RusT";

        let content = "\
    Rust:
    safe, fast and productive.
    pick three.
        ";

        assert_eq!(vec![(1, "Rust:")], search(query, content))
    }

    #[test]
    fn case_insensitive() {
        let query = "(?i)Arnold";

        let contents = "\
Hello There,
I am Arnold,
A software engineer.
My username is arnold.
        ";

        assert_eq!(
            vec![(2, "I am Arnold,"), (4, "My username is arnold.")],
            search(query, contents)
        )
    }
}
