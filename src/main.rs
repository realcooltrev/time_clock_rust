use std::io;

#[derive(Debug)]
enum Role {
    Employee,
    Manager,
}

#[derive(Debug)]
struct User {
    username: String,
    role: Role,
}

fn main() {
    let authenticated_user = login();
    println!("username: {}", &authenticated_user.username);
    println!("role: {:#?}", &authenticated_user.role);
    
    match &authenticated_user.role {
        Role::Employee => println!("Hello"),
        Role::Manager => println!("Ah, royalty"),
    }
}

fn login() -> User {
    display_header();
    const SECRET_KEY: &str = "admin"; // Just a placeholder until further functionality is setup

    loop {
        let mut username = String::new();
        let mut password = String::new();

        println!("username: ");

        io::stdin()
            .read_line(&mut username)
            .expect("Failed to read username");
        
        println!("password: ");

        io::stdin()
            .read_line(&mut password)
            .expect("Failed to read password");
        
        // Trim off /n's
        let username = username.trim();
        let password = password.trim();


        // This is definitely how authentication works
        if &password == &SECRET_KEY {
            println!("Logged in!");
            return User {
                username: String::from(username),
                role: Role::Employee,
            }
        } else {
            println!("Incorrect password. Please try again");
        }
    }
}

fn display_header() {
    println!("COMPANY NAME // DATE: TIME");
} 