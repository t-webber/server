mod functions;
mod models;
mod schema;

use functions::{create_note, edit_body, edit_title, establish_connection, get_notes};

fn main() {
    println!("INITIALISING");
    let conn = &mut establish_connection().unwrap();
    create_note(conn, "RA RA ").unwrap();
    create_note(conn, "OR RO").unwrap();
    edit_body(conn, 4, "Lorem ipsum sit amet!!").unwrap();
    edit_title(conn, 5, "Helo Word!!").unwrap();
    let notes = get_notes(conn).unwrap();
    dbg!(notes);
}
