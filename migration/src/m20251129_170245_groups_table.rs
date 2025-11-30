use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(Group::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Group::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Group::Name).string().not_null())
                    .to_owned()
                    .col(
                        ColumnDef::new(Group::Ticket)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .to_owned()
                    .col(ColumnDef::new(Group::Owner).integer().not_null())
                    .to_owned()
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-groups-owner-users-id")
                            .from(Group::Table, Group::Owner)
                            .to(User::Table, User::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Group::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Group {
    Table,
    Id,
    Name,
    Ticket,
    Owner,
}
#[derive(Iden)]
enum User {
    Table,
    Id,
}
