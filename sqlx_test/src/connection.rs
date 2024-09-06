use sqlx::mysql::MySqlPool;

pub async fn new_poll() -> MySqlPool {
    let db_url = "mysql://root:root@localhost:3305/app";
    let pool = MySqlPool::connect(db_url).await.expect("failt to connect db");
    pool
}
