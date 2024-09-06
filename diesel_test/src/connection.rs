use diesel::mysql::{Mysql, MysqlConnection};
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use diesel::r2d2::{self, ConnectionManager};

pub type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub fn establish_connection() -> DbPool {
    let database_url =  "mysql://root:root@mysql:3305/app";
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    r2d2::Pool::builder()
    .build(manager)
    .expect("Failed to create pool.")
}
