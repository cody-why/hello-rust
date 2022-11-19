/*
 * @Author: plucky
 * @Date: 2022-09-04 12:04:31
 * @LastEditTime: 2022-11-18 19:14:58
 * @Description: 
 */


use once_cell::sync::OnceCell;
use crate::config::state::State;

pub mod model;


static STATE: OnceCell<State> = OnceCell::new();

pub fn set_state(state: State) {
    STATE.set(state).unwrap();
}

pub fn get_pool() -> &'static sqlx::Pool<sqlx::MySql> {
    let pool = &STATE.get().unwrap().sql_pool;
    pool
}

pub async fn get_conn() -> sqlx::MySqlConnection {
    use sqlx::Connection;
    let url = "mysql://root:newpassword@192.168.1.199:3306/hello";
    sqlx::MySqlConnection::connect(url).await.unwrap()
}