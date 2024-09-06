
use sea_orm::DatabaseConnection;
use sea_orm::prelude::*;
use sea_orm::{EntityTrait, QueryTrait};

use crate::entities::users::Entity as Users;
use crate::entities::users::Model as UsersModel;

pub async fn get_users(conn: &DatabaseConnection) -> Vec<UsersModel> {
    Users::find().all(conn).await.expect("fail to get users")
}
