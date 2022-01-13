// To Do database connection
use sqlx::{postgres::PgPoolOptions, postgres::PgRow, PgPool, Row};

pub async fn init_connections(database_url: &str) -> PgPool {
    let pg_pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("postgres database unavailable");

    pg_pool
}

pub async fn get_all_account(
    pg_pool: &PgPool,
) -> Result<Vec<(i32, std::string::String, std::string::String)>, sqlx::Error> {
    sqlx::query(
        r#"
            SELECT * FROM account ORDER BY id ASC
        "#,
    )
    .try_map(|row: PgRow| {
        Ok((
            row.try_get::<i32, _>(0)?,
            row.try_get::<String, _>(1)?,
            row.try_get::<String, _>(2)?,
        ))
    })
    .fetch_all(pg_pool)
    .await
}

pub async fn user_create_account(
    pg_pool: &PgPool,
    username: &str,
    password: &str,
) -> Result<i32, sqlx::Error> {
    sqlx::query(
        r#"
            INSERT INTO account (username, password) VALUES ($1, $2) RETURNING id
        "#,
    )
    .bind(username)
    .bind(password)
    .map(|row: PgRow| Ok(row.get::<i32, _>(0)))
    .fetch_one(pg_pool)
    .await?
}
