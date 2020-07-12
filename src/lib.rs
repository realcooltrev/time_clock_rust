pub struct Config {
    pub username: String,
    pub password: String,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        // Skip the binary name
        args.next();

        let username = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a username"),
        };

        let password = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a password"),
        };

        Ok(Config { username, password })
    }
}

#[derive(Debug)]
pub enum Role {
    Employee,
    Manager,
}

#[derive(Debug)]
pub struct User {
    pub username: String,
    pub authenticated: bool,
    pub role: Role,
}

impl User {
    pub fn login(config: Config) -> Result<User, &'static str> {
        const SECRET_KEY: &str = "admin"; // Just a placeholder until further functionality is setup
        let username = config.username;
        let role = Role::Employee;
        let authenticated = config.password == SECRET_KEY;

        if !authenticated {
            return Err("Invalid password");
        }
        Ok(User {
            username,
            authenticated,
            role,
        })
    }
}
