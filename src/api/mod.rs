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
    account: String,
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
    // `POST /` called post api for creating account
    println!(
        "account: {:?}, password: {:?}",
        payload.account, payload.password
    )
}

async fn login(payload: extract::Json<UserData>) {
    // `POST /` called post api for logging in
    println!(
        "account: {:?}, password: {:?}",
        payload.account, payload.password
    )
}

async fn get_account_test() -> Json<Value> {
    // `GET /account for test
    let user_test = UserData {
        account: "John Doe".to_owned(),
        password: "test".to_owned(),
    };

    Json(json!(user_test))
}
