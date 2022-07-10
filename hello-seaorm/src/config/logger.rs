/*** 
 * @Author: plucky
 * @Date: 2022-07-08 16:00:59
 * @LastEditTime: 2022-07-09 22:59:16
 * @Description: 
 */
pub use sea_orm::{DbErr};
use std::str::FromStr;
use tracing_subscriber::fmt::time::OffsetTime;


pub fn init_log(){
    // 设置输出时间为utc+8:00
    let local_time = OffsetTime::new(
        time::UtcOffset::from_hms(8, 0, 0).unwrap(),
        time::format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]").unwrap(),
    );
    
    let level = "debug";
    tracing_subscriber::fmt()
    .with_max_level(tracing::Level::from_str(level).unwrap())
    .with_target(false)
    .with_timer(local_time)
    .with_file(true)
    .with_line_number(true)
    .init();
}