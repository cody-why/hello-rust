/*** 
 * @Author: plucky
 * @Date: 2022-07-08 06:54:29
 * @LastEditTime: 2022-07-09 16:44:41
 * @Description: 
 */

mod entities;
pub mod db_post;
pub mod config;





pub use entities::*;
pub use entities::prelude::*;


use once_cell::sync::{ OnceCell};
pub use tracing::{info, debug, warn, error,instrument};

pub use sea_orm::entity::prelude::*;


pub static SQLPOOL: OnceCell<DatabaseConnection> = OnceCell::new();





