use axum::{
    extract::{Extension, MatchedPath, Request, State}, 
    middleware::{self, Next}, 
    response::{Html, IntoResponse}, 
    routing::get, Router
};

use diesel::mysql::{Mysql, MysqlConnection};

use std::sync::{Arc, Mutex};

use diesel::r2d2::{self, ConnectionManager, PooledConnection};

use diesel_test::{connection, select::{self, select_comments_by, select_comments_by_v2}};
use diesel_test::get;
use diesel_test::delete;
use diesel_test::insert;
use diesel_test::update;

#[tokio::main]
async fn main() {
    let database_url = "mysql://root:root@mysql:3305/app";
    let pool = connection::establish_connection(&database_url);

    let conn = match pool.get() {
        Ok(conn) => Arc::new(Mutex::new(conn)),
        Err(_) => panic!("fail to get connection"),
    };
    insert::create_user(conn.clone(), 1).await;
    update::update_user(conn.clone(), 1).await;
    insert::create_commnet(conn.clone(), 1).await;

    let conn = match pool.get() {
        Ok(conn) => Mutex::new(conn),
        Err(_) => panic!("fail to get connection"),
    };
    insert::create_user_and_comment(conn).await;

    let conn = match pool.get() {
        Ok(conn) => Arc::new(Mutex::new(conn)),
        Err(_) => panic!("fail to get connection"),
    };

    let comments = select::select_comments_by(1, conn.clone()).await;
    println!("{:?}", comments);

    let comments_v2 = select::select_comments_by_v2(1, conn.clone()).await;
    println!("{:?}", comments_v2);
    
    let users = get::get_users(conn.clone()).await;
    print!("{:?}", users);

    let conn = match pool.get() {
        Ok(conn) => Arc::new(Mutex::new(conn)),
        Err(_) => panic!("fail to get connection"),
    };

    delete::delete_user_by(1, conn).await;
}
