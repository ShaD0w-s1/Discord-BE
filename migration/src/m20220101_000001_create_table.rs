/*
 * @Author: zhangyuxuan
 * @Date: 2022-07-04 17:10:11
 * @LastEditTime: 2022-07-07 22:55:17
 * @LastEditors: zhangyuxuan
 * @FilePath: \Discord-BE\migration\src\m20220101_000001_create_table.rs
 */
use sea_orm_migration::{prelude::*, sea_orm::schema::Schema};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000001_create_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Schema::new(manager.get_database_backend())
                    .create_table_from_entity(entity::duty::Entity),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(entity::duty::Entity).to_owned())
            .await
    }
}
