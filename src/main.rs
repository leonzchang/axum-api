mod api;
mod app;
mod utils;
use crate::app::service;

#[tokio::main]
async fn main() {
    let config = app::Config {
        database_url: "postgres://postgres:test123@localhost:5432/domain?sslmode=disable"
            .to_string(),
    };

    service(config).await
}
