#[macro_use]
extern crate cached;
#[macro_use]
extern crate lazy_static;

pub mod db;
pub mod models;
pub mod routes;
pub mod schema;

use actix_cors::Cors;
use actix_web::middleware::{DefaultHeaders, Logger};
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use jwt_validator::CertSources;
use std::env;
use std::io;
use std::sync::Arc;
use uuid::Uuid;

use db::Clients;
use schema::create_schema;

use routes::app_routes;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    let rust_log = dotenv::var("RUST_LOG").unwrap_or("info,actix_web=warn".to_owned());
    std::env::set_var("RUST_LOG", rust_log);
    env_logger::init();

    let base_path = env::var("BASE_PATH").unwrap_or("".to_string());
    let port = dotenv::var("PORT").unwrap_or("8080".to_owned());
    let cpu_workers: usize = env::var("NUM_WORKERS")
        .unwrap_or("".to_string())
        .parse()
        .unwrap_or(num_cpus::get() + 2);

    let db_clients = Arc::new(Clients {
        mongo: db::mongo::connect(),
    });

    let cert_sources: Vec<String> = dotenv::var("CERTS")
        .unwrap_or("https://www.googleapis.com/oauth2/v2/certs".to_owned())
        .split(",")
        .map(|s| s.to_owned())
        .collect();
    // this pulls down the sources so that we can validate them
    let certs = CertSources::new(cert_sources);
    let _cert_result = certs.build_keys().await;
    let certs_client = Arc::new(certs);

    let gql = std::sync::Arc::new(create_schema());
    // Start http server
    HttpServer::new(move || {
        App::new()
            .data(gql.clone())
            .data(db_clients.clone())
            .data(certs_client.clone())
            .wrap(Cors::new().finish())
            .wrap(DefaultHeaders::new().header("x-request-id", Uuid::new_v4().to_string()))
            .wrap(Logger::new("IP:%a DATETIME:%t REQUEST:\"%r\" STATUS: %s DURATION:%D X-REQUEST-ID:%{x-request-id}o")
                .exclude(format!("/{}/health", base_path))
                .exclude(format!("/{}/~/ready", base_path)))
            .configure(app_routes)
    })
    .workers(cpu_workers)
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
