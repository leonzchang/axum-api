mod api;
mod app;

use crate::app::service;
#[tokio::main]
async fn main() {
    service().await
}
