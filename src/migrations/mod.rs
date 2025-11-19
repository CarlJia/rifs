use sea_orm_migration::prelude::*;

mod m20240101_000001_create_images_table;
mod m20241201_000001_create_cache_table;
mod m20250101_000001_add_original_filename;
mod m20250201_000001_create_api_tokens_table;
mod m20250201_000002_add_owner_to_images;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240101_000001_create_images_table::Migration),
            Box::new(m20241201_000001_create_cache_table::Migration),
            Box::new(m20250101_000001_add_original_filename::Migration),
            Box::new(m20250201_000001_create_api_tokens_table::Migration),
            Box::new(m20250201_000002_add_owner_to_images::Migration),
        ]
    }
}
