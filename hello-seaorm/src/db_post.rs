/*
 * @Author: plucky
 * @Date: 2022-07-10 10:25:05
 * @LastEditTime: 2022-11-21 09:24:43
 * @Description: 
 */


use sea_orm::{ActiveValue, debug_query, QueryOrder};

use crate::*;


pub async fn query_db() -> Result<(), DbErr> {
    use posts::Column::*;
    //let db = set_up_db().await?;
    let  db = SQLPOOL.get().unwrap();
    let s =Posts::find()
    .filter(Id.eq(3).and(Published.eq(true)));
   
    //Posts::find_by_id(3)
    let sql = debug_query!(&s, db);
    debug!("{}", sql);
    
    let posts =s.all(db).await?;
    println!("{:?}", posts);

    Ok(())
}

#[instrument(fields(file = format!("{}:{}", file!(),line!() )))]
pub async fn updat_db() -> Result<(), DbErr> {
    // let db = set_up_db().await?;
    let  db = SQLPOOL.get().unwrap();
    let post = posts::ActiveModel{
        id: ActiveValue::Set(3),
        title: ActiveValue::Set("hello".into()),
        ..Default::default()
    };
    //post.update(db).await?;

    let s = Posts::update(post);

    // debug sql
    let sql = debug_query!(&s, db);
    debug!("{}", sql);

    let post = s.exec(db).await?;
    
    println!("{:?}", post);
      
    // Posts::Entity::update_many().filter(Title.eq("hello")).exec(db).await?;
    
    Ok(())
}

// 分页查询
pub async fn query_db_page() -> Result<(), DbErr> {
    let page = 1;
    let per_page = 5;
    
    let  db = SQLPOOL.get().unwrap();

    let paginator = Posts::find()
    .order_by_asc(posts::Column::Id)
    .paginate(db, per_page);
    //.limit(per_page);
    //.offset((page - 1) * per_page);
    
    let num_pages = paginator.num_pages().await.ok().unwrap();
    let posts = paginator
    .fetch_page(page - 1)
    .await
    .expect("could not retrieve posts");
   
    //debug!("page={},per_page={}", page, per_page);
    debug!("pages:{}, {:?}",num_pages, posts.len());
    Ok(())
}

