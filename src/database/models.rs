use super::schema;
use chrono::NaiveDateTime;
use diesel::{sqlite::Sqlite, Insertable, Queryable, Selectable};

#[allow(dead_code)]
#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = schema::notes)]
#[diesel(check_for_backend(Sqlite))]
pub struct Note {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = schema::notes)]
pub struct NewNote<'creation> {
    pub title: &'creation str,
    pub body: &'creation str,
    pub updated_at: NaiveDateTime,
}
