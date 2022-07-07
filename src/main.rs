use actix_files::Files;
use actix_web::{get, middleware, web, App, HttpServer, Responder};

#[get("/")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8082");

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .service(Files::new("/", "./static/build/").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8082))?
    .run()
    .await
}
