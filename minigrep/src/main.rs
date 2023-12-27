use std::{env, process};

use minigrep::Config;

fn main() {
    println!("\tChapter 11...! Mini grep project...\n");

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem passing argument: {}", err);
        process::exit(1);
    });
    println!("Searching for [{}], in file [{}]", config.query, config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
