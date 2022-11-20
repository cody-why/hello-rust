/*
 * @Author: plucky
 * @Date: 2022-11-20 10:45:48
 * @LastEditTime: 2022-11-20 12:53:01
 * @Description: 
 */

#[cfg(test)]
mod tests {
    use crate::{get_connection, models::user::User};
    // use crate::models::user::*;
    use diesel::{prelude::*};


    // 百万数据,使用批量vec插入,时间很久
    // 用sql,42s
    #[tokio::test]
    async fn test_insert_batch_sql(){
        use crate::schema::users::dsl::*;
        let connection =&mut get_connection();
        let del = diesel::delete(users).execute(connection).unwrap();
        println!("del: {:?}", del);

        let time = std::time::Instant::now();
        let (tx, mut rx) = tokio::sync::mpsc::channel(100);
        // 用tuple插入
        //let new_user = (name.eq("vec1"), age.eq(18));

        for i in 0..1{
            let total = 2000;
            
            // sql 插入
            let mut sql = String::from("insert into users(name, age) values");
            for j in 0..total{
                sql.push_str(&format!("('{}', {}),",format!("diesel{}",i*total+j), 18));
            }
            sql.pop();

            // 使用 vec 插入
            // let mut new_users = Vec::with_capacity(total);
            // for j in 0..total{
            //     new_users.push((name.eq(format!("diesel{}", i*total+j)), age.eq(18)));
            // }         
            
            let tx = tx.clone();
            tokio::spawn(async move {
                let connection =&mut get_connection();
                // let _inserted = diesel::insert_into(users).values(&new_users).execute(connection);
                diesel::sql_query(sql).execute(connection).unwrap();
                tx.send(1).await.unwrap();
            });
        }

        drop(tx);
        let mut count = 0;
        while let Some(_) = rx.recv().await {
            count += 1;
            
        }
        println!("job count: {}", count);
        
        println!("insert_batch = {:?}", time.elapsed());
        let res = users.count().get_result::<i64>(connection).unwrap();
        println!("select_all = {:?}", res);

        let u=  users.filter(name.eq("diesel98")).first::<User>(connection).unwrap();
        println!("u = {:?}", u);
    }
}