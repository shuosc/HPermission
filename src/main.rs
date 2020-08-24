#[global_allocator]
static ALLOC: snmalloc_rs::SnMalloc = snmalloc_rs::SnMalloc;

use actix_web::{HttpServer, App, web};
use crate::db::DB;
use crate::handler::ping;

mod db;
mod model;
mod handler;

const DB_URL: &'static str = "postgres://testuser@localhost:5432/postgres";

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .data_factory(|| DB::connect(DB_URL))
            .route("/ping", web::get().to(ping))
    }).bind("0.0.0.0:8000")?.run().await
}
