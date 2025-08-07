use entity::auth_users::Entity as AuthUsers;
use entity::auth_users_ext::AuthUserEntityExt;
use log::info;
use std::io;

use service::config::Settings;
use service::db;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    info!("Creating superuser...");

    let settings = Settings::new().expect("Failed to read configuration");

    let db_conn = db::init_db(&settings)
        .await
        .expect("Failed to connect to database");

    // Get user input
    let email = get_user_input("Enter superuser email: ")?;
    let username = get_user_input("Enter superuser username: ")?;
    let password = get_user_input("Enter superuser password: ")?;

    // Create superuser
    match AuthUsers::create_superuser(&db_conn, email.clone(), username.clone(), password).await {
        Ok(user) => {
            info!("Superuser created successfully!");
            info!("Email: {}", user.email);
            info!("Username: {}", user.username);
            info!("ID: {}", user.id);
        }
        Err(e) => {
            eprintln!("Failed to create superuser: {:?}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}

fn get_user_input(prompt: &str) -> io::Result<String> {
    use std::io::{self, Write};

    print!("{}", prompt);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input.trim().to_string())
}
