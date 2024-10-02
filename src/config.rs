use std::fs;
use std::env;
use crate::Result;

pub fn run(config: Config) -> Result<()> {
    let contents = fs::read_to_string(&config.get_file_path())?;
    let results = if config.get_ignore_case() {
        search_case_insensitive(&config.get_query(), &contents)
    } else {
        search(&config.get_query(), &contents)
    };
    for line in results {
        println!("{line}");
    }
    Ok(())
}

pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config> {
        if args.len() != 3 {
            return Err("Not enough arguments".into());
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config {query, file_path, ignore_case})
        
    }
    pub fn get_query(&self) -> &str {
        &self.query
    }
    
    pub fn get_file_path(&self) -> &str {
        &self.file_path
    }

    pub fn get_ignore_case(&self) -> bool {
        self.ignore_case
    }
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

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_sensitive() {
        let query = "duct";
        let contents = "\
rust:
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

        assert_eq!(vec!["Rust:", "Trust me."], 
        search_case_insensitive(query, contents)
        );
    }
}