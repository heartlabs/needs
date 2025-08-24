use loco_rs::prelude::*;

use crate::models::_entities::needs;

/// Render a list view of `needs`.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn list(v: &impl ViewRenderer, items: &Vec<needs::Model>) -> Result<Response> {
    format::render().view(v, "need_html/list.html", data!({"items": items}))
}

/// Render a single `need` view.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn show(v: &impl ViewRenderer, item: &needs::Model) -> Result<Response> {
    format::render().view(v, "need_html/show.html", data!({"item": item}))
}

/// Render a `need` create form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn create(v: &impl ViewRenderer) -> Result<Response> {
    format::render().view(v, "need_html/create.html", data!({}))
}

/// Render a `need` edit form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn edit(v: &impl ViewRenderer, item: &needs::Model) -> Result<Response> {
    format::render().view(v, "need_html/edit.html", data!({"item": item}))
}
