mod payments;
mod schemas;

use actix_web::{
    App, HttpServer,
    middleware::{self, Logger},
    web::{self},
};
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenvy::dotenv;
use env_logger::Env;
use memcache::Client;
/*
 * docker run -d --name postgres17.5-rinha-enferrujada --restart always -p 5432:5432 -e POSTGRES_USER=rinha-enferrujada -e POSTGRES_PASSWORD=rinha-enferrujada -e POSTGRES_DB=rinha-enferrujada --health-cmd "pg_isready -U rinha-enferrujada" --health-interval 5s --health-retries 5 --health-timeout 3s postgres:17.5-alpine
 *
 * */

struct AppState {
    pub pool: Pool<ConnectionManager<PgConnection>>,
    pub memcached_client: Client,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let port = std::env::var("PORT").expect("PORT env var must be set");
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in environment variables");
    let cache_url =
        std::env::var("CACHE_URL").expect("CACHE_URL must be set in environment variables");

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    let pool = Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Failed to create database pool");

    let mclient = Client::connect(cache_url).expect("Failed to connect to Memcached");

    let initial_state = web::Data::new(AppState {
        pool: pool.clone(),
        memcached_client: mclient.clone(),
    });

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .app_data(initial_state.clone())
            .wrap(middleware::NormalizePath::trim())
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            //     .prefer_utf8(true)
            //     .show_files_listing()
            //     .use_last_modified(true)
            // )
            .service(web::scope("").configure(crate::payments::routes::payment_routes))
    })
    .bind(("0.0.0.0", port.parse().unwrap()))?
    .run()
    .await
}
