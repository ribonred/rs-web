use crate::config::Settings;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DatabaseConnection>,
    pub config: Arc<Settings>,
}

impl AppState {
    pub fn new(db: DatabaseConnection, config: Settings) -> Self {
        Self {
            db: Arc::new(db),
            config: Arc::new(config),
        }
    }
}
