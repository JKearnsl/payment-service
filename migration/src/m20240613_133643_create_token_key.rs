use sea_orm_migration::prelude::*;
use crate::m20240612_063317_create_user::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TokenKeys::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TokenKeys::Id)
                            .uuid()
                            .not_null()
                            .extra("DEFAULT gen_random_uuid()")
                            .primary_key(),
                    )
                    .col(ColumnDef::new(TokenKeys::UserId)
                        .uuid()
                        .not_null()
                    )
                     .foreign_key(
                         ForeignKey::create()
                             .from(TokenKeys::Table, TokenKeys::UserId)
                             .to(Users::Table, Users::Id)
                             .on_delete(ForeignKeyAction::Cascade)
                     )
                    .col(ColumnDef::new(TokenKeys::Hash).string_len(255).unique_key().not_null())
                    .col(
                        ColumnDef::new(TokenKeys::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp())
                            .not_null()
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TokenKeys::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum TokenKeys {
    Table,
    Id,
    UserId,
    Hash,
    CreatedAt,
}
