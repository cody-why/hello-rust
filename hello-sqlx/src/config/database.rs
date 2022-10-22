/*** 
 * @Author: plucky
 * @Date: 2022-09-04 20:03:53
 * @LastEditTime: 2022-09-04 22:25:47
 * @Description: 
 */



use sqlx::mysql::{MySqlPoolOptions};
use sqlx::{MySql, Pool};
use tracing::debug;

use super::*;



pub async fn init_mysql_pool(config: &MysqlConfig) -> Pool<MySql> {
    // let mut opt =  MySqlConnectOptions::new().
    // opt.log_statements(tracing::log::LevelFilter::Off);
   
    // mysql://user:pwd@host:port/db
    let pool = MySqlPoolOptions::new()
        .max_connections(config.max_connections)
        .min_connections(config.min_connections)
        //.connect_with(opt).await;
        .connect(&config.url).await;
    debug!("mysql pool: {:?}", pool);
    pool.unwrap()
}


