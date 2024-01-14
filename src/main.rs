use std::io::Result;

use actix_cors::Cors;
use actix_web::{HttpServer, App, http::header, middleware::Logger, web::{self, Data}};
use routes::config::config;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

mod models;
mod routes;

pub struct AppState {
    db: Pool<Postgres>
}

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool: Pool<Postgres> = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await {
            Ok(pool) => {
                println!("✅ Connection to the db is successful!");
                pool
            }
            Err(err) => {
                println!("🔥 Failed to connect to the database {:?}", err);
                std::process::exit(1);
            }
        };
    
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_origin("http://localhost:3000")
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::CONTENT_ENCODING,
                header::ACCEPT,
            ])
            .supports_credentials();
        App::new()
            .app_data(Data::new(AppState{db: pool.clone()}))
//----------- Write functional here ---------------------
            .route("/", web::get().to(routes::user_routes::home))
            .configure(config)
//----------- Write functional here ---------------------
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}