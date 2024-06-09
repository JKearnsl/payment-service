use sea_orm_migration::prelude::*;
use crate::m20240413_194825_create_payment_state_enum::PaymentState;
use crate::m20240413_194826_create_payment_method_enum::PaymentMethod;
use crate::m20240612_063317_create_user::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Payment::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Payment::Id)
                            .uuid()
                            .not_null()
                            .extra("DEFAULT gen_random_uuid()")
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Payment::Amount)
                            .decimal_len(
                                10,
                                2,
                            )
                            .not_null()
                    )
                    .col(ColumnDef::new(Payment::SellerId)
                        .uuid()
                        .not_null()
                    )
                     .foreign_key(
                         ForeignKey::create()
                             .from(Payment::Table, Payment::SellerId)
                             .to(Users::Table, Users::Id)
                             .on_delete(ForeignKeyAction::Cascade)
                     )
                    .col(
                        ColumnDef::new(Payment::State)
                            .custom(PaymentState::Enum)
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(Payment::Method)
                            .custom(PaymentMethod::Enum)
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(Payment::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp())
                            .not_null()
                    )
                    .col(
                        ColumnDef::new(Payment::UpdatedAt)
                            .timestamp_with_time_zone()
                            .null()
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Payment::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Payment {
    Table,
    Id,
    State,
    Method,
    Amount,
    SellerId,
    CreatedAt,
    UpdatedAt,
}
