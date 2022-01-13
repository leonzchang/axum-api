use crate::api;
use crate::utils::init_connections;
use axum::{response::Json, routing::get, Router};
use serde_json::{json, Value};
use tower_http::add_extension::AddExtensionLayer;

pub struct Config {
    pub database_url: String,
}

pub async fn service(config: Config) {
    env_logger::init();

    let Config { database_url } = config;
    let pg_pool = init_connections(&database_url).await;

    // build our application with a single route
    let app = Router::new()
        .route("/healthz", get(service_info))
        .nest("/api", api::routes())
        .layer(AddExtensionLayer::new(pg_pool));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:1234".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn service_info() -> Json<Value> {
    Json(json!({
        "name":    env!("CARGO_PKG_NAME"),
        "version": env!("CARGO_PKG_VERSION"),
        "branch": env!("VERGEN_GIT_BRANCH"),
        "commit": env!("VERGEN_GIT_SHA_SHORT"),
        "rustc_host": env!("VERGEN_RUSTC_HOST_TRIPLE"),
        "rustc_semver": env!("VERGEN_RUSTC_SEMVER"),
        "startTime": chrono::Utc::now().timestamp(),
    }))
}
