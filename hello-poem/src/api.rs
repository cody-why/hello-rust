/*** 
 * @Author: plucky
 * @Date: 2022-07-10 16:30:12
 * @LastEditTime: 2022-07-11 18:11:01
 * @Description: 
 */

use poem_openapi::{param::Query, payload::PlainText, OpenApi, payload::Json,};
use crate::model::User;

pub struct Api;

#[OpenApi]
impl Api {

    /// 您好
    /// 
    /// 您好,你的名字是: [name]
    /// 
    /// 换行了吗
    #[oai(path = "/hello", method = "get")]
    async fn index(&self, name: Query<Option<String>>) -> PlainText<String> {
        match name.0 {
            Some(name) => PlainText(format!("hello, {}!", name)),
            None => PlainText("hello!".to_string()),
        }
    }

 
    /// 登陆
    #[oai(path = "/login", method = "post")]
    async fn login(&self, user:Json<User>) -> PlainText<String> {
        tracing::info!(user.username);
        PlainText(format!("hello, {}!", user.username))
    }
}