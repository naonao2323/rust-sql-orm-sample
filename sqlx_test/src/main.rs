use sqlx_test::{connection::new_poll, delete::delete_user, get::{get_user, User}, insert::{create_commnet, create_user}, select::select_comments_by_v2, update::update_users};

#[tokio::main]
async fn main() {
    let pool = new_poll().await;

    let _ = create_user(&pool, 1, "test".to_string(), 1).await;

    let _ = create_commnet(&pool, 1, 1, "test".to_string()).await;

    let _ = update_users(&pool, 1, "test1".to_string()).await;

    let user = get_user(&pool, 1).await;
    println!("{:?}", user);

    let comments = select_comments_by_v2(&pool, 1).await;
    println!("{:?}", comments);

    let _ = delete_user(&pool, 1);
}
