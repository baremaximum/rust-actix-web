use actix_web::{get, HttpRequest, Responder};

#[get("/")]
pub async fn handler(_req: HttpRequest) -> impl Responder {
    "OK"
}

mod tests {
    use super::*;
    use actix_web::{rt as actix_rt, test, web, App};
    #[actix_rt::test]
    async fn test_handler() {
        let mut app = test::init_service(App::new().service(handler)).await;
        // Change HTTP method below if required
        let req = test::TestRequest::get().uri("/").to_request();
        let response = test::call_service(&mut app, req).await;
        assert!(response.status().is_success());
    }
}
