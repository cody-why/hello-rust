use diesel::{debug_query, mysql::Mysql, sql_query};


use crate::{*, models::*};

// query_posts
pub fn query_posts() {
    use schema::posts::dsl::*;
    // let connection = establish_connection();
    let connection = get_connection();
    let results = posts.filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{:?}", post);
        println!("--------------------");
    }
}

// 泛型查询函数,适配所有struct

// pub fn query_for<T>(t:T)
// where T: Queryable<schema::posts::SqlType, Mysql> + std::fmt::Debug {
//     let connection = get_connection();
    
//     let results = t
//         // .limit(5)
//         .load::<T>(&connection)
//         .expect("Error loading posts");

//     println!("Displaying {} posts", results.len());
//     for post in results {
//         println!("{:?}", post);
//         println!("--------------------");
//     }
// }


//: Queryable<schema::posts::SqlType, Mysql>


pub fn query_by_sql() {
    use diesel::sql_types::*;
    
    let connection = get_connection();
    let results = connection.execute("SELECT * FROM posts WHERE published = 1 LIMIT 5").unwrap();
    println!("execute results {:?}", results);

    //  用sql_query查询
    let results = sql_query("SELECT * FROM short_links WHERE id > ?  limit 3")
        .bind::<Integer, _>(1)
        // .bind::<Text, _>("Tess")
        .get_results::<ShortLinks>(&connection);
        // 这里一直报错,返回类型不匹配,原来是ShortLinks的id类型不对,应该是u32,之前是i32,i64,u64,都不对
    println!("{:?}", results);
    println!("--------------------");
    
}

#[allow(dead_code)]
// insert_post
pub fn insert_post(){
    use schema::posts::dsl::*;
    // let connection = establish_connection();
    let connection = get_connection();

    // 用定义的struct插入
    let new_post = NewPost {
        title: "A new post",
        body: "This is a new post by struct",

    };
    let inserted = diesel::insert_into(posts)
        .values(&new_post)
        .execute(&connection);
        // .expect("Error saving new post");
    println!("Inserted: {:?}",inserted);
    
    // 用tuple插入
    //let new_post = (title.eq("tuple"),body.eq("This is insert by tuple"));
    // 批量插入
    let new_posts = vec![
        (title.eq("vec1"),body.eq("This is insert by vec")),
        (title.eq("vec2"),body.eq("This is insert by vec")),
    ];
    let inserted = diesel::insert_into(posts)
        .values(&new_posts)
        .execute(&connection);
        // .expect("Error saving new post");
    println!("Inserted: {:?}",inserted);


}

// update_post
pub fn update_post(){
    use schema::posts::dsl::*;
    let ids:i64 = 2;
    // let connection = establish_connection();
    let connection = get_connection();
    let updated = diesel::update(posts.find(ids))
        .set(title.eq("A new title"))
        .execute(&connection);
        //.get_result::<Post>(&connection)
        //.expect("Error updating post");
     println!("Updated {:?} posts", updated);

    // 更新published=false的文章,更新标题和已发布状态
    let target = posts.filter(published.eq(false));
    let _post = diesel::update(target)
        .set((published.eq(true), title.eq("A published title")))
        .execute(&connection);
        //.expect(&format!("Unable to find post {}", ids));
    println!("Updated posts: {:?}", _post);
}

// delete_post
pub fn delete_post(){
    use schema::posts::dsl::*;

    // let connection = establish_connection();
    let connection = get_connection();
    let query = diesel::delete(posts.find(1));
    
    let deleted_post = query.execute(&connection)
        .expect("Error deleting post");
    println!("Deleted {} posts", deleted_post);

    println!("Debug_query: {:?}", debug_query::<Mysql, _>(&query));
}

// 分页查询
pub fn paginate_posts() {
    let results = paginate(1, 3);

    println!("Total {} pages", get_total_pages(3));
    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.id);
        println!("{}", post.title);
        println!("{}", post.body);
        println!("--------------------");
    }

    
}

// Post分页, 参数是页码和每页的数量
pub fn paginate(page: i32, per_page: i32) -> Vec<Post> {
    use schema::posts::dsl::*;
    let connection = get_connection();
    let query = posts.filter(published.eq(true))
    .limit(per_page as i64)
    .offset(((page-1) * per_page) as i64);
    
    let results = query.load::<Post>(&connection)
    .expect("Error loading posts");

    println!("Debug_query: {:?}", debug_query::<Mysql, _>(&query));
    results
}

// Post分页有多少页,参数是每页的数量
pub fn get_total_pages(per_page: i32) -> i32 {
    use schema::posts::dsl::*;
    let connection = get_connection();
    let total_count = posts.filter(published.eq(true))
        .count()
        .get_result::<i64>(&connection)
        .unwrap_or_else(|e| {println!("Error loading posts {}",e);0});
    (total_count as i32 + per_page - 1) / per_page
}



// Post分页, 参数是页码和每页的数量,返回(页总数,每页的数据)
pub fn paginate_with_tuple(page: i32, per_page: i32) -> (i32, Vec<Post>) {
    use schema::posts::dsl::*;
    let connection = get_connection();
    
    let select = posts.filter(published.eq(true));
    // 查询分页数据
    let query = select
    .limit(per_page as i64)
    .offset(((page-1) * per_page) as i64);
    
    let results = query.load::<Post>(&connection)
    .unwrap_or_else(|e| {println!("Error loading posts {}",e);vec![]});

    //查询页总数
    let total_count = select
        .count()
        .get_result::<i64>(&connection)
        .unwrap_or_else(|e| {println!("Error loading posts {}",e);0});
    let pages =(total_count as i32 + per_page - 1) / per_page;
    println!("Debug_query: {:?}", debug_query::<Mysql, _>(&query));

    (pages, results)
}
