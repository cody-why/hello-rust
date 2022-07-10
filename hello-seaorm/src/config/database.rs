/*** 
 * @Author: plucky
 * @Date: 2022-07-08 10:09:41
 * @LastEditTime: 2022-07-10 07:35:09
 * @Description: 
 */

use std::time::Duration;

use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use tracing::debug;

#[allow(dead_code)]
#[derive(Debug)]
pub struct MysqlConfig{
    url: String,
    max_connections: u32,
    min_connections: u32,
}

// 连接池
pub async fn init_mysql_pool() -> DatabaseConnection {
    dotenv::dotenv().ok();
    let url = dotenv::var("DATABASE_URL").unwrap();
    let mut opt = ConnectOptions::new(url);
    opt.max_connections(10)
    .min_connections(1)
    .connect_timeout(Duration::from_secs(30))
    .idle_timeout(Duration::from_secs(600))
    .max_lifetime(Duration::from_secs(30*60))
    .sqlx_logging(false);
    
    debug!("{:?}", opt);
     Database::connect(opt).await.expect("connect mysql error")

}

// 单连接
pub async fn set_up_db() -> Result<DatabaseConnection, DbErr> {
    let url = dotenv::var("DATABASE_URL").unwrap();
    let db = sea_orm::Database::connect(url).await?;

    Ok(db)
}

