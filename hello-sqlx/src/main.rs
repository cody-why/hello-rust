/*
 * @Author: plucky
 * @Date: 2022-09-04 00:49:25
 * @LastEditTime: 2022-11-18 20:07:27
 * @Description: 
 */

use hello_sqlx::{config::*, repository::{*, model::{user::User}}};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_state().await;
    

    // println!("users: {:?}", users);
    let pool = get_pool();
    let user = User{
        id: 0,
        name: "jack3".to_string(),
        age: 18,
        adr: "beijing".to_string(),
    };
    // user.insert(pool).await.unwrap();
        
    match user.update(pool).await{
        Ok(e) => println!("update success {:?}", e.rows_affected()),
        Err(e) => println!("update error: {}", e),
    }

    let users = User::query_by(pool,"").await.unwrap();
    dbg!("users: {:?}", users);

    match User::get(pool, 1).await{
        Ok(e) => println!("user: {:?}", e),
        Err(e) => println!("error: {}", e),
    }
   
    Ok(())
}
