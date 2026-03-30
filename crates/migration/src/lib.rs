pub use sea_orm_migration::prelude::*;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260330_061253_create_user_prefs::Migration),
            Box::new(m20260330_063639_create_server_prefs::Migration),
            Box::new(m20260330_064340_create_translation_support::Migration),
        ]
    }
}
mod m20260330_061253_create_user_prefs;
mod m20260330_063639_create_server_prefs;
mod m20260330_064340_create_translation_support;
