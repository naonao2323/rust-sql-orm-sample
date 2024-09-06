use sqlx::mysql::MySqlPool;

#[derive(Debug)]
pub struct User {
    id: i32,
    name: String,
    manager_id: i32
}

pub async fn get_user(pool: &MySqlPool, user_id: i32) -> User {
    let user = sqlx::query_as!(
        User,
        "SELECT id, name, manager_id FROM users WHERE id = ?",
        user_id
    )
    .fetch_one(pool)
    .await
    .expect("fail to get user");
    user
}