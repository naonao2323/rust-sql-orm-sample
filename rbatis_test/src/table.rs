use rbatis::RBatis;

//users table
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Users {
    pub id: i32,
    pub name: String,
    pub manager_id: i32
}

//comments table
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Comments {
    pub id: i32,
    pub test: String,
    pub user_id: i32, // foreign key
}
rbatis::crud!(Users{});
rbatis::crud!(Comments{});
