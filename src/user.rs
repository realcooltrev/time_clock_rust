use super::Cli;

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
    pub fn login(args: Cli) -> Result<User, &'static str> {
        const SECRET_KEY: &str = "admin"; // Just a placeholder until further functionality is setup

        let username = args.username;
        let role = if args.review {
            Role::Manager
        } else {
            Role::Employee
        };
        let authenticated = args.password == SECRET_KEY;

        if !authenticated {
            return Err("Invalid password");
        }

        Ok(User::new(username, role))
    }
}

#[cfg(test)]
mod tests {
    use super::Cli;
    use crate::user::*;

    #[test]
    fn successful_login() {
        let cli = Cli {
            username: String::from("test_user"),
            password: String::from("admin"),
            review: false,
        };
        let user = User::login(cli);
        assert!(user.is_ok());
    }

    #[test]
    fn bad_login() {
        let cli = Cli {
            username: String::from("test_user"),
            password: String::from("bad_pass"),
            review: false,
        };
        let user = User::login(cli);
        assert!(user.is_err());
    }

    #[test]
    fn user_is_manager() {
        let cli = Cli {
            username: String::from("test_user"),
            password: String::from("admin"),
            review: true,
        };
        let user = User::login(cli).unwrap();
        assert!(matches!(user.get_role(), Role::Manager));
    }

    #[test]
    fn user_is_employee() {
        let cli = Cli {
            username: String::from("test_user"),
            password: String::from("admin"),
            review: false,
        };
        let user = User::login(cli).unwrap();
        assert!(matches!(user.get_role(), Role::Employee));
    }
}
