use std::env::args;
use std::process;

use minigrep::Config;
fn main(){
    
    let config = Config::build(args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing {err}");
        process::exit(1);
    });
    println!("searching {}", config.query);
    println!("In file {}", config.file_path);
    if let Err(e) = minigrep::run(config){
        eprintln!("Application Err: {e}");
        process::exit(1);
    }

}
