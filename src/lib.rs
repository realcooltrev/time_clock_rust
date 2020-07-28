pub mod user;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Cli {
    /// Username
    #[structopt(short, long, env = "TC_USER")]
    pub username: String,

    /// Password
    #[structopt(short, long, env = "TC_PASS")]
    pub password: String,

    /// Review department timesheets [`true` or `false`]
    #[structopt(short, long, parse(try_from_str))]
    pub review: bool,
}
