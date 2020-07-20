use std::{env, process};

use time_clock::config::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = time_clock::run(config) {
        eprintln!("{}", e);
        process::exit(1);
    }
}
