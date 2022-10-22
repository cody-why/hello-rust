/***
 * @Author: plucky
 * @Date: 2022-09-04 16:10:31
 * @LastEditTime: 2022-09-04 23:33:51
 * @Description: 
 */
pub mod database;
pub mod state;
pub mod  init;

pub use state::*;
pub use init::*;

use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct Config {
   pub server: ServerConfig,
   pub sql: MysqlConfig,
   pub log: LogConfig,
    
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
pub struct MysqlConfig{
    url: String,
    max_connections: u32,
    min_connections: u32,
}


#[derive(Debug, Deserialize)]
pub struct LogConfig{
    level: String,
    tofile: bool,
}
impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            tofile: false,
        }
    }
}