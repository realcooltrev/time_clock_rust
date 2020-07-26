use std::process;
use structopt::StructOpt;

use time_clock::Cli;

fn main() {
    let args = Cli::from_args();

    if let Err(e) = time_clock::run(args) {
        eprintln!("{}", e);
        process::exit(1);
    }
}
