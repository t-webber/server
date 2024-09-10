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

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use database::functions::{establish_connection, get_notes};
use std::io;

mod database;

#[get("/")]
async fn hello() -> impl Responder {
    match establish_connection() {
        Ok(mut conn) => match get_notes(&mut conn) {
            Ok(notes) => HttpResponse::Ok().body(format!("{notes:?}")),
            Err(err) => HttpResponse::BadRequest().body(format!("Failed to fetch notes: {err}")),
        },
        Err(err) => HttpResponse::InternalServerError()
            .body(format!("Failed to connect to database: {err}")),
    }
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
