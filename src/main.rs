use std::env;
use std::fs;
// use std::error::Error;
use std::io;

fn read_file_content(file_path: &str) -> Result<String, io::Error> {
    let file_content = fs::read_to_string(file_path)?;
    Ok(file_content)
}

struct Config {
    search_query: String,
    file_path: String,
}
impl Config {
    fn new(args: &[String]) -> Config {
        let search_query = args[1].clone();
        let file_path = args[2].clone();
    
        Config { search_query, file_path }
    }   
}

fn main() {
    let command_line_arguments: Vec<String> = env::args().collect();
    let config = Config::new(&command_line_arguments);

    let file_content = read_file_content(&config.file_path).expect("failed to read file content!");
    println!("search query is {}",config.search_query);
    println!("file path is {}",config.file_path);
    println!("{file_content}");
}
