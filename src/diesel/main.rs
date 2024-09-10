use diesel::prelude::*;
use dotenvy::dotenv;
use models::{NewUser, User};
use schema::users::{self, dsl};
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

fn show_users(conn: &mut SqliteConnection) {
    let results = dsl::users
        .limit(5)
        .select(User::as_select())
        .load(conn)
        .expect("Error loading users");

    dbg!(&results);

    println!("Displaying {} users", results.len());
    for user in results {
        println!("{user:?}\n-------------");
    }
}

pub fn create_user(conn: &mut SqliteConnection, email: &str) -> User {
    let mut split = email.split('@').next().unwrap().split('.');
    let new_user = NewUser {
        email,
        firstname: split.next().unwrap(),
        lastname: split.next().unwrap(),
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(conn)
        .expect("Error saving new user")
}

// pub fn publish_user(conn: &mut SqliteConnection, id: i32) {
//     let user = diesel::update(dsl::users.find(id))
//         .returning(User::as_returning())
//         .get_result(conn)
//         .unwrap();
//     println!("Published user {}", user.email);
// }

fn main() {
    let conn = &mut establish_connection();
    create_user(conn, "bobby.bob982E@gmail.com");
    // publish_user(conn, 5);
    show_users(conn);
}
