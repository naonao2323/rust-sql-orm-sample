use sqlx::mysql::MySqlPool;

pub async fn delete_user(pool: &MySqlPool, user_id: i32) {
    sqlx::query!(
        "DELETE FROM users WHERE id = ?", 
        user_id
    ).
    execute(pool)
    .await;
}