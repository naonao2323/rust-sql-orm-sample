use diesel::mysql::{Mysql, MysqlConnection};
use std::sync::{Arc, Mutex};
use diesel::r2d2::{self, ConnectionManager, PooledConnection};
use diesel::ExpressionMethods;
use diesel::RunQueryDsl;
use crate::model;

pub async fn create_user(conn: Arc<Mutex<PooledConnection<ConnectionManager<MysqlConnection>>>>) {
    use self::model::schema::users::dsl::*;
    let r = tokio::task::spawn_blocking(move || {
        let mut conn = conn.lock().unwrap();
        diesel::insert_into(users)
            .values((name.eq("ddd"), id.eq(1), manager_id.eq(2)))
            .execute(&mut *conn)
            .expect("Error deleting user");
    })
    .await
    .expect("fail to insert users");
}

pub async fn create_commnet(conn: Arc<Mutex<PooledConnection<ConnectionManager<MysqlConnection>>>>, uid: i32) {
    use self::model::schema::comments::dsl::*;

    let r = tokio::task::spawn_blocking(move || {
        let mut conn = conn.lock().unwrap();
        diesel::insert_into(comments)
            .values((test.eq("ddd"), id.eq(1), user_id.eq(uid)))
            .execute(&mut *conn)
            .expect("Error deleting user");
    })
    .await
    .expect("fail to insert users");
}