/*
 * @Author: plucky
 * @Date: 2022-11-20 10:56:37
 * @LastEditTime: 2022-11-20 12:48:50
 * @Description: 
 */

use diesel::prelude::*;
// use crate::schema::*;

#[derive(Queryable, Debug)]
pub struct User{
    pub id: u64,
    pub name: String,
    pub age: u8,
    // pub password:Option<String>,
}

// #[derive(Insertable, Debug)]
// #[diesel(table_name = users)]
// pub struct NewUser<'a> {
//     pub name: &'a str,
//     pub age: u8,
// }