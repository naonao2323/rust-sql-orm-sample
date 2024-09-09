use rbatis::RBatis;
use rbatis::executor;
use crate::table::{Users, Comments};

pub async fn delete_users(conn: &dyn executor::Executor, users: &Users) {
    Users::delete_by_column(conn, "id", users.id).await;
}
