pub mod v1;

use apistos::web::ServiceConfig;

pub fn configure(cfg: &mut ServiceConfig) {
    // Configure v1 routes
    v1::configure(cfg);

    // Future: Configure v2 routes
    // v2::configure(cfg);
}
