//! # Testing
//! Where is this section

use std::fs;
use std::error::Error;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }
    
    Ok(())
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();
        /*
        if args.len() < 3 {
            return Err("Usage: blah");
        }
        */

        // let query = args[1].clone();
        let query = match args.next() {
            Some(v) => v,
            None => return Err("Missing query string"),
        };

        let filename = match args.next() {
            Some(v) => v,
            None => return Err("Missing filename"),
        };

        // let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    /*
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
    */

    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

/// Like a grep
///
/// # Examples
/// ```
///use day8::search;
///let query = "duct";
///let contents = "\
///Rust:
///safe, fast, productive.
///Pick three.
///Duct tape.";
///
///        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
/// ```
pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    /*
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    
    results
    */
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}
