pub struct Config {
    pub username: String,
    pub password: String,
    pub department_review: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        // Skip the binary name
        args.next();

        // First argument should be username
        let username = match std::env::args().nth(1) {
            Some(arg) => arg,
            None => return Err("Didn't get a username"),
        };

        // Sencond argument should be password
        let password = match std::env::args().nth(2) {
            Some(arg) => arg,
            None => return Err("Didn't get a password"),
        };

        // Optional third argurment for managers
        let department_review = match std::env::args().nth(3) {
            Some(_) => true,
            None => false,
        };

        Ok(Config {
            username,
            password,
            department_review,
        })
    }
}
