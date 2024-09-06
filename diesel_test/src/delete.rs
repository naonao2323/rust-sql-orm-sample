use diesel::mysql::{Mysql, MysqlConnection};
use std::sync::{Arc, Mutex};
use diesel::r2d2::{self, ConnectionManager, PooledConnection};
use diesel::QueryDsl;
use diesel::SelectableHelper;
use diesel::RunQueryDsl;
use diesel::ExpressionMethods;
use diesel::sql_types::Integer;

use crate::model;

pub async fn delete_user_by(user_id: i32, conn: Arc<Mutex<PooledConnection<ConnectionManager<MysqlConnection>>>>) -> usize {
    use self::model::schema::users::dsl::*;

    let r: usize = tokio::task::spawn_blocking(move || {
        let mut conn = conn.lock().unwrap();
        diesel::delete(users.filter(id.eq(user_id)))
            .execute(&mut *conn)
            .expect("Error deleting user")
    })
    .await
    .expect("fail to delete user");
    r
}
