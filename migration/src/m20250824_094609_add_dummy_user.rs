use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        let db = m.get_connection();

        db.execute_unprepared("INSERT INTO public.users (created_at,updated_at,pid,email,\"password\",api_key,\"name\",reset_token,reset_sent_at,email_verification_token,email_verification_sent_at,email_verified_at,magic_link_token,magic_link_expiration) VALUES
	 ('2025-08-24 12:03:53.934','2025-08-24 12:03:53.934','4803afb8-56af-4d6c-ac31-f54dd3788583'::uuid,'user@loco.rs','$argon2id$v=19$m=19456,t=2,p=1$queS+zgqSu/L3acUDKCDxQ$6JbG925fHdmHwCtygt/0WkS5soITt1a9k0Osqg72A00','lo-59467757-8196-434d-99e3-94ec6baea84e','Loco user',NULL,NULL,'4d7d1036-7c8a-4c9b-86bd-04d850026948','2025-08-24 12:03:55.001',NULL,NULL,NULL);
")
          .await?;
        Ok(())
    }

    async fn down(&self, _m: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}
