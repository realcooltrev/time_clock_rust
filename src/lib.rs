pub mod config;
mod user;

pub fn run(config: config::Config) -> Result<(), &'static str> {
    let user = user::User::login(config)?;

    println!("Clocked in with role: {:?}!", &user.get_role());
    println!("Have a great day, {:?}", &user.get_username());

    Ok(())
}
