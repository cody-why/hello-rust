#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    #![allow(dead_code)]
    // use futures::{ StreamExt};
    use sqlx::{Row};
    use tokio::sync::mpsc;
    // use rayon::prelude::*;
    use crate::{config::init_state, repository::get_pool};


    #[tokio::test]
    async fn test_insert_many() {
        // let rt = tokio::runtime::Runtime::new().unwrap();
        // rt.block_on(async {
            
        // });
        init_state().await;
        delete_all().await.unwrap();
        let t = time::Instant::now();

        insert_many1().await.unwrap();

        query_count().await.unwrap();
        println!("time: {:?}", t.elapsed().as_seconds_f32());
    }

    // 批量插入1000000条数据 
    // 连接url加入参数：?rewriteBatchedStatements=true
    // 使用sql测试,2000条一次,时间为5.9s,1000/次,6.6s
    pub async fn insert_many1() -> Result<(), sqlx::Error> {
        // let conn = get_pool();
    
        // let mut tasks = Vec::with_capacity(1000);
        let (tx, mut rx) = mpsc::channel(100);

        // 
        for i in 0..500 {
            let total = 2000;
            let tx = tx.clone();
            let task = async move {
                let mut sql = "insert into users(name, age) values".to_string();
                for j in 0..total {
                    sql.push_str(&format!("('{}', {}),", i*total+j, 18));
                }
                sql.pop();

                let  conn = get_pool();
                sqlx::query(&sql).execute(conn).await.unwrap();
                tx.send(1).await.unwrap();
            };
            // tasks.push(sql);
            tokio::spawn(task);
        }
        

        // futures::future::join_all(tasks).await;
        drop(tx);
        let mut count = 0;
        while let Some(_) = rx.recv().await {
            count += 1;
        }
        println!("job count: {}", count);
       
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
        for _ in 0..100{
            let tx = tx.clone();
            let _task = tokio::spawn(async move {
                // let  conn = get_pool();
                for _j in 0..500 {
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
}


