#[allow(warnings, unused)]
 
use prisma_test::prisma::PrismaClient;
use prisma_client_rust::NewClientError;
use prisma_test::{insert, get, update, delete, select};
 
#[tokio::main]
async fn main() {
    let client = PrismaClient::_builder().build().await.expect("fail to create prisma client");

    let _ = insert::create_user(&client, 1, "test".to_string(), 1).await;

    let _ = update::update_user(&client, 1, "test2".to_string()).await;

    let _ = insert::create_comment(&client, 1, 1).await;

    // let users = get::get_users(&client).await;
    // println!("{:?}", users);

    let comments = select::select_commensts_by_v2(&client, 1).await;
    println!("{:?}", comments);

    let comment_with_users = select::select_commencts_by_v1(&client, 1).await;
    println!("{:?}", comment_with_users);

    let _ = delete::delete_user(&client, 1).await;
}
