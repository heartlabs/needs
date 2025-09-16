#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;
mod m20220101_000001_users;

mod m20250824_084201_needs;
mod m20250824_092201_add_user_ref_to_needs;
mod m20250824_094609_add_dummy_user;
mod m20250915_211003_invites;
mod m20250916_164512_feelings;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20250824_084201_needs::Migration),
            Box::new(m20250824_092201_add_user_ref_to_needs::Migration),
            Box::new(m20250824_094609_add_dummy_user::Migration),
            Box::new(m20250915_211003_invites::Migration),
            Box::new(m20250916_164512_feelings::Migration),
            // inject-above (do not remove this comment)
        ]
    }
}