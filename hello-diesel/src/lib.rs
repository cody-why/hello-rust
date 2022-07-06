/*** 
 * @Author: plucky
 * @Date: 2022-07-05 19:00:17
 * @LastEditTime: 2022-07-07 00:24:45
 * @Description: 
 */

pub mod schema;
pub mod models;
pub mod db_service;

 #[macro_use]
extern crate diesel;


use diesel::{r2d2::{ConnectionManager, PooledConnection}};
use dotenv::dotenv;
use std::env;

pub use  diesel::{prelude::*, r2d2::Pool};

pub type MysqlPool = ConnectionManager<diesel::MysqlConnection>;
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
pub fn get_connection() -> MysqlPooledConnection {
    let pool =
    POOL.get().unwrap();
    pool
}

// 初始化连接池
pub fn init_pool() -> Pool<MysqlPool> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // 创建一个新的连接池
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    let pool = Pool::builder()
        .max_size(10)
        .min_idle(Some(1))
        .test_on_check_out(true)
        .build(manager).expect("Failed to create pool");
    pool

}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    // test init_pool
    #[test]
    fn test_init_pool() {
        let pool = init_pool();

        println!("{:?}", pool.state());
        //测试一下连接是否成功
        let pooled_connection = pool.get().unwrap();
        let test_connection = pooled_connection.execute("SELECT 1");
        println!("{:?}", test_connection);
    }
}