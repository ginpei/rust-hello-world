use std::env;
use std::process;

use rust_minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    // If missed `use std::process;`, error:
    // > mismatched types
    // > expected struct `Config`, found `()`rustc (E0308)
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // Q. how to get Ok result?
    if let Err(e) = rust_minigrep::run(config) {
        eprintln!("Application error: {}", e);
        return;
    }
}
