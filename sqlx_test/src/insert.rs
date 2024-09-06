use sqlx::mysql::MySqlPool;

pub async fn create_user(pool: &MySqlPool, user_id: i32, name: String, manager_id: i32) {
    sqlx::query!(
        "INSERT INTO users (id, name, manager_id) VALUES(?, ?, ?)",
        user_id,
        name,
        manager_id
    )
    .execute(pool)
    .await;
}

pub async fn create_commnet(pool: &MySqlPool, comment_id: i32, user_id: i32, test: String) {
    sqlx::query!(
        "INSERT INTO comments(id, user_id, test) VALUES(?, ?, ?)",
        comment_id,
        user_id,
        test
    )
    .execute(pool)
    .await;
}