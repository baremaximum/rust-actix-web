use actix_web::{post, HttpRequest, Responder};

#[post("/")]
pub async fn handler(_req: HttpRequest) -> impl Responder {
    "OK"
}

mod tests {
    use actix_web;
    use actix_web::rt as actix_rt;
    #[actix_rt::test]
    async fn test_handler() {
        let mut app =
            actix_web::test::init_service(actix_web::App::new().service(super::handler)).await;
        let req = actix_web::test::TestRequest::post().uri("/").to_request();
        let response = actix_web::test::call_service(&mut app, req).await;
        assert!(response.status().is_success());
    }
}
