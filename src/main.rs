#[macro_use]
extern crate diesel;
use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
pub mod schema;
pub mod models;
pub mod dao;
use models::NewPost;
use dao::*;
use std::io;


fn main() {

    let database_url = "sample.db";
    
    let connection = SqliteConnection::establish(database_url).expect("SQLite connection error");
    let post_dao = PostDAO {
        connection: &connection
    };

   
    loop {
        println!("操作を選択 1: 作成, 2: 検索, 3: 全表示, 4: 終了");

        let mut action = String::new();
        io::stdin().read_line(&mut action)
            .expect("選択失敗");
        let action_number: usize = action.trim().parse().expect("数値を入力してください");

        match action_number {
            1 => {
                println!("タイトルを入力:");
                let mut title = String::new();
                io::stdin().read_line(&mut title)
                    .expect("タイトルの読み込みに失敗しました");
                let title = &title[..(title.len() - 1)];
            
                println!("本文を入力:");
                let mut body = String::new();
                io::stdin().read_line(&mut body)
                    .expect("本文の読み込みに失敗しました");
                let body = &body[..(body.len() - 1)];
            
                    
                let result = post_dao.create(NewPost{
                    title: title,
                    body: body
                });
                println!("result id:{}", result);
            },
            2 => {
                println!("検索番号を入力");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("error");
                
                let result = post_dao.find_one(id.trim().parse().expect("error"));
                println!("id:{}, titile:{}, body:{}", result.id, result.title, result.body);
            },  
            3 => {
                print_posts(&post_dao);
            },
            4 => {
                break;
            },
            _ => {
                println!("無効な入力です");
            }
        }
    }
 



    //let created = post_dao.findOne(result)
}


fn print_posts(post_dao: &PostDAO) {
    //use schema::posts::dsl::*;
    let results = post_dao.find_all();

    for post in results {
        println!("id:{}, title:{}, body:{}", post.id, post.title, post.body);
    }
}