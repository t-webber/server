use diesel::prelude::*;
use dotenvy::dotenv;
use models::{NewPost, Post};
use schema::posts::{self, dsl, published};
use std::env;

pub mod models;
pub mod schema;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    dbg!(&database_url);
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn show_posts(conn: &mut SqliteConnection) {
    let results = dsl::posts
        .filter(published.eq(true))
        .limit(5)
        .select(Post::as_select())
        .load(conn)
        .expect("Error loading posts");

    dbg!(&results);

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.body);
    }
}

pub fn create_post(conn: &mut SqliteConnection, title: &str, body: &str) -> Post {
    let new_post = NewPost { title, body };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .returning(Post::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn publish_post(conn: &mut SqliteConnection, id: i32) {
    let post = diesel::update(dsl::posts.find(id))
        .set(published.eq(true))
        .returning(Post::as_returning())
        .get_result(conn)
        .unwrap();
    println!("Published post {}", post.title);
}

fn main() {
    let conn = &mut establish_connection();
    // create_post(conn, "Hello", "World");
    publish_post(conn, 5);
    show_posts(conn);
}
