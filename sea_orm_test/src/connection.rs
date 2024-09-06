use sea_orm::Database;
use sea_orm::DatabaseConnection;

pub async fn new_poll() -> Result<DatabaseConnection, sea_orm::DbErr> {
    let db_url = "mysql://root:root@mysql:3305/app";
    let pool: DatabaseConnection = Database::connect(db_url).await?;
    Ok(pool)
}

