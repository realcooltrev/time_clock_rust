use std::io;

fn main() {

    const SECRET_KEY: &str = "admin";

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
        
        let username = username.trim();
        let password = password.trim();

        if password == SECRET_KEY {
            println!("Logged in!");
            break;
        }
    }
}