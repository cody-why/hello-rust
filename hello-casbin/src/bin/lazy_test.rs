/*** 
 * @Author: plucky
 * @Date: 2022-07-06 10:05:04
 * @LastEditTime: 2022-07-06 10:13:53
 * @Description: 
 */

use std::collections::HashMap;

use once_cell::sync::Lazy;
#[macro_use]
extern crate lazy_static;

// 全局变量用lazy_static实现
lazy_static::lazy_static! {
    static ref HASHMAP: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();
        m.insert(0, "foo");
        m.insert(1, "bar");
        m.insert(2, "baz");
        m
    };
}
// 全局变量用once_cell实现
pub static HASHMAP2: Lazy<HashMap<u32, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert(0, "foo");
    m.insert(1, "bar");
    m.insert(2, "baz");
    m
});

fn main(){
    println!("Hello, world!");
    // First access to `HASHMAP` initializes it
    println!("The entry for `0` is \"{}\".", HASHMAP.get(&0).unwrap());
    println!("The entry for `0` is \"{}\".", HASHMAP2.get(&0).unwrap());
}