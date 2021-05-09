use actix_web::{get, middleware, App, HttpServer};

#[get("/")]
async fn index() -> String {
    "Hello, world!".to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::DefaultHeaders::new().header("X-Version", "0.2"))
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(index)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
