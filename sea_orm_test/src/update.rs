use sea_orm::DatabaseConnection;
use sea_orm::prelude::*;
use sea_orm::{EntityTrait, Set};

use crate::entities::users::Entity as Users;
use crate::entities::users::ActiveModel as UsersModel;
use crate::entities::users::Column as UsersColumn;

pub async fn update_user(connection: &DatabaseConnection, id: i32, name: String) {
    let _ = Users::update_many()
    .col_expr(UsersColumn::Name, Expr::value(name))
    .filter(UsersColumn::Id.eq(id))
    .exec(connection)
    .await;  
}