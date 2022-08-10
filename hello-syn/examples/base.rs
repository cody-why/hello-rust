/*** 
 * @Author: plucky
 * @Date: 2022-07-14 00:49:04
 * @LastEditTime: 2022-07-14 11:58:58
 * @Description: 
 */

use my_macro::*;
use hello_syn::*;

fn main(){
    //调用声明宏
    // let v = my_vec!(1,2,3);
    let v: Vec<i32> = my_vec![];
    
    println!("自定义的vec: {:?}",v);
    
    // sql!(select * from user wher id = 1);

    some_fn();
}

#[hello_attribute_base(123)]
fn some_fn() {
    println!("hello");
}