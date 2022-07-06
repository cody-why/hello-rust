/*** 
 * @Author: plucky
 * @Date: 2022-07-05 21:46:32
 * @LastEditTime: 2022-07-07 00:48:10
 * @Description: 
 */

use super::schema::*;

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

