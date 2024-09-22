use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(config.file_path)?;
    println!("file content is:\n {}", file_content);
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