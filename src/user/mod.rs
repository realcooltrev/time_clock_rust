use crate::config;

#[derive(Debug)]
pub enum Role {
    Employee,
    Manager,
}

#[derive(Debug)]
pub struct User {
    username: String,
    role: Role,
}

impl User {
    fn new(username: String, role: Role) -> User {
        return User { username, role };
    }

    pub fn get_username(&self) -> &str {
        &self.username
    }

    pub fn get_role(&self) -> &Role {
        &self.role
    }

    // Check the user's login credentials and return an authenticated user
    pub fn login(config: config::Config) -> Result<User, &'static str> {
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

        Ok(User::new(username, role))
    }
}
