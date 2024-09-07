use crate::prisma::{comments, users::{self, manager_id}, PrismaClient};
use prisma_client_rust::QueryError;

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
            vec![]
        )
        .exec()
        .await
        .expect("fail to create comment");
}

pub async fn create_user_and_comment_v1(client: &PrismaClient) {
    let tx = client._transaction();
    let (_t, c) = tx.begin().await.expect("fail to begin transaction");
    create_user(&c, 100, "test".to_string(), 1).await;
    create_comment(&c, 100, 444).await;
    let r = _t.commit(c).await;
}

pub async fn create_user_and_comment_2(client: &PrismaClient) {
    client._transaction().run(|c| async move {
        create_user(&c, 100, "test".to_string(), 1).await;
        create_comment(&c, 100, 444).await;
        Ok::<(), QueryError>(())
    }).await;
}
