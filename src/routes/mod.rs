mod graphql;
mod health;

use actix_web::{web, HttpResponse};
use graphql::{graphiql, graphql};
use health::{get_health, pong, readiness};
use std::env;

pub fn app_routes(config: &mut web::ServiceConfig) {
    let base_path = env::var("BASE_PATH").unwrap_or("".to_string());

    config
        .service(
            web::scope(&format!("{}/", base_path))
                .route("ping", web::get().to(pong))
                .route("~/ready", web::get().to(readiness))
                .route("health", web::get().to(get_health))
                .route("graphiql", web::get().to(graphiql))
                .route("graphql", web::post().to(graphql)),
        )
        .route("", web::get().to(|| HttpResponse::NotFound()));
}
