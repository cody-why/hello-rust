/**
 * @Author: plucky
 * @Date: 2022-09-04 12:04:31
 * @LastEditTime: 2022-09-16 15:31:50
 * @Description: 
 */


use once_cell::sync::OnceCell;
use crate::config::state::State;

pub mod model;


static STATE: OnceCell<State> = OnceCell::new();

pub fn set_state(state: State) {
    STATE.set(state).unwrap();
}

pub fn get_conn() -> &'static sqlx::Pool<sqlx::MySql> {
    let pool = &STATE.get().unwrap().sql_pool;
    pool
}

// pub async fn get_conn() -> sqlx::MySqlConnection {
//     use sqlx::Connection;
//     let url = "mysql://root:123456@localhost:3306/shortlink";
//     sqlx::MySqlConnection::connect(url).await.unwrap()
// }