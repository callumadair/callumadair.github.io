pub use sea_orm_migration::prelude::*;

mod m20250216_151738_software_tools;
mod m20250216_153522_image_links;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator
{
    fn migrations() -> Vec<Box<dyn MigrationTrait>>
    {
        vec![
            Box::new(m20250216_151738_software_tools::Migration),
            Box::new(m20250216_153522_image_links::Migration),
        ]
    }
}
