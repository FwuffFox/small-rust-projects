use std::error::Error;
use std::fs;
use crate::config::Config;

pub mod config;

#[cfg(test)]
mod tests;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    if config.case_insensitive {
        for line in search_insensitive(&config.query, &contents) {
            println!("{line}")
        }
    } else {
        for line in search(&config.query, &contents) {
            println!("{line}")
        }
    }

    return Ok(());
}

pub fn search<'a>(query: &str, text: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in text.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    return results;
}

pub fn search_insensitive<'a>(query: &str, text: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    let lower_query = query.to_lowercase();

    for line in text.lines() {
        if line.to_lowercase().contains(&lower_query) {
            results.push(line);
        }
    }

    return results;
}