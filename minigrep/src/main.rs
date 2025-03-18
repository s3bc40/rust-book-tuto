use std::{env, process};

// Code from lib.rs with crate name
use minigrep::Config;

fn main() {
    // Reading args
    let args: Vec<String> = env::args().collect();
    // dbg!(args);

    // Saving args
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // Handling errors returned by run (succes is empty)
    // Access run from lib crate name
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
