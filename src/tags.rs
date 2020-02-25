use crate::warp_rejection::AnyhowError;
use anyhow::{Context, Result};
use chrono::prelude::*;
use futures::stream::TryStreamExt;
use rweb::*;
use serde::{Deserialize, Serialize};
use sqlx::{query_as, PgPool};

type WebResult<T> = Result<Json<T>, Rejection>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {
    id: i32,
    state: String,
    user_id: Option<i32>,
    wanted: i32,
    created: DateTime<Utc>,
}

#[get("/tags")]
pub async fn get_tags(#[data] pool: PgPool) -> WebResult<Vec<Tag>> {
    let mut pool = pool;
    let tags: Vec<Tag> = query_as!(Tag, "SELECT * FROM tags where user_id = NULL")
        .fetch(&mut pool)
        .try_collect()
        .await
        .context("Getting tags")
        .map_err(AnyhowError)
        .map_err(reject::custom)?;
    Ok(tags.into())
}

#[get("/tags/{id}")]
pub async fn get_tag(#[data] pool: PgPool, id: i32) -> WebResult<Tag> {
    let mut pool = pool;
    let tag: Tag = query_as!(Tag, "SELECT * FROM tags WHERE id = $1", id)
        .fetch_one(&mut pool)
        .await
        .ok()
        .with_context(|| format!("Couldn't find {}", id))
        .map_err(AnyhowError)
        .map_err(reject::custom)?;
    Ok(tag.into())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTag {
    state: String,
    wanted: i32,
}

#[post("/tags")]
pub async fn post_tag(#[data] pool: PgPool, #[json] tag: CreateTag) -> WebResult<Tag> {
    let mut pool = pool;
    let tag = query_as!(
        Tag,
        "INSERT INTO tags (state, wanted) VALUES ($1, $2) RETURNING id, state, wanted, user_id, created",
        tag.state,
        tag.wanted
    )
    .fetch_one(&mut pool)
    .await
    .with_context(|| format!("Inserting tag {:#?}", tag))
    .map_err(AnyhowError)
    .map_err(reject::custom)?;
    Ok(tag.into())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserTag {
    state: String,
    wanted: i32,
}

#[post("/users/{user_id}/tags")]
pub async fn post_user_tag(
    #[data] pool: PgPool,
    user_id: i32,
    #[json] tag: CreateTag,
) -> WebResult<Tag> {
    let mut pool = pool;
    let tag = query_as!(
        Tag,
        "INSERT INTO tags (state, wanted, user_id) VALUES ($1, $2, $3) RETURNING id, state, wanted, user_id, created",
        tag.state,
        tag.wanted,
        user_id
    )
    .fetch_one(&mut pool)
    .await
    .with_context(|| format!("Inserting tag {:#?}", tag))
    .map_err(AnyhowError)
    .map_err(reject::custom)?;
    Ok(tag.into())
}

#[get("/users/{user_id}/tags")]
pub async fn get_user_tags(#[data] pool: PgPool, user_id: i32) -> WebResult<Vec<Tag>> {
    let mut pool = pool;
    let tags: Vec<Tag> = query_as!(Tag, "SELECT * FROM tags WHERE user_id = $1", user_id)
        .fetch(&mut pool)
        .try_collect()
        .await
        .with_context(|| format!("Getting tags from user {}", user_id))
        .map_err(AnyhowError)
        .map_err(reject::custom)?;
    Ok(tags.into())
}
