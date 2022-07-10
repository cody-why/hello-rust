/*** 
 * @Author: plucky
 * @Date: 2022-07-08 06:54:29
 * @LastEditTime: 2022-07-08 16:01:40
 * @Description: 
 */

mod entities;
pub mod db_post;
pub mod config;


pub use entities::*;
pub use entities::prelude::*;
use once_cell::sync::{ OnceCell};
pub use tracing::*;



pub use sea_orm::entity::prelude::*;



pub static POOL: OnceCell<DatabaseConnection> = OnceCell::new();


pub async fn set_up_db() -> Result<DatabaseConnection, DbErr> {
    let url = dotenv::var("DATABASE_URL").unwrap();
    let db = sea_orm::Database::connect(url).await?;

    Ok(db)
}


