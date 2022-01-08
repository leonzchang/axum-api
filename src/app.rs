use crate::api;
use axum::{response::Json, routing::get, Router};
use serde_json::{json, Value};

pub async fn service() {
    // build our application with a single route
    let app = Router::new()
        .route("/healthz", get(service_info))
        .nest("/api", api::routes());

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
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
