use anyhow::Result;
use rweb::*;
use sqlx::PgPool;
use std::env;

mod tags;
mod users;
mod wanted;
mod warp_rejection;

#[tokio::main]
async fn main() -> Result<()> {
    if env::var_os("RUST_LOG").is_none() {
        // Set `RUST_LOG=web=debug` to see debug logs,
        // this only shows access logs.
        env::set_var("RUST_LOG", "web=info,warn");
    }
    pretty_env_logger::init();

    let pool = PgPool::new(&env::var("DATABASE_URL").expect(
        "DATABASE_URL to be set to something like postgres://postgres:password@localhost/test",
    ))
    .await?;
    serve(
        wanted::get_wanteds(pool.clone())
            .or(wanted::get_wanted(pool.clone()))
            .or(wanted::post_wanted(pool.clone()))
            .or(users::get_user(pool.clone()))
            .or(users::get_users(pool.clone()))
            .or(users::post_user(pool.clone()))
            .or(tags::get_tag(pool.clone()))
            .or(tags::get_tags(pool.clone()))
            .or(tags::post_tag(pool.clone()))
            .or(tags::get_user_tags(pool.clone()))
            .or(tags::post_user_tag(pool))
            .or(get().and(warp::fs::dir("static/")))
            .with(rweb::log("web"))
            .recover(warp_rejection::handle_rejection),
    )
    .run(([127, 0, 0, 1], 3030))
    .await;
    Ok(())
}
