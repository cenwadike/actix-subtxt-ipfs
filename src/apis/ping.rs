use actix_web::{get, HttpResponse, Responder};

#[get("/")]
async fn ping() -> impl Responder {
    HttpResponse::Ok()
}
