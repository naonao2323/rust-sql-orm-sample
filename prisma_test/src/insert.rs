use crate::prisma::{comments, users::{self, manager_id}, PrismaClient};

pub async fn create_user(client: &PrismaClient, id: i32, name: String, manager_id: i32) {
    client
    .users()
    .create(
        id,
        name, 
        manager_id,
        vec![],
    )
    .exec()
    .await
    .expect("fail to create user");
}


pub async fn create_comment(client: &PrismaClient, user_id: i32, commnet_id: i32) {
    client
        .comments()
        .create(
            "test".to_string(),
            commnet_id,
            users::UniqueWhereParam::IdEquals(user_id),
            vec![
                comments::user_id::set(user_id),
            ]
        )
        .exec()
        .await
        .expect("fail to create comment");
}