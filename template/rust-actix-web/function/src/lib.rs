use actix_web::{middleware, post, web, App, HttpRequest, HttpServer, Responder};
use log::info;
use std::env;

#[post("/")]
pub async fn handler(_req: HttpRequest) -> impl Responder {
    "OK"
}

#[actix_web::main]
pub async fn app_init() -> std::io::Result<()> {
    // get worker pool size from env.
    let cnt = env::var("WORKER_POOL_SIZE");
    let mut worker_count: usize = 1;

    match cnt {
        Ok(cnt) => {
            worker_count = cnt
                .parse::<usize>()
                .expect("Could not parse WORKER_POOL_SIZE. Value must parse to valid usize")
        }
        Err(_) => info!("WORKER_POOL_SIZE not set. Using default value 1."),
    }

    // get max json size from env.
    let max = env::var("JSON_MAX_SIZE");
    let mut max_size: usize = 4096;

    match max {
        Ok(max) => {
            max_size = max
                .parse::<usize>()
                .expect("Could not parse WORKER_POOL_SIZE. Value must parse to valid usize")
        }
        Err(_) => info!("JSON_MAX_SIZE not set. Using default value 4096."),
    }

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(max_size))
            .service(handler)
    })
    .workers(worker_count)
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[allow(unused_imports)]
    use actix_web::{rt as actix_rt, test};
    #[actix_rt::test]
    async fn test_handler() {
        let mut app = test::init_service(App::new().service(handler)).await;
        let req = test::TestRequest::post().uri("/").to_request();
        let response = test::call_service(&mut app, req).await;
        assert!(response.status().is_success());
    }
}
