#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::debug_handler;
use loco_rs::prelude::*;

use crate::views;

#[debug_handler]
pub async fn render_home(
    ViewEngine(v): ViewEngine<TeraView>,
    Path(paths): Path<String>,
) -> Result<impl IntoResponse> {
    // if headers.get("Hx-Boosted").is_some() {}
    views::app(v, paths.split("/").map(|p| p.to_owned()).collect())
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("app/")
        .add("/{*paths}", get(render_home))
}
