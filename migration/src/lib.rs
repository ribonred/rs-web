pub use sea_orm_migration::prelude::*;
mod m20250807_065844_create_users_table;
mod m20250807_091101_add_auth_users_indexes;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250807_065844_create_users_table::Migration),
            Box::new(m20250807_091101_add_auth_users_indexes::Migration),
        ]
    }
}
