use crate::utils::{get_all_account, user_create_account, verify_login};
use axum::{
    extract,
    extract::Extension,
    http::StatusCode,
    response,
    routing::{get, post},
    Router,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::PgPool;
use std::ops::Deref;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("DB error: `{0}`")]
    Database(#[from] sqlx::Error),
}

pub fn routes() -> Router {
    let router = Router::new()
        .route("/create_account", post(signup))
        .route("/login", post(login))
        .route("/get_accounts", get(get_accounts));

    router
}

#[derive(Deserialize, Serialize, Debug)]
struct UserData {
    username: String,
    password: String,
}

// FIXME: empty input
async fn signup(
    req: extract::Json<UserData>,
    Extension(pg_pool): Extension<PgPool>,
) -> Result<response::Json<Value>, (StatusCode, response::Json<Value>)> {
    let UserData { username, password } = req.deref();
    let hashed_password = &hash(password, DEFAULT_COST).unwrap();
    log::warn!("create account {:?} {:?}", username, hashed_password);
    let id = user_create_account(&pg_pool, username, hashed_password)
        .await
        .map_err(|_| {
            (
                StatusCode::UNAUTHORIZED,
                response::Json(json!({"error": "Another user with this username already exists."})),
            )
        })?;

    Ok(response::Json(json!({ "message": id })))
}

async fn login(
    req: extract::Json<UserData>,
    Extension(pg_pool): Extension<PgPool>,
) -> Result<response::Json<Value>, (StatusCode, response::Json<Value>)> {
    log::warn!("create account {:?}", req);
    // `POST /` called post api for logging in
    let UserData { username, password } = req.deref();

    let user_password = verify_login(&pg_pool, username).await.map_err(|_| {
        (
            StatusCode::UNAUTHORIZED,
            response::Json(json!({"error": "Invalid username or password."})),
            // response::Json(json!({"error": err.to_string()})),
        )
    })?;

    let valid = verify(password, &user_password).unwrap();
    if !valid {
        return Err((
            StatusCode::UNAUTHORIZED,
            response::Json(json!({"error": "Invalid username or password."})),
        ));
    };

    Ok(response::Json(json!({
        "message": format!("Welcome back {} !", username)
    })))
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
