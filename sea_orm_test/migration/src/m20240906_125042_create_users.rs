use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let conn = manager.get_connection();

        conn.execute_unprepared(
            "
            CREATE TABLE `users` (
            `id` int NOT NULL,
            `name` varchar(255) NOT NULL,
            `manager_id` int NOT NULL,
            PRIMARY KEY (`id`)
            ) CHARSET utf8mb4 COLLATE utf8mb4_0900_ai_ci;
            "
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        let conn = manager.get_connection();

        conn.execute_unprepared(
            "
            DROP TABLE `users`;
            "
        )
        .await?;
        Ok(())
    }
}
