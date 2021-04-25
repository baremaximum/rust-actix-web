use function;

/// Initialized env_logger and starts the app from the object
/// that is defined in the function crate
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    function::app_init().await
}
