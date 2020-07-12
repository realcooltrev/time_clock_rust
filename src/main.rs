use std::{env, process};

use time_clock::Config;
use time_clock::User;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = User::login(config) {
        eprintln!("Authentication error: {}", e);
        process::exit(1);
    } else {
        println!("Clocked in!");
    }
}
