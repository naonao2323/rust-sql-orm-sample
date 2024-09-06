use sea_orm::DatabaseConnection;
use sea_orm::prelude::*;
use sea_orm::{EntityTrait, Set};
use sea_orm::DatabaseBackend;
use sea_orm::Statement;
use sea_orm::JoinType;
use crate::entities::users::Entity as Users;
use crate::entities::users::ActiveModel as UsersActiveModel;
use crate::entities::comments::Entity as Comments;
use crate::entities::comments::ActiveModel as CommentsActiveModel;
use crate::entities::comments::Model as CommentsModel;
use crate::entities::{comments, users, prelude::*};

pub async fn select_comments_by_v1(connection: &DatabaseConnection, user_id: i32) -> Vec<CommentsModel>{
    let comments = Comments::find()
        .inner_join(Users)
        .filter(users::Column::Id.eq(user_id))
        .all(connection)
        .await
        .expect("fail to get comments");
    
    comments
}

pub async fn select_comments_by_v2(connection: &DatabaseConnection, user_id: i32) -> Vec<QueryResult> {
    connection
    .query_all(
        Statement::from_sql_and_values(
            DatabaseBackend::MySql,
            "SELECT test
            FROM users INNER JOIN comments 
            ON users.id = comments.user_id
            WHERE users.id = ?",
            [user_id.into()]
        )
    )
    .await
    .expect("fail to get comments")
}