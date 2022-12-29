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
