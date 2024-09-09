use rbatis::executor;
use crate::table::{Users, Comments};

pub async fn get_users(conn: &dyn executor::Executor, user_id: i32) -> Vec<Users> {
    let users = Users::select_by_column(conn, "id", user_id)
    .await
    .expect("fail to get users");
    users
}