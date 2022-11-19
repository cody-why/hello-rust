/*
 * @Author: plucky
 * @Date: 2022-07-10 10:25:05
 * @LastEditTime: 2022-11-19 23:22:50
 * @Description: 
 */

use super::prelude::*;
pub use chrono::{DateTime, Local, Utc};
use diesel::{sql_types::Text, deserialize::{FromSql, self}};


//  DateTime<Local> 指定序列化方法
#[derive( Queryable)]
pub struct Location {
    #[diesel(deserialize_as = MyDatetimeWrapper)]
    pub publication_time: DateTime<Local>,
    pub id: i32,
    pub name: String,
   
}

pub struct MyDatetimeWrapper(DateTime<Local>);

impl Into<DateTime<Local>> for MyDatetimeWrapper {
    fn into(self) -> DateTime<Local> {
        self.0
    }
}


impl<DB, ST> Queryable<ST, DB> for MyDatetimeWrapper
where
    DB: Backend,
    DateTime<Utc>: Queryable<ST, DB>,
{
    type Row = <DateTime<Utc> as Queryable<ST, DB>>::Row;

    fn build(row: Self::Row) -> deserialize::Result<Self> {
        // 带上时区信息
        Ok(Self(<DateTime<Utc> as Queryable<ST, DB>>::build(row)?.with_timezone(&Local)))
        
    }
}

// 2.0官方的例子
// https://docs.rs/diesel/2.0.2/diesel/prelude/trait.Queryable.html
struct LowercaseString(String);

impl Into<String> for LowercaseString {
    fn into(self) -> String {
        self.0
    }
}

impl<DB> Queryable<Text, DB> for LowercaseString
where
    DB: Backend,
    String: FromSql<Text, DB>,
{
    type Row = String;

    fn build(s: String) -> deserialize::Result<Self> {
        Ok(LowercaseString(s.to_lowercase()))
    }
}

#[derive(Queryable, PartialEq, Debug)]
struct User {
    id: i32,
    #[diesel(deserialize_as = LowercaseString)]
    name: String,
}
