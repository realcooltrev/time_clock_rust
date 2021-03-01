use std::process;
use time_clock::user::User;

use clap::{clap_app, ArgMatches};

fn main() -> std::io::Result<()> {
    // Setup CLI interface
    let matches = clap_app!(app =>
        (version: "0.1.0")
        (author: "Trevor Pierce <yeahtrevorpierce@gmail.com>")
        (about: "A CLI solution for clocking in/out that no one asked for")
        (@arg USERNAME: -u --user +takes_value "Sets the username for the user clocking in/out")
        (@arg PASSWORD: -p --password +takes_value "Sets the password for the user clocking in/out")
    )
    .get_matches();

    if let Err(e) = run(matches) {
        eprintln!("{}", e);
        process::exit(1);
    }

    Ok(())
}

fn run(matches: ArgMatches) -> Result<(), &'static str> {
    let username = matches.value_of("USERNAME").unwrap();
    let password = matches.value_of("PASSWORD").unwrap();

    /*
     * We should probably check for environment vars to get the database connection.
     * If not set, prompt the user to provide the connection details, then write the
     * values to a .env file and set the environment variables then at runtime.
     * For future runs, the .env file will be sourced for every rune.
     */
    let user = User::login(username, password)?;

    println!("Clocked in with role: {:?}!", user.get_role());
    println!("Have a great day, {:?}", user.get_username());

    Ok(())
}
