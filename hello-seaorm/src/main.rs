/*** 
 * @Author: plucky
 * @Date: 2022-07-08 06:50:02
 * @LastEditTime: 2022-07-10 08:09:34
 * @Description: 
 */

// use futures::executor::block_on;
use std::env;


use hello_seaorm::*;
use hello_seaorm::db_post::*;



#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "debug");
    config::init_log();

    tracing::info3!("Hello, world! {}","sea-orm");
 
   test_trace("jack".into());
    // std::thread::sleep(std::time::Duration::from_secs(1));
    run().await.unwrap();
}


#[instrument(fields(file = format!("{}:{}", file!(),line!())))]
async fn run() -> Result<(), DbErr> {
    SQLPOOL.set(config::init_mysql_pool().await).unwrap();

    updat_db().await?;
    // query_db().await?;
    
    query_db_page().await?;
    Ok(())
}

#[instrument(name = "my_span",fields(name = name,ile = format!("{}:{}", file!(),line!())))]
fn test_trace(name:String){
    let yaks =1usize;
    info!(yaks, "hello! I'm gonna shave a yak.");
}