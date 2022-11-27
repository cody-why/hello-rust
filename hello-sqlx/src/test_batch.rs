#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    #![allow(dead_code)]
    
    // use futures::{ StreamExt};
    use sqlx::{Row};
    use tokio::sync::mpsc;
    // use rayon::prelude::*;
    use crate::{config::init_state, repository::get_pool};


    // 批量插入1000000条数据 
    // 连接url加入参数：?rewriteBatchedStatements=true
    // 使用sql测试,2000条一次,时间为5.9s,1000/次,6.6s
    // async_std::task::spawn 比 tokio::spawn 差不多
    #[tokio::test]
    pub async fn insert_many1() -> Result<(), sqlx::Error> {
        init_state().await;
        delete_all().await.unwrap();
        let t = time::Instant::now();
    
        // let mut tasks = Vec::with_capacity(1000);
        let (tx, mut rx) = mpsc::channel(100);

        let total = 1000000;
        let pair = 2000;
        println!("pair:{}", total/pair);
        for i in 0..total/pair {
            let tx = tx.clone();
            let task = async move {
                let mut sql = "insert into users(name, age) values".to_string();
                for j in 0..pair {
                    sql.push_str(&format!("('{}', {}),", i*total+j, 18));
                }
                sql.pop();

                let conn = get_pool();
                sqlx::query(&sql).execute(conn).await.unwrap();
                tx.send(1).await.unwrap();
            };
            // tasks.push(sql);
            tokio::spawn(task);
            // async_std::task::spawn(task);
        }
        

        // futures::future::join_all(tasks).await;
        drop(tx);
        let mut count = 0;
        while let Some(_) = rx.recv().await {
            count += 1;
        }
        println!("job count: {}", count);
       
        query_count().await.unwrap();
        println!("time: {:?}", t.elapsed().as_seconds_f32());

        Ok(())
    }

    pub async fn delete_all() -> Result<(), sqlx::Error> {
        let conn = get_pool();
        sqlx::query("delete from users").execute(conn).await?;
        Ok(())
    }
    pub async fn query_count() -> Result<(), sqlx::Error> {
        let conn = get_pool();
        let n =  sqlx::query("select count(*) as n from users").fetch_one( conn).await?;
        let count: i64 = n.try_get("n").unwrap();
        println!("db count: {:?}", count);
        Ok(())
    }
    

    // 很慢
    pub async fn insert_many2() -> Result<(), sqlx::Error> {
        let conn = get_pool();
    
        // let mut tasks = Vec::with_capacity(1000);
        let (tx, mut rx) = mpsc::channel(100);
        
        use crate::repository::model::user::User;
        let total = 1000000;
        let pair = 2000;
        println!("pair:{}", total/pair);

        for _ in 0..total/pair{
            let tx = tx.clone();
            let _task = tokio::spawn(async move {
                // let  conn = get_pool();
                for _j in 0..pair {
                    let user  = User {
                        id: 0,
                        name: "test".to_string(),
                        age: 18,
                        adr: "t".to_string(),
                    };
                    user.insert(conn).await.unwrap();
                }
                tx.send(1).await.unwrap();
            });
        }
        // (0..500).into_par_iter().for_each(|i| {
        

        drop(tx);
        let mut count = 0;
        while let Some(_) = rx.recv().await {
            count += 1;
        }
        println!("job count: {}", count);
        

      
        Ok(())
    }

    // sqlx 需要 futures runtime-async-std-native-tls,runtime-tokio-native-tls用不了 async_std::task::spawn
    // async_std::task::spawn + rayon into_par_iter 百万,2000/次 6.1s
    #[tokio::test]
    pub async fn insert_many_iter() -> Result<(), sqlx::Error> {
        use rayon::prelude::IntoParallelIterator;
        use rayon::prelude::ParallelIterator;

        init_state().await;
        delete_all().await.unwrap();
        let t = time::Instant::now();

        
        // let mut tasks = Vec::with_capacity(1000);
        let (tx, mut rx) = mpsc::channel(100);
        let total = 1000000;
        let pair = 2000;
        println!("pair: {}", total/pair);
        (0..total/pair).into_par_iter().for_each(|i| {
            let mut sql = "insert into users(name, age) values".to_string();
            for j in 0..pair {
                sql.push_str(&format!("('{}', {}),", i*total+j, 18));
            }
            sql.pop();

            let tx = tx.clone();
            async_std::task::spawn(async move {
                let conn = get_pool();
                sqlx::query(&sql).execute(conn).await.unwrap();
                tx.send(1).await.unwrap();
            });
            
        });
        

        drop(tx);
        let mut count = 0;
        while let Some(_) = rx.recv().await {
            count += 1;
        }
        println!("job count: {}", count);
        

        query_count().await.unwrap();
        println!("time: {:?}", t.elapsed().as_seconds_f32());

      
        Ok(())
    }
}


