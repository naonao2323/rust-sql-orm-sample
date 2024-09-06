use crate::prisma::{comments, users::{self, manager_id}, PrismaClient};

pub async fn create_user(client: &PrismaClient, id: i32, name: String, manager_id: i32) {
    client
    .users()
    .create(
        name, 
        vec![
            users::id::set(id),
            users::manager_id::set(Some(manager_id))
        ]
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
            users::UniqueWhereParam::IdEquals(user_id),
            vec![
                comments::id::set(commnet_id),
            ]
        )
        .exec()
        .await
        .expect("fail to create comment");
}