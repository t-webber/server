use crate::schema;
use diesel::{sqlite::Sqlite, Insertable, Queryable, Selectable};

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = schema::posts)]
#[diesel(check_for_backend(Sqlite))]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable)]
#[diesel(table_name = schema::posts)]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = schema::users)]
#[diesel(check_for_backend(Sqlite))]
pub struct User {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
}

#[derive(Insertable)]
#[diesel(table_name = schema::users)]
pub struct NewUser<'a> {
    pub firstname: &'a str,
    pub lastname: &'a str,
    pub email: &'a str,
}
