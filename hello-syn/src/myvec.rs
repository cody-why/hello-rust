/*** 
 * @Author: plucky
 * @Date: 2022-07-11 16:45:02
 * @LastEditTime: 2022-07-14 11:58:09
 * @Description: 
 */


/// 这是一个声明宏,模拟vec!创建一个Vec,可变参数
/// vec![1,2,3] or vec![0;10]
#[macro_export]
macro_rules! my_vec {
    // 处理 my_vec![1, 2, 3]
    ($($el:expr),*) => ({
        let mut v = std::vec::Vec::new();
        $(v.push($el);)*
        v
    });
    // 处理 my_vec![0; 10]
    ($el:expr; $n:expr) => {
        std::vec::from_elem($el, $n)
    }
}
