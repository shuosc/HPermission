#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

#[global_allocator]
static ALLOC: snmalloc_rs::SnMalloc = snmalloc_rs::SnMalloc;

use actix_web::{HttpServer, App, web};
use crate::handler::{ping, check};
use crate::config::CONFIG;

mod config;
mod handler;
mod token;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .route("/ping", web::get().to(ping))
            .default_service(web::route().to(check))
    }).bind("0.0.0.0:80")?.run().await
}
