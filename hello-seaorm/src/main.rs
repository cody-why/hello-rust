/*** 
 * @Author: plucky
 * @Date: 2022-07-08 06:50:02
 * @LastEditTime: 2022-07-08 16:02:20
 * @Description: 
 */

use std::env;

use futures::executor::block_on;

use hello_seaorm::*;
use hello_seaorm::db_post::*;

fn main() {
    env::set_var("RUST_LOG", "debug");
    config::init_log();
    println!("Hello, world!");
    
    // std::thread::sleep(std::time::Duration::from_secs(1));
    block_on(run()).unwrap();
}

async fn run() -> Result<(), DbErr> {
    // config::init_mysql_pool().await;
    // updat_db().await?;
    POOL.set(config::init_mysql_pool().await).unwrap();
    query_db().await?;

    Ok(())
}