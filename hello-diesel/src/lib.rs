/*** 
 * @Author: plucky
 * @Date: 2022-07-05 19:00:17
 * @LastEditTime: 2022-07-08 06:45:56
 * @Description: 
 */

pub mod schema;
pub mod models;
pub mod db_service;
mod test_batch;

pub use tracing::*;

#[macro_use]
extern crate diesel;



use diesel::{r2d2::{ConnectionManager, PooledConnection, HandleError, HandleEvent, event::TimeoutEvent}};
use dotenv::dotenv;
use std::{env, error};

use diesel::prelude::*;
use diesel::r2d2::Pool;

pub type MysqlPool = ConnectionManager<MysqlConnection>;
pub type MysqlPooledConnection = PooledConnection<MysqlPool>;

// 全局变量用lazy_static实现
lazy_static::lazy_static! {
    static ref POOL:Pool<MysqlPool> = {
       init_pool()
    };
}

/// 普通连接
pub fn establish_connection() -> MysqlConnection {

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
        MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

// 连接池获得连接
pub fn get_connection() ->MysqlPooledConnection{
    POOL.get().unwrap()
    
}

// 初始化连接池
pub fn init_pool() -> Pool<MysqlPool> {
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    
    // 创建一个新的连接池
    let manager = ConnectionManager::<MysqlConnection>::new(url);
    let pool = Pool::builder()
        //  .max_size(100)
        .min_idle(Some(1))
        .test_on_check_out(true)
        .error_handler(Box::new(LoggingHandler{}))
        .event_handler(Box::new(LoggingHandler{}))

        // .thread_pool(thread_pool)
        .build(manager).expect("Failed to create pool");
    tracing::debug!("init_pool {:?}", pool.state());
    
    pool

}

#[derive(Copy, Clone, Debug)]
pub struct LoggingHandler;
// 输出错误日志
impl<E> HandleError<E> for LoggingHandler
where
    E: error::Error,
{
    fn handle_error(&self, error: E) {
        tracing::error!("{}", error);
    }
}
// 输出事件日志,只实现了timeout事件
impl HandleEvent for LoggingHandler{
    fn handle_timeout(&self, event: TimeoutEvent) {
        tracing::debug!("{:?}", event);
    }
}


// tests
#[cfg(test)]
mod tests {
    use diesel::connection::SimpleConnection;

    use super::*;

    // test init_pool
    #[test]
    fn test_init_pool() {
        let pool = init_pool();

        println!("{:?}", pool.state());
        //测试一下连接是否成功
        let mut con = pool.get().unwrap();
        let test_con = con.batch_execute("SELECT 1");
        println!("{:?}", test_con);
        let test_con = diesel::sql_query("SELECT 1").execute(&mut con).unwrap();
        println!("{:?}", test_con);
    }
}