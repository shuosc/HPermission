use actix_web::{HttpRequest, Responder};

pub async fn ping(_: HttpRequest) -> impl Responder {
    "pong"
}
