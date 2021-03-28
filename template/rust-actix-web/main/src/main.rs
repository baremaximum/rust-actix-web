use actix_web::{App, HttpServer};
use handler;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(handler::handler))
        .bind("127.0.0.1:3000")?
        .run()
        .await
}
