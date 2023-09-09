use actix_web::{get, HttpResponse, Responder};

#[get("/")]
async fn ping() -> impl Responder {
    log::info!("API: Handle ping request");
    HttpResponse::Ok()
}
