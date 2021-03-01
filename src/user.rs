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
    pub fn login(username: &str, password: &str) -> Result<User, &'static str> {
        const SECRET_KEY: &str = "admin"; // Just a placeholder until further functionality is setup
        const MANAGER_USERNAME: &str = "trev";
        let user_role;

        if password != SECRET_KEY {
            return Err("Invalid password");
        }

        if username == MANAGER_USERNAME {
            user_role = Role::Manager;
        } else {
            user_role = Role::Employee;
        }

        Ok(User::new(String::from(username), user_role))
    }
}

#[cfg(test)]
mod tests {
    use crate::user::*;

    #[test]
    fn successful_login() {
        let user = User::login("test_user", "admin");
        assert!(user.is_ok());
    }

    #[test]
    fn bad_login() {
        let user = User::login("test_user", "bad_pass");
        assert!(user.is_err());
    }

    #[test]
    fn user_is_manager() {
        let user = User::login("trev", "admin").unwrap();
        assert!(matches!(user.get_role(), Role::Manager));
    }

    #[test]
    fn user_is_employee() {
        let user = User::login("test_user", "admin").unwrap();
        assert!(matches!(user.get_role(), Role::Employee));
    }
}
