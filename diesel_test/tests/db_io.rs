use diesel::RunQueryDsl;
use diesel_test::model::models::UserComment;
use diesel_test::model::schema::comments;
use diesel_test::{connection, delete, get, insert, select, update};
use diesel::mysql::{Mysql, MysqlConnection};
use std::sync::{Arc, Mutex};
use diesel::r2d2::{self, ConnectionManager, PooledConnection};
use diesel::r2d2::Pool;
use diesel_test::model;
use diesel::result::Error;


#[tokio::test] 
async fn main() {
    let database_url = "mysql://root:root@127.0.0.1:3305/app";
    let pool = connection::establish_connection(&database_url);
    delete_all(&pool);
    upsert_and_get(&pool).await;
    delete_all(&pool);
    insert_and_delete(&pool).await;
    delete_all(&pool);
    insert_and_select(&pool).await;
    delete_all(&pool);
}

fn delete_all(pool: &Pool<ConnectionManager<MysqlConnection>>) {
    use self::model::schema::users::dsl::*;

    let mut conn = match pool.get() {
        Ok(conn) => Arc::new(Mutex::new(conn)),
        Err(_) => panic!("fail to get connection"),
    };

    diesel::delete(users).execute(&mut *conn.lock().unwrap());
}

pub async fn upsert_and_get(pool: &Pool<ConnectionManager<MysqlConnection>>) -> Result<(), Error> {
    let conn = match pool.get() {
        Ok(conn) => Arc::new(Mutex::new(conn)),
        Err(_) => panic!("fail to get connection"),
    };
    let _ = insert::create_user(conn.clone(), 1).await;
    let _ = update::update_user(conn.clone(), 1).await;
    let users = get::get_users(conn.clone()).await?;

    let expected_users: Vec<model::models::User> = vec![
        model::models::User{ id: 1, name: "test2".to_string(), manager_id: 2 },
    ];
    assert_eq!(expected_users, users);
    Ok(())
}

pub async fn insert_and_delete(pool: &Pool<ConnectionManager<MysqlConnection>>) -> Result<(), Error> {
    let conn = match pool.get() {
        Ok(conn) => Arc::new(Mutex::new(conn)),
        Err(_) => panic!("fail to get connection"),
    };
    let _ = insert::create_user(conn.clone(), 1).await;
    let cnt = delete::delete_user_by(1, conn.clone()).await;

    assert_eq!(1, cnt);
    Ok(())
}

pub async fn insert_and_select(pool: &Pool<ConnectionManager<MysqlConnection>>) -> Result<(), Error> {
    let conn = match pool.get() {
        Ok(conn) => Arc::new(Mutex::new(conn)),
        Err(_) => panic!("fail to get connection"),
    };
    let _ = insert::create_user(conn.clone(), 1).await;
    let _ = insert::create_commnet(conn.clone(), 1).await;

    let comments = match select::select_comments_by(1, conn.clone()).await { 
        Ok(cs) => cs,
        Err(_) => panic!("fail to get comments"),
    };

    let expected: Vec<model::models::UserComment> = vec![
        model::models::UserComment{test: "ddd".to_string()},
    ];

    assert_eq!(expected, comments);
    Ok(())
}