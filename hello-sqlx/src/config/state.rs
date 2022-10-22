/*** 
 * @Author: plucky
 * @Date: 2022-09-04 20:49:16
 * @LastEditTime: 2022-09-04 23:12:03
 * @Description: 全局共享状态
 */

// use redis::aio::{ ConnectionManager};
use sqlx::{Pool, MySql};
//use tokio::sync::Mutex;


#[allow(dead_code)]
#[derive(Debug)]
pub struct State {
    // pub counter: std::sync::atomic::AtomicU64,
    pub sql_pool: Pool<MySql>,
    // pub redis_pool: ConnectionManager,
}

impl State {
    pub fn new(sql_pool: Pool<MySql>) -> Self {
        //let redis_conn = Mutex::new(redis_conn);
        State {
            // counter: std::sync::atomic::AtomicU64::new(0),
            sql_pool,
            // redis_pool,
        }
    }
}
    

