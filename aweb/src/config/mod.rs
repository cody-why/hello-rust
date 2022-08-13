/***
 * @Author: plucky
 * @Date: 2022-08-10 19:49:39
 * @LastEditTime: 2022-08-13 17:11:54
 * @Description: 
 */


use std::{fmt::Debug, str::FromStr};

use serde::Deserialize;
use time::{UtcOffset};

use tracing::Level;
use tracing_subscriber::{ fmt::{time::OffsetTime}};

//filter::EnvFilter, 
pub mod routes;




const CONFIGFILE: &str = "app.yaml";

pub fn load_config() -> Config {
    let path = std::env::current_exe().unwrap().parent().unwrap().join("");
    println!("{:?}", path);
    // std::env::set_current_dir(path).unwrap();
    // let yaml_str = read_to_string(CONFIGFILE);
    serde_any::from_file::<Config,_>(CONFIGFILE).unwrap_or_default()

}

pub fn init_log(config: &LogConfig) {
    //println!("{}",cfg!(unsound_local_offset));
    // 设置输出时间为utc+8:00
    let local_time = OffsetTime::new(
        UtcOffset::from_hms(8, 0, 0).unwrap(),
        time::format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]").unwrap(),

    );
    
    tracing_subscriber::fmt()
    .with_max_level(Level::from_str(&config.level).unwrap())
    .with_writer(std::io::stdout)
    // .with_writer(non_blocking)
    .with_target(true)
    .with_timer(local_time)
    .with_line_number(true)
    .with_file(false)
    .init();
    
}

#[derive(Debug, Deserialize, Default)]
pub struct Config {
    pub server: ServerConfig,
    pub log: LogConfig,
    pub service: Vec<RouteConfig>,
}

// write with

#[derive(Debug, Deserialize)]
pub struct ServerConfig{
    pub port: u16,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            port: 3000,
        }
    }
}



#[derive(Debug, Deserialize)]
pub struct LogConfig{
    pub level: String,
    // tofile: bool,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct RouteConfig{
    pub path: String,
    pub dir: String,
}