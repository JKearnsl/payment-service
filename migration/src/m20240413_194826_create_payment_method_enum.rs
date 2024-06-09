use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_query::extension::postgres::Type;
use crate::sea_orm::DbBackend;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        match db.get_database_backend() {
            DbBackend::MySql | DbBackend::Sqlite => {}
            DbBackend::Postgres => {
                manager
                    .create_type(
                        Type::create()
                            .as_enum(PaymentMethod::Enum)
                            .values([
                                PaymentMethod::Card,
                                PaymentMethod::QrCode,
                            ])
                            .to_owned(),
                    )
                    .await?;
            }
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        match db.get_database_backend() {
            DbBackend::MySql | DbBackend::Sqlite => {}
            DbBackend::Postgres => {
                manager
                    .drop_type(Type::drop().name(PaymentMethod::Enum).to_owned())
                    .await?;
            }
        }

        Ok(())
    }
}

#[derive(DeriveIden)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "payment_method")]
pub enum PaymentMethod {
    #[sea_orm(iden = "payment_method")]
    Enum,

    #[sea_orm(string_value = "card")]
    Card,

    #[sea_orm(string_value = "qrcode")]
    QrCode,
}

