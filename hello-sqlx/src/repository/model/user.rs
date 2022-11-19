/*
 * @Author: plucky
 * @Date: 2022-09-04 19:14:14
 * @LastEditTime: 2022-11-19 11:04:40
 * @Description: 
 */



use crate::repository::*;

// #[derive(sqlx::FromRow, )]
#[derive(Debug, co_orm::Crud, co_orm::FromRow)]
#[orm_rename = "users"]
pub struct User { 
    pub id: u64,
    #[orm_by]
    #[orm_update]
    pub name: String,
    pub age: u8,
    #[orm_ignore]
    pub adr: String,
}



pub async fn get_by_name(name: &str) -> Result<User, sqlx::Error> {
    let conn = get_pool();
    sqlx::query_as::<_, User>(
        r#"SELECT * FROM users WHERE name = ?"#)
        .bind(name)
        .fetch_one(conn)
        .await
}




