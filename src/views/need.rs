use loco_rs::prelude::*;

use crate::models::_entities::needs;

/// Render a list view of `needs`.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn list(v: &impl ViewRenderer, items: &Vec<needs::Model>) -> Result<Response> {
    format::render().view(v, "need/list.html", data!({"items": items}))
}

/// Render a single `need` view.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn show(v: &impl ViewRenderer, item: &needs::Model) -> Result<Response> {
    format::render().view(v, "need/show.html", data!({"item": item}))
}

/// Render a `need` create form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn create(v: &impl ViewRenderer) -> Result<Response> {
    format::render().view(v, "need/create.html", data!({}))
}

/// Render a `need` edit form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn edit(v: &impl ViewRenderer, item: &needs::Model) -> Result<Response> {
    format::render().view(v, "need/edit.html", data!({"item": item}))
}
pub fn edit_one(v: &impl ViewRenderer, item: &needs::Model) -> Result<Response> {
    format::render().view(v, "need/edit_one.html", data!({"item": item}))
}
pub fn login(v: &impl ViewRenderer) -> Result<Response> {
    format::render().view(v, "need/login.html", data!({}))
}

pub fn home(v: impl ViewRenderer, path: String) -> Result<impl IntoResponse> {
    format::render().view(&v, "home/hello.html", data!({ "path" : path}))
}
