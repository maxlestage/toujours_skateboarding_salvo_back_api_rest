pub use sea_orm_migration::prelude::*;

mod m20230301_124434_user_table;
mod m20230416_151948_data;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230301_124434_user_table::Migration),
            Box::new(m20230416_151948_data::Migration),
        ]
    }
}
