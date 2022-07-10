/*** 
 * @Author: plucky
 * @Date: 2022-07-08 00:13:34
 * @LastEditTime: 2022-07-08 00:21:49
 * @Description: 
 */
use super::prelude::*;
pub use chrono::{DateTime, Local, Utc};


//  DateTime<Local> 指定序列化方法
#[derive( Queryable)]
pub struct Location {
    #[diesel(deserialize_as = "MyDatetimeWrapper")]
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

    fn build(row: Self::Row) -> Self {
        // 带上时区信息
        Self(<DateTime<Utc> as Queryable<ST, DB>>::build(row).with_timezone(&Local))
    }
}