use std::fs;
use std::error::Error;
use std::env;
pub fn run(config: Config) -> Result<(), Box<dyn Error>>{  
    let contents = fs::read_to_string(config.file_path)?;
    println!("With text:\n{contents}");
    Ok(())
}
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}
impl Config{
    pub fn build(
        mut args: impl Iterator<Item = String> + ExactSizeIterator,
    ) -> Result<Config, &'static str> {
        if args.len() > 3 {
            return Err("Not enough arguments")
        }
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didnt get query"),
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("didnt get file path"),
        };
        let ignore_case = env::var("Ignore").is_ok();
        Ok(Config {
            query, 
            file_path,
            ignore_case,
        })
    }
    pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
        contents
            .lines()
            .filter(|line| line.contains(query))
            .collect() 
    }
}
