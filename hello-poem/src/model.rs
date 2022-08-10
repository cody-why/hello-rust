/*** 
 * @Author: plucky
 * @Date: 2022-07-10 14:06:55
 * @LastEditTime: 2022-07-11 15:24:46
 * @Description: 
 */

use poem_openapi::Object;
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize, Object)]
pub struct User {
    pub username: String,
}