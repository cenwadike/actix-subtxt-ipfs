use actix_web::{App, HttpServer};
use subxt_ipfs::apis::ping;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    log::info!("starting actix server");

    HttpServer::new(|| App::new().service(ping))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
