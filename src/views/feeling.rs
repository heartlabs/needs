use loco_rs::prelude::*;

use crate::models::_entities::feelings;

/// Render a list view of `feelings`.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn list(v: &impl ViewRenderer, items: &Vec<feelings::Model>) -> Result<Response> {
    format::render().view(v, "feeling/list.html", data!({"items": items}))
}

pub fn edit_one(v: &impl ViewRenderer, item: &feelings::Model) -> Result<Response> {
    format::render().view(v, "feeling/edit_one.html", data!({"item": item}))
}
