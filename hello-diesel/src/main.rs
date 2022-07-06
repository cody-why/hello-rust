/*** 
 * @Author: plucky
 * @Date: 2022-07-05 21:46:32
 * @LastEditTim&e: 2022-07-06 00:00:49
 * @Description: 
 */



use time::UtcOffset;
use tracing_subscriber::fmt::time::OffsetTime;
use std::{env, str::FromStr};

use hello_diesel::db_service::*;

fn main() {
    init_log();
    dotenv::from_path(".env").ok();
    tracing::debug!("hello_diesel {:?}",env::var("DATABASE_URL"));

    query_by_sql();

    query_posts();
    println!("***");
    //insert_post();
    //query_posts();
    //println!("***");
    update_post();
    //query_posts();
    println!("***");
    delete_post();
    // query_posts();
    paginate_posts();
}

fn init_log(){
    // 设置输出时间为utc+8:00
    let local_time = OffsetTime::new(
        UtcOffset::from_hms(8, 0, 0).unwrap(),
        time::format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]").unwrap(),

    );
    
    tracing_subscriber::fmt()
    .with_max_level(tracing::Level::from_str("debug").unwrap())
    .with_target(false)
    .with_timer(local_time)
    .with_line_number(true)
    .with_file(true)
    .init();
}
