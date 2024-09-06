use diesel::mysql::{Mysql, MysqlConnection};
use std::sync::{Arc, Mutex};
use diesel::r2d2::{self, ConnectionManager, PooledConnection};
use diesel::QueryDsl;
use diesel::SelectableHelper;
use diesel::RunQueryDsl;
use crate::model;

pub async fn get_users(conn: Arc<Mutex<PooledConnection<ConnectionManager<MysqlConnection>>>>) -> Result<Vec<model::models::User>, diesel::result::Error> {
    use self::model::schema::users::dsl::*;
    let r = tokio::task::spawn_blocking(move || {
        let mut conn = conn.lock().unwrap();
        users
            .limit(3)
            .select(model::models::User::as_select())
            .load(&mut *conn)
    })
    .await
    .expect("fail to get users");
    r
}
