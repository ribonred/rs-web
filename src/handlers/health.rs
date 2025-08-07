use crate::state::AppState;
use actix_web::{HttpResponse, web};
use apistos::{ApiComponent, api_operation};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub environment: String,
    pub database: String,
}

#[api_operation(
    summary = "Health check endpoint",
    description = "Check if the API and database are healthy",
    tag = "health"
)]
pub async fn health_check(app_state: web::Data<AppState>) -> HttpResponse {
    match app_state.db.ping().await {
        Ok(_) => {
            let response = HealthResponse {
                status: "healthy".to_string(),
                version: app_state.config.application.api_version.clone(),
                environment: app_state.config.application.environment.clone(),
                database: "connected".to_string(),
            };
            HttpResponse::Ok().json(response)
        }
        Err(_) => {
            let response = HealthResponse {
                status: "unhealthy".to_string(),
                version: app_state.config.application.api_version.clone(),
                environment: app_state.config.application.environment.clone(),
                database: "disconnected".to_string(),
            };
            HttpResponse::ServiceUnavailable().json(response)
        }
    }
}
