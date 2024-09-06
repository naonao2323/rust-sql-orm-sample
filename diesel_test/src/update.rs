use diesel::mysql::{Mysql, MysqlConnection};
use std::sync::{Arc, Mutex};
use diesel::r2d2::{self, ConnectionManager, PooledConnection};
use diesel::ExpressionMethods;
use diesel::RunQueryDsl;
use crate::model;

pub async fn update_user(conn: Arc<Mutex<PooledConnection<ConnectionManager<MysqlConnection>>>>, uid: i32) {
    use self::model::schema::users::dsl::*;
    let r = tokio::task::spawn_blocking(move || {
        let mut conn = conn.lock().unwrap();
        diesel::update(users)
        .filter(id.eq(uid))
        .set(name.eq("test2"))
        .execute(&mut *conn)
        .expect("unexpetec update user");
    })
    .await;
}