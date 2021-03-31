use actix_web::{web, middleware, App, HttpServer};
use log::info;
use function;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    // get worker pool size from env.
    let cnt = env::var("WORKER_POOL_SIZE");
    let mut worker_count: usize = 1;

    match cnt {
        Ok(cnt) => { 
            worker_count = cnt.parse::<usize>()
                .expect("Could not parse WORKER_POOL_SIZE. Value must parse to valid usize") 
        }
        Err(_) => info!("WORKER_POOL_SIZE not set. Using default value 1.")
    }

    // get max json size from env.
    let max = env::var("JSON_MAX_SIZE");
    let mut max_size: usize = 4096;

    match max {
        Ok(max) => { 
            max_size = max.parse::<usize>()
                .expect("Could not parse WORKER_POOL_SIZE. Value must parse to valid usize") 
        }
        Err(_) => info!("JSON_MAX_SIZE not set. Using default value 4096.")
    }

    HttpServer::new(move || 
        App::new()
        .wrap(middleware::Logger::default())
        .data(web::JsonConfig::default().limit(max_size))
        .service(function::handler))
        .workers(worker_count)
        .bind("127.0.0.1:3000")?
        .run()
        .await
}
