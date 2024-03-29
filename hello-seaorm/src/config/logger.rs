/*** 
 * @Author: plucky
 * @Date: 2022-07-08 16:00:59
 * @LastEditTime: 2022-07-10 16:25:27
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
    //tracing_subscriber::fmt().with_timer(local_time).init();
    let level = "debug";
    tracing_subscriber::fmt()
    .with_max_level(tracing::Level::from_str(level).unwrap())
    .with_target(false)
    .with_timer(local_time)
    .with_file(true)
    .with_line_number(true)
    .init();
}