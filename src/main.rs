use std::env;
use std::process;

mod songs;
mod config;

fn main() {
    let config = config::Config::new(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = config::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
