/*** 
 * @Author: plucky
 * @Date: 2022-07-05 21:46:32
 * @LastEditTime: 2022-07-06 18:39:02
 * @Description: 
 */


use super::schema::posts;

#[derive(Queryable)]
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


