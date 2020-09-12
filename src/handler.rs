use actix_web::{HttpRequest, Responder, HttpResponse};
use crate::token::get_username;
use crate::CONFIG;
use actix_http::http::Method;

pub async fn ping(_: HttpRequest) -> impl Responder {
    "pong"
}

pub async fn check(request: HttpRequest) -> impl Responder {
    let username = request.headers().get("Authorization")
        .and_then(|it| it.to_str().ok())
        .map(|it| &it["Bearer ".len()..])
        .and_then(get_username);
    let write = match request.method() {
        &Method::POST | &Method::PUT | &Method::DELETE => true,
        &Method::GET => false,
        _ => unimplemented!()
    };
    let passed = CONFIG.check(username.as_ref(), request.path(), request.query_string(), write);
    if passed {
        HttpResponse::Ok()
    } else {
        HttpResponse::Unauthorized()
    }
}