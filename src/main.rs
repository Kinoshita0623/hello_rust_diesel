#[macro_use]
extern crate diesel;
use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
pub mod schema;
use schema::posts;

#[derive(Queryable)]
struct Post {
    pub id: i32,
    pub title: String,
    pub body: String
}

#[derive(Insertable)]
#[table_name = "posts"]
struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

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
    use schema::posts::dsl::*;
    let results = posts
    .limit(5)
    .load::<Post>(connection)
    .expect("Error loading posts");

    for post in results {
        println!("id:{}, title:{}, body:{}", post.id, post.title, post.body);
    }
}