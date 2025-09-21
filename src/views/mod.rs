pub mod auth;
pub mod feeling;
pub mod need;

use loco_rs::prelude::*;
use tracing::info;

pub fn app(v: impl ViewRenderer, paths: Vec<String>) -> Result<impl IntoResponse> {
    let default = "".to_string();
    let path1 = paths.get(0).unwrap_or(&default);

    let history_path = if let Some(path2) = paths.get(1) {
        path1.to_owned() + "/" + path2
    } else {
        path1.to_owned()
    };

    let component_path = if let Some(path2) = paths.get(1) {
        path1.to_owned() + "/components/" + path2
    } else {
        "app_components/".to_owned() + path1
    };

    let history_path = format!("app/{}", history_path);

    info!("paths: {:?}", paths);
    info!("Component path: {}", component_path);
    info!("History path: {}", history_path);

    format::render().view(
        &v,
        "home/hello.html",
        data!({ "history_path" : history_path, "component_path" : component_path }),
    )
}

pub fn login(v: &impl ViewRenderer) -> Result<Response> {
    format::render().view(v, "home/login.html", data!({}))
}
