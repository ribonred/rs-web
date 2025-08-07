use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_index(
                Index::create()
                    .name("idx_auth_users_email")
                    .table(AuthUsersTable::Table)
                    .col(AuthUsersTable::Email)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_auth_users_username")
                    .table(AuthUsersTable::Table)
                    .col(AuthUsersTable::Username)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_auth_users_is_active")
                    .table(AuthUsersTable::Table)
                    .col(AuthUsersTable::IsActive)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_auth_users_active_verified")
                    .table(AuthUsersTable::Table)
                    .col(AuthUsersTable::IsActive)
                    .col(AuthUsersTable::IsVerified)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("idx_auth_users_active_verified")
                    .table(AuthUsersTable::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_auth_users_is_active")
                    .table(AuthUsersTable::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_auth_users_username")
                    .table(AuthUsersTable::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_auth_users_email")
                    .table(AuthUsersTable::Table)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

/// Note: We only define what we need for this migration
#[derive(DeriveIden)]
enum AuthUsersTable {
    #[sea_orm(iden = "auth_users")]
    Table,
    #[sea_orm(iden = "email")]
    Email,
    #[sea_orm(iden = "username")]
    Username,
    #[sea_orm(iden = "is_active")]
    IsActive,
    #[sea_orm(iden = "is_verified")]
    IsVerified,
}
