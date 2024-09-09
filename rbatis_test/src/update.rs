use rbatis::executor;
use crate::table::{Users, Comments};

pub async fn update_users(conn: &dyn executor::Executor, users: &Users) {
    Users::update_by_column(conn, users, "id").await;
}