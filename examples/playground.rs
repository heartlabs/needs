#[allow(unused_imports)]
use loco_rs::{cli::playground, prelude::*};
use needs::app::App;

#[tokio::main]
async fn main() -> loco_rs::Result<()> {
    let ctx = playground::<App>().await?;

    let active_model: needs::models::needs::ActiveModel = needs::models::needs::ActiveModel {
        title: Set("Space".to_string()),
        value: Set(50),
        ..Default::default()
    };

    active_model.insert(&ctx.db).await.unwrap();

    let res = needs::models::needs::Entity::find()
        .all(&ctx.db)
        .await
        .unwrap();

    println!("{:?}", res);

    Ok(())
}
