pub struct Config {
    pub username: String,
    pub password: String,
    pub department_review: bool,
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

        let department_review = match args.next() {
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

pub fn run(config: Config) -> Result<(), &'static str> {
    let user = login(config)?;

    println!("Clocked in with role: {:?}!", &user.role);
    println!("Have a great day, {}", &user.username);

    Ok(())
}

fn login(config: Config) -> Result<User, &'static str> {
    const SECRET_KEY: &str = "admin"; // Just a placeholder until further functionality is setup
    let username = config.username;
    let role = if config.department_review {
        Role::Manager
    } else {
        Role::Employee
    };
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
