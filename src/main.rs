#[macro_use]
extern crate diesel;
use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
pub mod schema;
pub mod models;
pub mod dao;
use models::NewPost;
use dao::*;


fn main() {

    let database_url = "sample.db";
    
    let connection = SqliteConnection::establish(database_url).expect("SQLite connection error");
    let post_dao = PostDAO {
        connection: connection
    };

    let result = post_dao.create(NewPost{
        title: "hogehoge",
        body: "Piyopiyo"
    });
    println!("result id:{}", result);

    //let created = post_dao.findOne(result)
    print_posts(&post_dao);
}


fn print_posts(post_dao: &PostDAO) {
    //use schema::posts::dsl::*;
    let results = post_dao.find_all();

    for post in results {
        println!("id:{}, title:{}, body:{}", post.id, post.title, post.body);
    }
}