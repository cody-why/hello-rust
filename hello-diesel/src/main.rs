/*** 
 * @Author: plucky
 * @Date: 2022-07-05 21:46:32
 * @LastEditTim&e: 2022-07-06 00:00:49
 * @Description: 
 */


use diesel::{debug_query, mysql::Mysql};
use hello_diesel::{*, models::*};
use std::env;

fn main() {
    dotenv::from_path(".env").ok();
    println!("Hello, world! {:?}",env::var("DATABASE_URL"));

    query_posts();
    println!("***");
    //insert_post();
    //query_posts();
    //println!("***");
    update_post();
    //query_posts();
    println!("***");
    delete_post();
    // query_posts();
    paginate_posts();
}

// query_posts
fn query_posts() {
    use schema::posts::dsl::*;
    // let connection = establish_connection();
    let connection = get_connection();
    let results = posts.filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.id);
        println!("{}", post.title);
        println!("{}", post.body);
        println!("--------------------");
    }
}

#[allow(dead_code)]
// insert_post
fn insert_post(){
    use schema::posts::dsl::*;
    // let connection = establish_connection();
    let connection = get_connection();
    let new_post = NewPost {
        title: "A new post",
        body: "This is a new post",

    };
    let inserted = diesel::insert_into(posts)
        .values(&new_post)
        .execute(&connection);
        // .expect("Error saving new post");
    println!("Inserted: {:?}",inserted);

}

// update_post
fn update_post(){
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
fn delete_post(){
    use schema::posts::dsl::*;

    // let connection = establish_connection();
    let connection = get_connection();
    let query = diesel::delete(posts.find(1));
    let deleted_post = query.execute(&connection)
        .expect("Error deleting post");
    debug_query::<Mysql, _>(&query);
    println!("Deleted {} posts", deleted_post);
}

// 分页查询
fn paginate_posts() {
    let results = paginate(1, 10);

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

    debug_query::<Mysql, _>(&query);

    results
}