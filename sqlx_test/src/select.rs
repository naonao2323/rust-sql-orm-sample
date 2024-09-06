use sqlx::mysql::MySqlPool;

pub struct Comment {
    id: i32,
    user_id: i32,
    test: String
}

pub async fn select_comments_by_v2(pool: &MySqlPool, user_id: i32) -> Vec<String> {
    sqlx::query!(
        "SELECT 
        c.test 
        FROM users as u INNER JOIN comments as c 
        ON u.id = c.user_id
        WHERE c.user_id = ?",
        user_id
    )
    .fetch_all(pool)
    .await
    .expect("fail to get users")
    .into_iter()
    .map(|record| record.test)
    .collect()
}