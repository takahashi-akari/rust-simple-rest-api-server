/**
 * Simple REST API Server
 */
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

// Define the index route
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

// Define the hello route
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello!")
}

// Define the main function
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Start the server
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/hello", web::get().to(hello))
    })
    .bind("localhost:8080")?
    .run()
    .await
}
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;
    use actix_web::http::StatusCode;

    #[actix_rt::test]
    async fn test_index() {
        let mut app = test::init_service(App::new().route("/", web::get().to(index))).await;
        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
        let body = test::read_body(resp).await;
        assert_eq!(body, "Hello world!");
    }

    #[actix_rt::test]
    async fn test_hello() {
        let mut app = test::init_service(App::new().route("/hello", web::get().to(hello))).await;
        let req = test::TestRequest::get().uri("/hello").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
        let body = test::read_body(resp).await;
        assert_eq!(body, "Hello!");
    }
}
