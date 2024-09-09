use rbatis::{Error, RBatis};
use rbatis::executor;
use rbatis::py_sql;
use rbs::{Value, to_value};

use crate::table::{Users, Comments};


#[py_sql(
    "
    `SELECT c.id, c.test, c.user_id FROM users as u INNER JOIN comments as c ON u.id = c.user_id`
    ` WHERE u.id = #{user_id}`
    "
)]
pub async fn get_users_v1(conn: &dyn executor::Executor, user_id: i32) -> Result<Vec<Comments>, Error> {
    impled!()
}

pub async fn get_users_v2(conn: &dyn executor::Executor, user_id: i32) -> Result<Value, Error> {
    conn
    .query(
        "SELECT c.id, c.test, c.user_id FROM users as u INNER JOIN comments as c
            ON u.id = c.user_id
            WHERE u.id = ?",
        vec![to_value!(user_id)]
    )
    .await
}

