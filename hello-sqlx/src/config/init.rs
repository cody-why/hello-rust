/***
 * @Author: plucky
 * @Date: 2022-09-04 19:58:56
 * @LastEditTime: 2022-09-06 16:29:09
 * @Description: 
 */

use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{fmt::{time::OffsetTime, self}, EnvFilter, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

use super::{*, database::*};

const CONFIGFILE: &str = "app.yaml";

pub fn load_config() -> Config {
    let path = std::env::current_exe().unwrap().parent().unwrap().join("");
    println!("{:?}", path);
    // rust if not debug
    #[cfg(not(debug_assertions))]
    std::env::set_current_dir(path).unwrap();
    
    serde_any::from_file::<Config,_>(CONFIGFILE).unwrap()

}

pub async fn init_state(config: &Config)->state::State{
    let mysql_pool = init_mysql_pool(&config.sql).await;
    state::State::new(mysql_pool)
   
}

pub fn init_log(config: &LogConfig)-> Option<WorkerGuard> {
    // 设置输出时间为utc+8:00
    let local_time = OffsetTime::new(
        time::UtcOffset::from_hms(8, 0, 0).unwrap(),
        time::format_description::parse("[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]").unwrap(),//[year]-
    );
    // 日志级别
    let env_filter = EnvFilter::new(&config.level);

    // 输出到控制台中
    let stdout_layer = fmt::layer().with_timer(local_time.clone())
    .pretty().with_writer(std::io::stderr);
    
    
    if config.tofile{
        // 输出到文件中
        let file_appender = tracing_appender::rolling::hourly("logs", "app.log");
        // non_blocking 在非阻塞的线程中输出日志。需要返回WorkerGuard，保证不会被销毁。
        let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
        
        let file_layer = fmt::layer().with_timer(local_time)
        .with_line_number(true)
        .with_ansi(false)
        .with_writer(non_blocking);
        // 注册
        tracing_subscriber::Registry::default()
        .with(stdout_layer)
        .with(file_layer)
        .with(env_filter)
        .init();
        
        return Some(_guard);
    } 
    
     // 注册
     tracing_subscriber::Registry::default()
     .with(stdout_layer)
     .with(env_filter)
     .init();
    
    None

    // let sub = tracing_subscriber::fmt()
    // .with_max_level(Level::from_str(&config.level).unwrap())
    // .with_target(false)
    // .with_timer(local_time)
    // .with_line_number(true)
    // .with_file(true)
    //.init();
    
}
