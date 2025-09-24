use std::{fs, path::PathBuf};

use crate::APP_VERSION;
use axum::{
    body::Body,
    debug_handler,
    http::{header, HeaderMap},
};
use loco_rs::prelude::*;
use tokio_stream::StreamExt;
use tokio_util::io::ReaderStream;

pub mod app;
pub mod app_components;
pub mod auth;
pub mod feeling;
pub mod feeling_components;
pub mod invite;
pub mod need_components;

#[debug_handler]
pub async fn service_worker() -> Result<impl IntoResponse> {
    let srcdir = PathBuf::from("./");
    println!("{:?}", fs::canonicalize(&srcdir));

    let file = match tokio::fs::File::open("assets/static/sw.js").await {
        Ok(file) => file,
        Err(_err) => return Err(Error::NotFound),
    };
    let mut headers = HeaderMap::new();
    headers.insert("Service-Worker-Allowed", "/".parse().unwrap());
    headers.insert(
        header::CONTENT_TYPE,
        "text/javascript; charset=utf-8".parse().unwrap(),
    );

    let app_version_stream = ReaderStream::new("const APP_VERSION = '".as_bytes())
        .chain(ReaderStream::new(APP_VERSION.as_bytes()))
        .chain(ReaderStream::new("'\n".as_bytes()));

    let file_stream = ReaderStream::new(file);
    let body = Body::from_stream(app_version_stream.chain(file_stream));

    Ok((headers, body))
}

pub fn routes() -> Routes {
    Routes::new().add("/static/sw.js", get(service_worker))
}
