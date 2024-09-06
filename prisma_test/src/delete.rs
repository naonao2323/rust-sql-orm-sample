use crate::prisma::{users, PrismaClient};

pub async fn delete_user(client: &PrismaClient, id: i32) {
    client
    .users()
    .delete(users::id::equals(id))
    .exec()
    .await
    .expect("fail to delete users");
}
