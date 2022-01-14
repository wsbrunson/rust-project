extern crate env_logger;
use actix_web::{get, middleware::Logger, App, HttpRequest, HttpResponse, HttpServer, Responder};
use log::info;

#[get("/health")]
async fn health_check(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
        .content_type("application/json")
        .body("Fit as a fig!")
}

#[get("/")]
async fn index(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body("<html><body><h1>Rust Project</h1></body></html>")
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    info!("Server Starting...");

    return HttpServer::new(|| {
        App::new()
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(index)
            .service(health_check)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await;
}
