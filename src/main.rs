#[macro_use]
extern crate diesel;
use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
pub mod schema;
pub mod models;
pub mod dao;
use models::NewPost;
use dao::*;
pub mod repositories;
use repositories::*;
use std::io;


fn main() {

    let database_url = "sample.db";
    
    let connection = SqliteConnection::establish(database_url).expect("SQLite connection error");
    let post_dao = PostDAO {
        connection: &connection
    };

   
    show_post_console(post_dao);
}

fn show_post_console<T>(post_repository: T) where T: PostRepository{
    loop {
        println!("操作を選択 1: 作成, 2: 検索, 3: 全表示, 4: 削除 ,5: 終了");

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
            
                    
                let result = post_repository.create(NewPost{
                    title: title,
                    body: body
                });
                println!("result id:{}", result);
            },
            2 => {
                println!("検索番号を入力");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("error");
                
                let result = post_repository.find_one(id.trim().parse().expect("error"));
                println!("id:{}, titile:{}, body:{}", result.id, result.title, result.body);
            },  
            3 => {
                print_posts(&post_repository);
            },
            4 => {
                println!("削除番号を入力");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Input error");
                let result = post_repository.delete(id.trim().parse().expect("Input format error"));
                match result {
                    Ok(_) => println!("削除成功"),
                    Err(e) => println!("削除に失敗しました:{:?}", e),
                };

            },
            5 => {
                break;
            },
            _ => {
                println!("無効な入力です");
            }
        }
    }
}

fn print_posts<T>(post_dao: &T) where T: PostRepository{
    //use schema::posts::dsl::*;
    let results = post_dao.find_all();

    for post in results {
        println!("id:{}, title:{}, body:{}", post.id, post.title, post.body);
    }
}