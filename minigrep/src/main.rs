use std::{env, process};

use minigrep;
use minigrep::Config;

fn main() {

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        // eprintln! macro prints to the standard error stream
        eprintln!("Problem parsing arugments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application Error: {}", e);
        process::exit(1);
    }

}

