use std::env;
// use std::error::Error;
use std::io;
use std::process;

use minigrep::Config;
fn main() {
    let command_line_arguments: Vec<String> = env::args().collect();
    let config = Config::build(&command_line_arguments).unwrap_or_else(|err| {
      println!("error building config! error: {}", err);
      process::exit(1);
    });
    // what is a rust way of ensuring rust dont panic if no argument was provided?
    // use result type for config?
    println!("search query is {}",config.search_query);
    println!("file path is {}",config.file_path);    
    if let Err(e) = minigrep::run(config) {
        println!("Application Error! {e}");
        process::exit(1);
    }

}
