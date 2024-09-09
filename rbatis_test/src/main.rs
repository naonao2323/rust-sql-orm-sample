#[macro_use]
extern crate rbatis;

use rbatis::RBatis;
use serde_json::json;
use rbatis_test::{insert, update, delete, get, select, table};

#[tokio::main]
async fn main() {
    let database_url =  "mysql://root:root@mysql:3305/app";
    let rb = RBatis::new();
    rb.init(rbdc_mysql::driver::MysqlDriver {}, &database_url).unwrap();

    let users = table::Users{
        id: 2.into(),
        name: "test".to_string().into(),
        manager_id: 1.into()
    };

    let comments = table::Comments{
        id: 1.into(),
        test: "test".to_string(),
        user_id: 2
    };

    let mut tx = rb.acquire_begin().await.unwrap();
    insert::insert_users(&tx, &users).await;
    insert::insert_comments(&tx, &comments).await;
    tx.commit().await.unwrap();

    let users = table::Users{
        id: 2.into(),
        name: "test3".to_string().into(),
        manager_id: 1.into()
    };

    update::update_users(&rb, &users).await;

    // let resp = get::get_users(&rb, 2).await;
    // println!("{}", json!(resp));

    let resp = select::get_users_v1(&rb, 2).await.expect("fail to select comments with user id");
    println!("{}", json!(resp));

    let resp = select::get_users_v2(&rb, 2).await.expect("fail to get comments with user id");
    println!("{}", resp);

    delete::delete_users(&rb, &users).await;
}

