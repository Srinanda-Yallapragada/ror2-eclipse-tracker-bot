// use crate::survivor_enum::Survivors;
use crate::survivor_enum::NUM_SURVIVORS;
use std::usize;

use sqlx::PgPool;

pub(crate) async fn add(
    pool: &PgPool,
    user_name: &str,
    lvls: &[i32],
) -> Result<String, sqlx::Error> {
    sqlx::query("INSERT INTO eclipse_lvls (user_name, lvls) VALUES ($1, $2)")
        .bind(user_name)
        .bind(lvls)
        .execute(pool)
        .await?;

    Ok(format!("Added lvls for user '{}'", user_name))
}

pub(crate) async fn update(
    pool: &PgPool,
    user_name: &str,
    new_lvls: &[i32],
) -> Result<String, sqlx::Error> {
    sqlx::query("UPDATE eclipse_lvls SET lvls = $2 WHERE user_name = $1")
        .bind(user_name)
        .bind(new_lvls)
        .execute(pool)
        .await?;

    Ok(format!("Updated lvls for user '{}'", user_name))
}

pub(crate) async fn check_user_name_exists(
    pool: &PgPool,
    user_name: &str,
) -> Result<bool, sqlx::Error> {
    let exists: (bool,) =
        sqlx::query_as("SELECT EXISTS (SELECT 1 FROM eclipse_lvls WHERE user_name = $1) AS exists")
            .bind(user_name)
            .fetch_one(pool)
            .await?;

    Ok(exists.0)
}

pub(crate) async fn fetch_all_lvls(
    pool: &PgPool,
    user_name: &str,
) -> Result<[i32; NUM_SURVIVORS as usize], sqlx::Error> {
    let entry: ([i32; NUM_SURVIVORS as usize],) =
        sqlx::query_as("SELECT lvls FROM eclipse_lvls WHERE user_name=$1")
            .bind(user_name)
            .fetch_one(pool)
            .await?;
    Ok(entry.0)
}
