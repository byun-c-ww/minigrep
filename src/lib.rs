use std::error::Error;
use std::{fs, vec};
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(config.file_path)?;
    
    let results = if config.ignore_case {
        search_insensitive(&config.search_query, &file_content)
    } else {
        search(&config.search_query, &file_content)
    };
    for line in results {
        println!("{}",line);
    }
    Ok(())
}

pub struct Config {
    pub search_query: String,
    pub file_path: String,
    pub ignore_case: bool,
}
impl Config {
    pub fn build(args: &[String]) -> Result<Config,&'static str> {
        if args.len() < 3 {
            return Err("not enough arguments, must provide 3");
        }
        let search_query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        println!("args is this: {:?}", args);
    
        Ok(Config { search_query, file_path, ignore_case })
    }   
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
  let mut results = Vec::new();
  for line in content.lines() {
    if line.contains(query) {
      results.push(line);
    }
  }
  results
}

pub fn search_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
  let mut results = Vec::new();
  let query = query.to_lowercase();
  for line in content.lines() {
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
    fn case_sensitive() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUSt";
        let content = "\
Rust:
safe, fast, productive.
Pick three
trust me";
       assert_eq!(vec!["Rust:","trust me"], search_insensitive(query, content)); 
    }
}