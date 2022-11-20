/*
 * @Author: plucky
 * @Date: 2022-07-10 10:25:05
 * @LastEditTime: 2022-11-20 12:02:24
 * @Description: 
 */


use super::prelude::*;

#[derive(Queryable, Debug)]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub body: String,
    pub published: bool,
}


#[derive(Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

#[derive(QueryableByName, Debug)]
#[diesel(table_name = short_links)]
pub struct ShortLinks {
    pub id: u32,
    pub url: String,
}