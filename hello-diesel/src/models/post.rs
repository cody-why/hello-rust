/*** 
 * @Author: plucky
 * @Date: 2022-07-08 00:10:00
 * @LastEditTime: 2022-07-08 00:18:00
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
#[table_name="posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

#[derive(QueryableByName, Debug)]
#[table_name = "short_links"]
pub struct ShortLinks {
    pub id: u32,
    pub url: String,
}