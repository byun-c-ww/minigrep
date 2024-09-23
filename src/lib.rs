use std::error::Error;
use std::{fs, vec};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(config.file_path)?;
    for line in search(&config.search_query, &file_content) {
        println!("{}",line);
    }
    Ok(())
}

pub struct Config {
    pub search_query: String,
    pub file_path: String,
}
impl Config {
    pub fn build(args: &[String]) -> Result<Config,&'static str> {
        if args.len() < 3 {
            return Err("not enough arguments, must provide 3");
        }
        let search_query = args[1].clone();
        let file_path = args[2].clone();
    
        Ok(Config { search_query, file_path })
    }   
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut result_string = Vec::new();
  for line in contents.lines() {
    if line.contains(query) {
      result_string.push(line);
    }
  }
  result_string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}