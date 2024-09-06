use diesel::associations::HasTable;
use diesel::{sql_query, JoinOnDsl, QueryDsl};
use diesel::mysql::{Mysql, MysqlConnection};
use tokio::task::spawn_blocking;
use std::sync::{Arc, Mutex};
use diesel::r2d2::{self, ConnectionManager, PooledConnection};
use diesel::ExpressionMethods;
use diesel::RunQueryDsl;
use crate::model;
use crate::model::schema::comments::id;


pub async fn select_comments_by(user_id: u32, conn: Arc<Mutex<PooledConnection<ConnectionManager<MysqlConnection>>>>) -> Result<Vec<model::models::UserComment>, tokio::task::JoinError>{
    use self::model::schema::users::dsl::*;
    use diesel::ExpressionMethods;
    let r = tokio::task::spawn_blocking(move || {
        let mut conn = conn.lock().unwrap();
        let q = format!("
            SELECT test
            FROM users INNER JOIN comments 
            ON users.id = comments.user_id
            WHERE users.id = {}            
        ", user_id);
        sql_query(q)
        .load::<model::models::UserComment>(&mut *conn)
        .unwrap()
    }).await;
    r
}

pub async fn select_comments_by_v2(
    uid: i32,
    conn: Arc<Mutex<PooledConnection<ConnectionManager<MysqlConnection>>>>
) -> Result<Vec<(String, String)>, diesel::result::Error> {
    use crate::model::schema::users::dsl::*;
    use crate::model::schema::comments::dsl::*;
    use crate::model::schema::comments;
    use diesel::ExpressionMethods;
    use diesel::QueryDsl;
    use diesel::RunQueryDsl;
    use crate::model::schema::users;

    let r = tokio::task::spawn_blocking(move || {
        let mut conn = conn.lock().unwrap();
        let resp = users
            .inner_join(comments.on(user_id.eq(users::id)))
            .filter(users::id.eq(uid))
            .select((users::name, comments::test))
            .load::<(String, String)>(&mut *conn);
        resp
    })
    .await
    .expect("fail to get comment v2");

    r
}