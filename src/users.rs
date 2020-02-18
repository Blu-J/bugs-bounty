use crate::warp_rejection::AnyhowError;
use anyhow::{Context, Result};
use chrono::prelude::*;
use futures::stream::TryStreamExt;
use rweb::*;
use serde::{Deserialize, Serialize};
use sqlx::{query_as, PgPool};

type WebResult<T> = Result<Json<T>, Rejection>;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: i32,
    name: String,
}

#[get("/users")]
pub async fn get_users(#[data] pool: PgPool) -> WebResult<Vec<User>> {
    let mut pool = pool;
    let users: Vec<User> = query_as!(User, "SELECT * FROM users")
        .fetch(&mut pool)
        .try_collect()
        .await
        .context("Getting users")
        .map_err(AnyhowError)
        .map_err(reject::custom)?;
    Ok(users.into())
}

#[get("/users/{id}")]
pub async fn get_user(#[data] pool: PgPool, id: i32) -> WebResult<User> {
    let mut pool = pool;
    let user: User = query_as!(User, "SELECT * FROM users WHERE id = $1", id)
        .fetch_one(&mut pool)
        .await
        .ok()
        .with_context(|| format!("Couldn't find {}", id))
        .map_err(AnyhowError)
        .map_err(reject::custom)?;
    Ok(user.into())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUser {
    name: String,
}

#[post("/users")]
pub async fn post_user(#[data] pool: PgPool, #[json] user: CreateUser) -> WebResult<User> {
    let mut pool = pool;
    let user = query_as!(
        User,
        "INSERT INTO users (name) VALUES ($1) RETURNING id, name",
        user.name
    )
    .fetch_one(&mut pool)
    .await
    .with_context(|| format!("Inserting user {:#?}", user))
    .map_err(AnyhowError)
    .map_err(reject::custom)?;
    Ok(user.into())
}
