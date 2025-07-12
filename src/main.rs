use actix_web::{
    App, HttpServer,
    middleware::{self, Logger},
    web::{self},
};
use actix_web::{Error, HttpResponse, get, http::header::ContentType};
use dotenvy::dotenv;
use env_logger::Env;

#[get("/")]
async fn index() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body("<h1>Ola Mundo</h1>"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let port = std::env::var("PORT").expect("PORT env var must be set");

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::NormalizePath::trim())
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            //     .prefer_utf8(true)
            //     .show_files_listing()
            //     .use_last_modified(true)
            // )
            .service(index)
    })
    .bind(("0.0.0.0", port.parse().unwrap()))?
    .run()
    .await
}
