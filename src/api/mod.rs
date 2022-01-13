use std::ops::Deref;

use crate::utils::{get_all_account, user_create_account};
use axum::{
    extract,
    extract::Extension,
    http::StatusCode,
    response,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::PgPool;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("DB error: `{0}`")]
    Database(#[from] sqlx::Error),
}

pub fn routes() -> Router {
    let router = Router::new()
        .route("/create_account", post(create_account))
        .route("/login", post(login))
        .route("/get_accounts", get(get_accounts));

    router
}

#[derive(Deserialize, Serialize, Debug)]
struct UserData {
    username: String,
    password: String,
}

// TODO: authentication
async fn create_account(
    req: extract::Json<UserData>,
    Extension(pg_pool): Extension<PgPool>,
) -> Result<response::Json<Value>, (StatusCode, String)> {
    let UserData { username, password } = req.deref();
    let id = user_create_account(&pg_pool, username, password)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    Ok(response::Json(json!({ "account_id": id })))
}

// TODO: authentication, database function
async fn login(payload: extract::Json<UserData>) {
    log::warn!("create account {:?}", payload);
    // `POST /` called post api for logging in
}

// For test
async fn get_accounts(
    Extension(pg_pool): Extension<PgPool>,
) -> Result<response::Json<Value>, (StatusCode, String)> {
    #[derive(Serialize, Debug)]
    struct SerializeUsersData {
        id: i32,
        username: String,
        password: String,
    }

    let result = get_all_account(&pg_pool)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?
        .into_iter()
        .map(|(id, username, password)| SerializeUsersData {
            id,
            username,
            password,
        })
        .collect::<Vec<SerializeUsersData>>();
    log::warn!("{:?}", result);
    Ok(response::Json(json!(result)))
}
