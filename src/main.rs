#[macro_use]
extern crate diesel;
use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
pub mod schema;
use schema::posts;
mod models;
use models::NewPost;
use models::Post;


fn main() {

    let database_url = "sample.db";
    
    let connection = SqliteConnection::establish(database_url).expect("SQLite connection error");
    let result = diesel::insert_into(posts::table)
        .values(NewPost{
            title: "hogehoge",
            body: "piyopiyo"
        })
        .execute(&connection)
        .expect("SQLite insert error");
    println!("result:{}", result);

    print_posts(&connection);
}


fn print_posts(connection: &SqliteConnection) {
    //use schema::posts::dsl::*;
    let results = posts::dsl::posts
    .load::<Post>(connection)
    .expect("Error loading posts");

    for post in results {
        println!("id:{}, title:{}, body:{}", post.id, post.title, post.body);
    }
}