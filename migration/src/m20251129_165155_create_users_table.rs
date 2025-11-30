// migration/src/m20251129_165155_create_users_table.rs
use sea_orm_migration::prelude::*;
use sea_query::{ColumnDef, ColumnType};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20251129_165155_create_users_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::Name).string().not_null())
                    .col(
                        ColumnDef::new(User::PhoneNo)
                            .big_unsigned()
                            .not_null()
                            .unique_key(),
                    )
                    .to_owned()
                    .col(ColumnDef::new(User::Password).string_len(255).not_null())
                    .to_owned()
                    .col(ColumnDef::new(User::SecretKey).string().not_null())
                    .to_owned()
                    .col(
                        ColumnDef::new(User::Admin)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum User {
    Table,
    Id,
    Name,
    PhoneNo,
    Admin,
    Password,
    SecretKey,
}
