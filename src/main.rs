#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::style,
    clippy::perf,
    clippy::complexity,
    clippy::correctness,
    clippy::restriction,
    clippy::nursery,
    // clippy::cargo
)]
#![feature(stmt_expr_attributes)]
#![allow(clippy::blanket_clippy_restriction_lints)]
#![allow(clippy::implicit_return)]
#![allow(clippy::single_call_fn)]
#![allow(clippy::string_add)]
#![allow(clippy::missing_docs_in_private_items)]
#![allow(clippy::question_mark_used)]
#![allow(clippy::mod_module_files)]
#![allow(clippy::exhaustive_structs)]

use actix_web::{delete, get, post, web, App, HttpResponse, HttpServer, Responder};
use database::functions::{
    create_note, delete_note, edit_body, edit_title, establish_connection, get_notes,
};
use diesel::SqliteConnection;
use dotenvy::dotenv;
use errors::Res;
use load_env::get_env_var;
use serde::Deserialize;
use std::io;

mod database;
mod errors;
mod load_env;

fn connect_db() -> Result<SqliteConnection, HttpResponse> {
    match establish_connection() {
        Ok(conn) => Ok(conn),
        Err(err) => {
            Err(HttpResponse::InternalServerError().body(format!("Failed to fetch notes: {err}")))
        }
    }
}

#[get("/get-notes")]
async fn route_get_notes() -> impl Responder {
    match connect_db() {
        Ok(mut conn) => match get_notes(&mut conn) {
            Ok(notes) => HttpResponse::Ok().body(format!("{notes:?}")),
            Err(err) => HttpResponse::BadRequest().body(format!("Failed to fetch notes: {err}")),
        },
        Err(res) => res,
    }
}

#[derive(Deserialize)]
struct Edit {
    id: i32,
    content: String,
}

#[post("/edit-title")]
async fn route_edit_title(req_body: web::Json<Edit>) -> HttpResponse {
    match connect_db() {
        Ok(mut conn) => match edit_title(&mut conn, req_body.id, &req_body.content) {
            Ok(()) => HttpResponse::Ok().finish(),
            Err(err) => HttpResponse::BadRequest().body(format!("Failed to fetch notes: {err}")),
        },
        Err(err) => err,
    }
}

#[post("/edit-body")]
async fn route_edit_body(req_body: web::Json<Edit>) -> HttpResponse {
    match connect_db() {
        Ok(mut conn) => match edit_body(&mut conn, req_body.id, &req_body.content) {
            Ok(()) => HttpResponse::Ok().finish(),
            Err(err) => HttpResponse::BadRequest().body(format!("Failed to edit body: {err}")),
        },
        Err(err) => err,
    }
}

#[derive(Deserialize)]
struct NoteId {
    id: i32,
}

#[delete("/delete-notes")]
async fn route_delete_note(req_body: web::Path<NoteId>) -> HttpResponse {
    match connect_db() {
        Ok(mut conn) => match delete_note(&mut conn, req_body.id) {
            Ok(()) => HttpResponse::Ok().finish(),
            Err(err) => HttpResponse::BadRequest().body(format!("Failed to delete note: {err}")),
        },
        Err(err) => err,
    }
}

#[post("/create-note")]
async fn route_create_notes(title: String) -> HttpResponse {
    match connect_db() {
        Ok(mut conn) => match create_note(&mut conn, &title) {
            Ok(id) => HttpResponse::Ok().body(id.to_string()),
            Err(err) => HttpResponse::BadRequest().body(format!("Failed to create note: {err}")),
        },
        Err(err) => err,
    }
}

fn get_addrs() -> Res<(String, u16)> {
    dotenv().ok();
    let port = get_env_var("PORT")?;
    let host = get_env_var("HOST")?;
    Ok((
        host,
        port.parse()
            .map_err(|err| format!("Invalid PORT variable. Couldn't convert to integer: {err}"))?,
    ))
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    #[allow(clippy::print_stderr)]
    let addrs = get_addrs().unwrap_or_else(|err| {
        eprintln!(
            "Please specify a correct PORT or HOST variable in your `.env.local` file. An error occured while fetching these variables: {err}"
        );
        ("127.0.0.1".to_owned(), 8080)
    });
    #[allow(clippy::print_stdout)]
    {
        println!("Server running on http://{}:{}", addrs.0, addrs.1);
    };
    HttpServer::new(|| {
        App::new()
            .service(route_edit_body)
            .service(route_edit_title)
            .service(route_get_notes)
            .service(route_delete_note)
            .service(route_create_notes)
    })
    .bind(addrs)?
    .run()
    .await
}
