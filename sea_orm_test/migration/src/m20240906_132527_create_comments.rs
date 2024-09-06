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
                CREATE TABLE `comments` (
                `test` varchar(255) NOT NULL,
                `user_id` int NOT NULL,
                `id` int NOT NULL,
                PRIMARY KEY (`id`),
                INDEX `user_id_fk` (`user_id`),
                CONSTRAINT `user_id_fk` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`) ON UPDATE NO ACTION ON DELETE CASCADE
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
            DROP TABLE `comments`;
            "
        )
        .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Post {
    Table,
    Id,
    Title,
    Text,
}
