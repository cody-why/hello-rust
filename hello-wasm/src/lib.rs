/*
 * @Author: plucky
 * @Date: 2022-09-19 10:15:45
 * @LastEditTime: 2022-09-19 20:17:29
 * @Description: 
 */

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

/// 可多参数的console.log
#[allow(unused)]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

// 浏览器函数
#[wasm_bindgen]
extern "C" {
    // console.log
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    // window.alert
    fn alert(s: &str);
}

// window.alert
#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("{}", name));
}

// 普通实现斐波那契数列
#[wasm_bindgen]
pub fn fib(i: u32) -> u32 {
    // match i {
    //     0 => 0,
    //     1 => 1,
    //     _ => fib(i - 1) + fib(i - 2)
    // }
    fib_optimized(i , 0, 1)
    
}

// 尾递归优化的实现斐波那契数列 
// #[wasm_bindgen]
#[inline]
pub fn fib_optimized(i: u32, prev: u32, next: u32) -> u32 {
    match i {
        0 => next,
        1 => next,
        _ => fib_optimized(i - 1, next, prev + next)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = fib(3);
        assert_eq!(result, 2);
        let result = fib_optimized(3,0,1);
        assert_eq!(result, 2);
    }
}
