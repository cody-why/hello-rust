/*
 * @Author: plucky
 * @Date: 2022-07-10 10:25:05
 * @LastEditTime: 2022-11-20 11:46:58
 * @Description: 
 */

pub use tracing::*;


use time::UtcOffset;
use tracing_subscriber::fmt::time::OffsetTime;
use std::{env, str::FromStr, thread};

use hello_diesel::db_service::*;

fn main() {
    init_log();
    dotenv::from_path(".env").ok();
    debug!("hello_diesel {:?}",env::var("DATABASE_URL"));

    query_by_sql();

    // query_posts();
    println!("***");
    // insert_post();
    query_posts();
    //println!("***");
    update_post();
    //query_posts();
    println!("***");
    delete_post();
    // query_posts();
    paginate_posts();

    thread::sleep(std::time::Duration::new(20, 0));
}

fn init_log(){
    // 设置输出时间为utc+8:00
    let local_time = OffsetTime::new(
        UtcOffset::from_hms(8, 0, 0).unwrap(),
        time::format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]").unwrap(),

    );
    
    let level = "debug";
    tracing_subscriber::fmt()
    .with_max_level(tracing::Level::from_str(level).unwrap())
    .with_target(false)
    .with_timer(local_time)
    .with_line_number(true)
    .with_file(true)
    .init();
}
