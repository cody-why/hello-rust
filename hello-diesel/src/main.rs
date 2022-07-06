/*** 
 * @Author: plucky
 * @Date: 2022-07-05 21:46:32
 * @LastEditTim&e: 2022-07-06 00:00:49
 * @Description: 
 */


use hello_diesel::{*, models::*};

fn main() {
    query_posts();
    println!("***");
    //insert_post();
    //query_posts();
    //println!("***");
    update_post();
    //query_posts();
    println!("***");
    delete_post();
    query_posts();
}

// query_posts
fn query_posts() {
    use hello_diesel::schema::posts::dsl::*;
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
    use hello_diesel::schema::posts::dsl::*;
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
    use hello_diesel::schema::posts::dsl::*;
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
    use hello_diesel::schema::posts::dsl::*;

    // let connection = establish_connection();
    let connection = get_connection();
    let deleted_post = diesel::delete(posts.find(1))
        .execute(&connection)
        .expect("Error deleting post");
    println!("Deleted {} posts", deleted_post);
}