use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments.");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_config_not_enough_arguments_err() {
        let args = [String::from("minigrep")];
        let config = Config::new(&args);

        assert!(config.is_err(), "Not enough arguments.");
    }

    #[test]
    fn new_config_is_ok() {
        let args = [
            String::from("minigrep"),
            String::from("search"),
            String::from("poem.txt")
        ];

        let config = Config::new(&args);

        assert_eq!(config.is_ok(), true);
    }

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
            Rust:\n\
            safe, fast, productive.\n\
            Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
