#[allow(unused_imports)]
use loco_rs::{cli::playground, prelude::*};
use needs::app::App;

#[tokio::main]
async fn main() -> loco_rs::Result<()> {
    let ctx = playground::<App>().await?;

    let active_model: needs::models::needs::ActiveModel = needs::models::needs::ActiveModel {
        title: Set("Space".to_string()),
        value: Set(50),
        user_id: Set(1),
        ..Default::default()
    };

    active_model.insert(&ctx.db).await.unwrap();

    let needs = needs::models::needs::Entity::find()
        .all(&ctx.db)
        .await
        .unwrap();

    let need = needs.first().unwrap();

    let user = need
        .find_related(needs::models::users::Entity)
        .all(&ctx.db)
        .await?;
    println!("{:?}", need);
    println!("{:?}", user);

    Ok(())
}
