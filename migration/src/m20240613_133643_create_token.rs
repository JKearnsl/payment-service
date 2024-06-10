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
                    .table(Token::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Token::Id)
                            .uuid()
                            .not_null()
                            .extra("DEFAULT gen_random_uuid()")
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Token::UserId)
                        .uuid()
                        .not_null()
                    )
                     .foreign_key(
                         ForeignKey::create()
                             .from(Token::Table, Token::UserId)
                             .to(Users::Table, Users::Id)
                             .on_delete(ForeignKeyAction::Cascade)
                     )
                    .col(ColumnDef::new(Token::Hash).string_len(255).unique_key().not_null())
                    .col(
                        ColumnDef::new(Token::CreatedAt)
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
            .drop_table(Table::drop().table(Token::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Token {
    Table,
    Id,
    UserId,
    Hash,
    CreatedAt,
}
