use rbatis::executor;
use crate::table::{Users, Comments};

pub async fn insert_users(conn: &dyn executor::Executor, users: &Users) {
    Users::insert(conn, users).await;
}

pub async fn insert_comments(conn: &dyn executor::Executor, comments: &Comments) {
    Comments::insert(conn, comments).await;
}
