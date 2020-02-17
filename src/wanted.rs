use crate::warp_rejection::AnyhowError;
use anyhow::{Context, Result};
use chrono::prelude::*;
use futures::stream::TryStreamExt;
use rweb::*;
use serde::{Deserialize, Serialize};
use sqlx::{query_as, PgPool};

type WebResult<T> = Result<Json<T>, Rejection>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Wanted {
    id: i32,
    title: String,
    description: String,
    short_description: String,
    created: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetWantedsQuery {}

#[get("/wanteds")]
pub async fn get_wanteds(
    #[data] pool: PgPool,
    _query: Query<GetWantedsQuery>,
) -> WebResult<Vec<Wanted>> {
    let mut pool = pool;
    let wanteds: Vec<Wanted> = query_as!(Wanted, "SELECT * FROM wanteds")
        .fetch(&mut pool)
        .try_collect()
        .await
        .context("Getting wanteds")
        .map_err(AnyhowError)
        .map_err(reject::custom)?;
    Ok(wanteds.into())
}

#[get("/wanteds/{id}")]
pub async fn get_wanted(
    #[data] pool: PgPool,
    id: i32,
    _query: Query<GetWantedsQuery>,
) -> WebResult<Wanted> {
    let mut pool = pool;
    let wanted: Wanted = query_as!(Wanted, "SELECT * FROM wanteds WHERE id = $1", id)
        .fetch_one(&mut pool)
        .await
        .ok()
        .with_context(|| format!("Couldn't find {}", id))
        .map_err(AnyhowError)
        .map_err(reject::custom)?;
    Ok(wanted.into())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateWanted {
    title: String,
    description: String,
    short_description: String,
}

#[post("/wanteds")]
pub async fn post_wanted(#[data] pool: PgPool, #[json] wanted: CreateWanted) -> WebResult<Wanted> {
    let mut pool = pool;
    let wanted = query_as!(Wanted,
        "INSERT INTO wanteds (title, description, short_description) VALUES ($1, $2, $3) RETURNING id, title, description, short_description, created",
        wanted.title,
        wanted.description,
        wanted.short_description
    )
    .fetch_one(&mut pool)
    .await
    .with_context(|| format!("Inserting wanted {:#?}", wanted))
    .map_err(AnyhowError)
    .map_err(reject::custom)?;
    Ok(wanted.into())
}
