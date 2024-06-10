pub use sea_orm_migration::prelude::*;


mod m20240612_063317_create_user;
mod m20240614_133643_create_payment;
mod m20240613_133643_create_token;
mod m20240413_194825_create_payment_state_enum;
mod m20240413_194826_create_payment_method_enum;


pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240612_063317_create_user::Migration),
            Box::new(m20240613_133643_create_token::Migration),
            Box::new(m20240413_194825_create_payment_state_enum::Migration),
            Box::new(m20240413_194826_create_payment_method_enum::Migration),
            Box::new(m20240614_133643_create_payment::Migration),
        ]
    }
}
