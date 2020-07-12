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

#[derive(Debug)]
enum Role {
    Employee,
    Manager,
}

#[derive(Debug)]
struct User {
    username: String,
    authenticated: bool,
    role: Role,
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
