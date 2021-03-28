use actix_web::{App, HttpServer};
use function;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(function::handler))
        .bind("127.0.0.1:3000")?
        .run()
        .await
}
