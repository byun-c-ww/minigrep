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
    fn build(args: &[String]) -> Result<Config,&'static str> {
        if args.len() < 3 {
            return Err("not enough arguments, must provide 3");
        }
        let search_query = args[1].clone();
        let file_path = args[2].clone();
    
        Ok(Config { search_query, file_path })
    }   
}

fn main() {
    let command_line_arguments: Vec<String> = env::args().collect();
    let config = Config::build(&command_line_arguments);
    // what is a rust way of ensuring rust dont panic if no argument was provided?
    // use result type for config?
    
    let file_content = read_file_content(&config.file_path).expect("failed to read file content!");
    println!("search query is {}",config.search_query);
    println!("file path is {}",config.file_path);
    println!("{file_content}");
}
