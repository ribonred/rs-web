use crate::config::Settings;
use log::info;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::time::Duration;

pub async fn init_db(config: &Settings) -> Result<DatabaseConnection, sea_orm::DbErr> {
    let db_url = config.database.get_url();

    let mut opt = ConnectOptions::new(db_url);
    opt.max_connections(config.database.max_connections)
        .min_connections(config.database.min_connections)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Debug);

    let db = Database::connect(opt).await?;
    info!("Database connected successfully");
    let _ = db.ping().await?;
    info!("Database ping successful");

    Ok(db)
}
