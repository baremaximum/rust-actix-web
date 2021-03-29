use actix_web::{post, HttpRequest, Responder};

#[post("/")]
pub async fn handler(_req: HttpRequest) -> impl Responder {
    "OK"
}

mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[allow(unused_imports)]
    use actix_web::{rt as actix_rt, test, web, App};
    #[actix_rt::test]
    async fn test_handler() {
        let mut app = test::init_service(App::new().service(handler)).await;
        let req = test::TestRequest::post().uri("/").to_request();
        let response = test::call_service(&mut app, req).await;
        assert!(response.status().is_success());
    }
}
