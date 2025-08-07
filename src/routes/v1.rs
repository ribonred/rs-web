use crate::handlers;
use apistos::web::{ServiceConfig, get, scope};

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(scope("/api/v1").route("/health", get().to(handlers::health::health_check)));
}
