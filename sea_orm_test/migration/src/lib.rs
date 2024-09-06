pub use sea_orm_migration::prelude::*;

mod m20240906_125042_create_users;
mod m20240906_132527_create_comments;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240906_125042_create_users::Migration),
            Box::new(m20240906_132527_create_comments::Migration),
        ]
    }
}
