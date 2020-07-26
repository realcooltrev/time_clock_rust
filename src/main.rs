use std::process;
use structopt::StructOpt;

use time_clock::{user::User, Cli};

fn main() {
    let args = Cli::from_args();

    if let Err(e) = run(args) {
        eprintln!("{}", e);
        process::exit(1);
    }
}

fn run(args: Cli) -> Result<(), &'static str> {
    let user = User::login(args)?;

    println!("Clocked in with role: {:?}!", user.get_role());
    println!("Have a great day, {:?}", user.get_username());

    Ok(())
}
