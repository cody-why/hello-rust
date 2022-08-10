/*** 
 * @Author: plucky
 * @Date: 2022-07-11 13:37:38
 * @LastEditTime: 2022-07-14 11:44:28
 * @Description: 
 */

 #![allow(dead_code)]



use my_macro::*;


fn main() {
    

    //调用过程宏
    Test::hello();
    //调用属性宏
    dummy();

    //this_will_be_destroyed(); //因为macro改变了这个函数,没有了

    some_fn();

}


// hello macro, 自动实现 `Hello` trait
#[allow(unused)]
#[derive(Hello)]
struct Test{ 
    name: String,
}

#[allow(dead_code)]
#[hello_attribute(123)]
fn some_fn() {
    
}

// 这个属性宏,代码会生成宏里的dummy函数,这个函数就没有了,调用失败
#[log_entry_and_exit(hello, "world")]
fn this_will_be_destroyed() -> i32 {
    42
}

#[hello_attribute_base(123)]
fn some_fn2() {
    
}
