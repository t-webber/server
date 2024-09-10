#![allow(unused)]

use std::env;

use chrono::Utc;
use diesel::prelude::*;
use dotenvy::dotenv;

use super::errors::{Check, Res, ToCheck, ToR};
use super::models::{NewNote, Note};
use super::schema::notes::{self, dsl};

pub fn establish_connection() -> Res<SqliteConnection> {
    dotenv().ok();
    let database_url =
        env::var("DATABASE_URL").map_err(|err| format!("DATABASE_URL must be set: {err}"))?;
    SqliteConnection::establish(&database_url)
        .map_err(|err| format!("Error connecting to {database_url}: {err}"))
}

pub fn get_notes(conn: &mut SqliteConnection) -> Res<Vec<Note>> {
    dsl::notes
        // .limit(10)
        .select(Note::as_select())
        .load(conn)
        .to_r()
}

pub fn create_note(conn: &mut SqliteConnection, title: &str) -> Res<i32> {
    let new_note = NewNote {
        title,
        body: "",
        updated_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(notes::table)
        .values(&new_note)
        .returning(Note::as_returning())
        .get_result(conn)
        .to_r()
        .map(|note| note.id)
}

pub fn edit_body(conn: &mut SqliteConnection, note_id: i32, body: &str) -> Check {
    diesel::update(dsl::notes.find(note_id))
        .set((
            dsl::body.eq(body),
            dsl::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(conn)
        .to_check()
}

pub fn edit_title(conn: &mut SqliteConnection, note_id: i32, title: &str) -> Check {
    diesel::update(dsl::notes.find(note_id))
        .set((
            dsl::title.eq(title),
            dsl::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(conn)
        .to_check()
}

pub fn delete_note(conn: &mut SqliteConnection, note_id: i32) -> Check {
    diesel::delete(dsl::notes.find(note_id))
        .execute(conn)
        .to_check()
}
