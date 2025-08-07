use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AuthUsers::Table)
                    .if_not_exists()
                    .col(pk_auto(AuthUsers::Id))
                    .col(
                        ColumnDef::new(AuthUsers::Email)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(AuthUsers::Username)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(string(AuthUsers::Password))
                    .col(string_null(AuthUsers::FirstName))
                    .col(string_null(AuthUsers::LastName))
                    .col(boolean(AuthUsers::IsActive).not_null().default(true))
                    .col(boolean(AuthUsers::IsVerified).not_null().default(false))
                    .col(boolean(AuthUsers::IsSuperuser).not_null().default(false))
                    .col(boolean(AuthUsers::IsStaff).not_null().default(false))
                    .col(timestamp(AuthUsers::LastLogin).not_null())
                    .col(
                        timestamp(AuthUsers::CreatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp(AuthUsers::UpdatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AuthUsers::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum AuthUsers {
    Table,
    Id,
    Email,
    Username,
    Password,
    FirstName,
    LastName,
    IsActive,
    IsVerified,
    IsSuperuser,
    IsStaff,
    LastLogin,
    CreatedAt,
    UpdatedAt,
}
