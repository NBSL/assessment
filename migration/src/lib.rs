pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_devices;
mod m20240130_210657_create_questions;
mod z_seed_data;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_devices::Migration),
            Box::new(m20240130_210657_create_questions::Migration),
            Box::new(z_seed_data::Migration),
        ]
    }
}
