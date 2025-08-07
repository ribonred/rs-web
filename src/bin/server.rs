use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware::Logger, web};
use apistos::app::{BuildConfig, OpenApiWrapper};
use apistos::info::Info;
use apistos::spec::Spec;
use apistos::{RedocConfig, ScalarConfig};
use log::info;
use std::io;

use service::config::Settings;
use service::{db, routes, state};

#[tokio::main]
async fn main() -> io::Result<()> {
    // Initialize logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    info!("Starting r-web server...");

    // Load configuration
    let settings = Settings::new().expect("Failed to read configuration");
    let bind_address = settings.get_bind_address();
    let base_url = format!("http://{}", bind_address);
    let api_ver = settings.application.api_version.clone();

    info!("Environment: {}", settings.application.environment);
    info!("Connecting to database...");

    // Initialize database
    let db = db::init_db(&settings)
        .await
        .expect("Failed to connect to database");

    // Create application state
    let app_state = state::AppState::new(db, settings);

    info!("Starting HTTP server at http://{}", bind_address);

    // Start HTTP server
    HttpServer::new(move || {
        let spec = Spec {
            info: Info {
                title: "R-Web API Service".to_string(),
                version: api_ver.clone(),
                description: Some("R-Web REST API documentation".to_string()),
                ..Default::default()
            },
            servers: vec![apistos::server::Server {
                url: base_url.clone(),
                description: Some("Local development server".to_string()),
                ..Default::default()
            }],
            ..Default::default()
        };
        App::new()
            .document(spec)
            .app_data(web::Data::new(app_state.clone()))
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            .configure(routes::configure)
            .build_with(
                "/openapi.json",
                BuildConfig::default()
                    .with(RedocConfig::new(&"/redoc"))
                    .with(ScalarConfig::new(&"/scalar")),
            )
    })
    .bind(&bind_address)?
    .run()
    .await
}
