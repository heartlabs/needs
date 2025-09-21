use axum::debug_handler;
use loco_rs::prelude::*;

use crate::views;

#[debug_handler]
pub async fn login(ViewEngine(v): ViewEngine<TeraView>) -> Result<Response> {
    views::login(&v)
}
pub fn routes() -> Routes {
    Routes::new()
        .prefix("app_components/")
        .add("/login", get(login))
}
