use sea_orm::DatabaseConnection;
use sea_orm::prelude::*;
use sea_orm::{EntityTrait, Set};

use crate::entities::users::Entity as Users;
use crate::entities::users::ActiveModel as UsersModel;

pub async fn delete_user(connection: &DatabaseConnection, id: i32) {
    let _ = Users::delete_by_id(id).exec(connection).await;
}
