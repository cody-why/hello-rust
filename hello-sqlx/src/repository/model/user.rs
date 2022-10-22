/*
 * @Author: plucky
 * @Date: 2022-09-04 19:14:14
 * @LastEditTime: 2022-10-22 22:46:15
 * @Description: 
 */


use crate::repository::get_conn;

// #[derive(sqlx::FromRow, )]
#[derive(Debug, co_orm::Crud,co_orm::FromRow)]
#[orm_rename = "users"]
pub struct User { 
    pub id: u64,
    #[orm_by]
    #[orm_update]
    pub name: String,
    #[orm_ignore]
    pub age: u8,
}



pub async fn get_by_name(name: &str) -> Result<User, sqlx::Error> {
    let conn = get_conn();
    sqlx::query_as::<_, User>(
        r#"SELECT * FROM users WHERE name = ?"#)
        .bind(name)
        .fetch_one(conn)
        .await
}


