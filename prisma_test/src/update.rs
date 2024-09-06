use crate::prisma::{users, PrismaClient};

pub async fn update_user(client: &PrismaClient, id: i32, name: String) {
    client
        .users()
        .update(
            users::id::equals(id), 
            vec![
                users::name::set(name),
            ]
        )
        .exec()
        .await
        .expect("fail to update user");
}
