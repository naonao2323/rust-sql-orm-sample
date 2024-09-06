use sqlx::mysql::MySqlPool;

pub async fn update_users(pool: &MySqlPool, user_id: i32, name: String) {
    sqlx::query!(
        "UPDATE users SET name = ? WHERE id = ?",
        name,
        user_id
    )
    .execute(pool)
    .await;
}