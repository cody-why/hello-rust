/*** 
 * @Author: plucky
 * @Date: 2022-07-08 07:59:04
 * @LastEditTime: 2022-07-08 15:42:22
 * @Description: 
 */




use sea_orm::{ActiveValue, debug_query};

use crate::*;

pub async fn query_db() -> Result<(), DbErr> {
    use posts::Column::*;
    //let db = set_up_db().await?;
    let  db = POOL.get().unwrap();
    let s =Posts::find()
    .filter(Id.eq(3).and(Published.eq(true)));
    
    let sql = debug_query!(&s, db);
    debug!("{}", sql);

    let posts =s.all(db).await?;
    println!("{:?}", posts);

    Ok(())
}

pub async fn updat_db() -> Result<(), DbErr> {
    // use posts::Column::*;
    // let db = set_up_db().await?;
    let  db = POOL.get().unwrap();
    let post = posts::ActiveModel{
        id: ActiveValue::Set(3),
        title: ActiveValue::Set("hello".into()),
        ..Default::default()
    };
    let post = post.update(db).await.unwrap();
    
    println!("{:?}", post);
    
    
    Ok(())
}