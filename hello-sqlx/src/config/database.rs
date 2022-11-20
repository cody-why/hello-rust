/*
 * @Author: plucky
 * @Date: 2022-09-04 19:33:16
 * @LastEditTime: 2022-11-20 17:41:59
 * @Description: 
 */


use std::time::Duration;

use sqlx::mysql::{MySqlPoolOptions};
use sqlx::{MySql, Pool};
use tracing::debug;

use super::*;



pub async fn init_mysql_pool(config: &MysqlConfig) -> Pool<MySql> {
    dotenv::dotenv().ok();
    let url = &std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    _ = config.url;

    // let mut opt =  MySqlConnectOptions::new().
    // opt.log_statements(tracing::log::LevelFilter::Off);
   
    // mysql://user:pwd@host:port/db
    let pool = MySqlPoolOptions::new()
        .max_connections(config.max_connections)
        .min_connections(config.min_connections)
        // .max_lifetime(Some(Duration::from_secs(60 * 60)))
        .acquire_timeout(Duration::from_secs(60 * 60))
        //.connect_with(opt).await;
        .connect(url).await;
    debug!("mysql pool: {:?}", pool);
    pool.unwrap()
}


