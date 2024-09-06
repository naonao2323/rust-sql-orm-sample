use crate::prisma::PrismaClient;

pub async fn get_users(client: &PrismaClient) -> Vec<crate::prisma::users::Data> {
    client
    .users()
    .find_many(vec![])
    .exec()
    .await
    .expect("fail to get users")
}
