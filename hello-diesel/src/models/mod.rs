/*** 
 * @Author: plucky
 * @Date: 2022-07-08 00:09:31
 * @LastEditTime: 2022-07-08 00:22:27
 * @Description: 
 */

mod prelude {
    pub use diesel::{deserialize::Queryable,backend::Backend};
    pub use crate::schema::*;
}

pub use post::*;


mod post;
mod location;