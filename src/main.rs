use actix_files::Files;
use actix_web::{get, middleware, App, HttpServer, Responder};

#[get("/persons")]
async fn greet() -> impl Responder {
    actix_files::NamedFile::open_async("./db.json").await
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8082");

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .service(greet)
            .service(Files::new("/", "./static/build/").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8082))?
    .run()
    .await
}
