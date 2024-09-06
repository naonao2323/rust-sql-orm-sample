use sea_orm::DatabaseConnection;
use sea_orm::prelude::*;
use sea_orm::{EntityTrait, Set};


use crate::entities::users::Entity as Users;
use crate::entities::users::ActiveModel as UsersModel;

use crate::entities::comments::Entity as Comments;
use crate::entities::comments::ActiveModel as CommentsModel;

pub async fn create_user(conn: &DatabaseConnection, id: i32, name: String, manager_id: i32) {
    let user: UsersModel = UsersModel {
        id: Set(id.to_owned()),
        name: Set(name.to_owned()),
        manager_id: Set(manager_id.to_owned())
    };
    let _ = Users::insert(user).exec(conn).await;
}


pub async fn create_comments(conn: &DatabaseConnection) {
    let comments = CommentsModel {
        test: Set("test".to_string().to_owned()),
        user_id: Set(1.to_owned()),
        id: Set(1.to_owned())
    };

    let _ = Comments::insert(comments).exec(conn).await;
}