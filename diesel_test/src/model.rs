use diesel::prelude::*;
use diesel::Queryable;

pub mod schema {
    diesel::table! {
        users (id) {
            id -> Int4,
            name -> Varchar,
            manager_id -> Int4,
        }
    }
    diesel::table! {
        comments {
            id -> Int4,
            test -> Varchar,
            user_id -> Int4,
        }
    }
    diesel::joinable!(comments -> users(id));

    diesel::allow_tables_to_appear_in_same_query!(users, comments);
}

pub mod models {
    use super::schema::users;
    use super::schema::comments;
    use diesel::prelude::*;
    use diesel::sql_types::{Integer, Varchar};

    #[diesel(table_name = users)]
    #[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)]
    #[diesel(check_for_backend(diesel::mysql::Mysql))]
    pub struct User {
        pub id: i32,
        pub name: String,
        pub manager_id: i32,
    }

    #[diesel(table_name = comments)]
    #[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
    #[diesel(belongs_to(User))]
    #[diesel(check_for_backend(diesel::mysql::Mysql))]
    pub struct Commnet {
        pub id: i32,
        pub test: String,
        pub user_id: i32,
    }
    

    #[derive(QueryableByName, Debug, PartialEq)]
    pub struct UserComment {
        #[sql_type = "Varchar"]
        pub test: String,
    }
}
