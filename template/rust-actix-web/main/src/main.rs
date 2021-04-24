use function;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    function::app_init().await
}
