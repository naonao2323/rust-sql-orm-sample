use sqlx::mysql::MySqlPool;
use sqlx::{ MySql, Acquire};

pub async fn create_user(pool: impl Acquire<'_, Database = MySql>, user_id: i32, name: String, manager_id: i32) {
    let mut conn = pool.acquire().await.expect("fail to get connection");
    sqlx::query!(
        "INSERT INTO users (id, name, manager_id) VALUES(?, ?, ?)",
        user_id,
        name,
        manager_id
    )
    .execute(&mut *conn)
    .await;
}

pub async fn create_commnet(pool: impl Acquire<'_, Database = MySql>, comment_id: i32, user_id: i32, test: String) {
    let mut conn = pool.acquire().await.expect("fail to get connection");
    sqlx::query!(
        "INSERT INTO comments(id, user_id, test) VALUES(?, ?, ?)",
        comment_id,
        user_id,
        test
    )
    .execute(&mut *conn)
    .await;
}

pub async fn create_user_and_comment_v1(pool: impl Acquire<'_, Database = MySql>, user_id: i32, comment_id: i32, test: String) {
    let mut conn = pool.acquire().await.expect("fail to get connection");
    let mut tx = conn.begin().await.expect("fail to get transaction");
    create_user(&mut tx, user_id, "test".to_string(), 1).await;
    create_commnet(&mut tx, comment_id, user_id, test).await;
}