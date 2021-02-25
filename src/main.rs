use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::process;
use structopt::StructOpt;
use time_clock::{user::User, Cli};

#[derive(Serialize, Deserialize, Debug)]
struct Configs {
    connection_string: String,
}

fn main() -> std::io::Result<()> {
    // Need to use serde to read configs for database from config.json file

    let filed_configs = fs::read_to_string("config.json").expect("Could not load config.json file");
    let configs = serde_json::to_string(&filed_configs).unwrap();

    println!("{}", configs);

    Ok(())

    /*let args = Cli::from_args();

    if let Err(e) = run(args) {
        eprintln!("{}", e);
        process::exit(1);
    }*/
}

fn run(args: Cli) -> Result<(), &'static str> {
    let user = User::login(args)?;

    println!("Clocked in with role: {:?}!", user.get_role());
    println!("Have a great day, {:?}", user.get_username());

    Ok(())
}
