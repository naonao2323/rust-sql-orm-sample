use diesel::mysql::{Mysql, MysqlConnection};
use std::sync::{Arc, Mutex};
use diesel::r2d2::{self, ConnectionManager, PooledConnection};
use diesel::{Connection, ExpressionMethods};
use diesel::RunQueryDsl;
use crate::model;

pub async fn create_user(conn: Arc<Mutex<PooledConnection<ConnectionManager<MysqlConnection>>>>, uid: i32) {
    use self::model::schema::users::dsl::*;
    let r = tokio::task::spawn_blocking(move || {
        let mut conn = conn.lock().unwrap();
        diesel::insert_into(users)
            .values((name.eq("ddd"), id.eq(uid), manager_id.eq(2)))
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

pub async fn create_user_and_comment(conn: Mutex<PooledConnection<ConnectionManager<MysqlConnection>>>) {

    let r = tokio::task::spawn_blocking(move || {
        let mut conn = conn.lock().unwrap(); 

        conn.transaction::<_, diesel::result::Error, _>(|conn| {
            {
                use self::model::schema::users::dsl::*;
                diesel::insert_into(users)
                .values((name.eq("ddd"), id.eq(100), manager_id.eq(2)))
                .execute(conn)?;
            }
            {
                use self::model::schema::comments::dsl::*;
                diesel::insert_into(comments)
                .values((test.eq("ddd"), id.eq(100), user_id.eq(100)))
                .execute(conn)
                .expect("Error deleting user");
            }
            Ok(())
        })
    })
    .await
    .expect("Failed to insert user");
}
