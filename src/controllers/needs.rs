#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::debug_handler;
use loco_rs::prelude::*;

#[debug_handler]
pub async fn index(State(_ctx): State<AppContext>) -> Result<Response> {
    format::text("hello")
}

pub async fn list(State(ctx): State<AppContext>) -> Result<Response> {
    let res = crate::models::needs::Entity::find().all(&ctx.db).await?;
    format::json(res)
}

pub fn routes() -> Routes {
    Routes::new().prefix("api/needs/").add("/", get(list))
}
