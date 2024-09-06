use crate::prisma::{users, comments, PrismaClient};
use serde::Deserialize;
use prisma_client_rust::{raw, PrismaValue};

#[derive(Deserialize, Debug)]
pub struct QueryReturnType {
    test: String
}

pub async fn select_commensts_by_v2(client: &PrismaClient, user_id: i32) -> Vec<QueryReturnType> {
    let data: Vec<QueryReturnType> = client._query_raw(
        raw!(
            "SELECT test
            FROM users INNER JOIN comments 
            ON users.id = comments.user_id
            WHERE users.id = {}",
            PrismaValue::String(user_id.to_string())
        )
    )
    .exec()
    .await
    .expect("fail to fetch commnets");
    data
}

pub async fn select_commencts_by_v1(client: &PrismaClient, user_id: i32) -> Vec<users::Data>{
    let comment_with_users = client
    .users()
    .find_many(vec![users::id::equals(user_id)])
    .with(users::comments::fetch(vec![]))
    .exec()
    .await
    .expect("Failed to fetch user with comments");
    comment_with_users
}
