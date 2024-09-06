use sea_orm_test::{connection, delete::delete_user, get::get_users, insert::{create_comments, create_user}, select::{self, select_comments_by_v2, select_comments_by_v1}, update::update_user};

#[tokio::main]
async fn main() {
    let pool = connection::new_poll().await.expect("fail to get connnection pool");

    let _ = create_user(&pool, 1, "test".to_string(), 1).await;

    let _ = update_user(&pool, 1, "test2".to_string()).await;

    let _ = create_comments(&pool).await;

    let users = get_users(&pool).await;
    print!("{:?}", users);

    let comments = select_comments_by_v1(&pool, 1).await;
    println!("{:?}", comments);

    let comments = select_comments_by_v2(&pool, 1).await;
    println!("{:?}", comments);

    let _ = delete_user(&pool, 1).await;
}

