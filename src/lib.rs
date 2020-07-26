mod user;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Cli {
    /// Username
    #[structopt(short, long, env = "TC_USER")]
    pub username: String,

    /// Password
    #[structopt(short, long, env = "TC_PASS")]
    pub password: String,

    /// Review department timesheets
    #[structopt(short, long, parse(try_from_str))]
    pub review: bool,
}

pub fn run(args: Cli) -> Result<(), &'static str> {
    let user = user::User::login(args)?;

    println!("Clocked in with role: {:?}!", &user.get_role());
    println!("Have a great day, {:?}", &user.get_username());

    Ok(())
}
