use axum::{
    extract,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Deserialize, Serialize, Debug)]
struct UserData {
    username: String,
    password: String,
}

pub fn routes() -> Router {
    let router = Router::new()
        .route("/create_account", post(create_account))
        .route("/login", post(login))
        .route("/get_account", get(get_account_test));
    router
}

async fn create_account(payload: extract::Json<UserData>) {
    log::info!("create account {:?}", payload);
    // `POST /` called post api for creating account
}

async fn login(payload: extract::Json<UserData>) {
    log::warn!("create account {:?}", payload);
    // `POST /` called post api for logging in
}

async fn get_account_test() -> Json<Value> {
    // `GET /account for test
    let user_test = UserData {
        username: "John Doe".to_owned(),
        password: "test".to_owned(),
    };

    log::error!("create account {:?}", user_test);
    Json(json!(user_test))
}
