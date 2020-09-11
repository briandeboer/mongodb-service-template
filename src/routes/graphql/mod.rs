use crate::db::Clients;
use crate::schema::{Context, Schema};

use actix_web::{web, Error, HttpResponse};
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;
use jwt_validator::{Claims, TestClaims};

use std::env;
use std::sync::Arc;

lazy_static! {
    static ref DISABLE_AUTH: u8 = env::var("DISABLE_AUTH")
        .unwrap_or("".to_string())
        .parse()
        .unwrap_or(0);
    static ref REQUIRED_EMAIL_DOMAIN: String =
        env::var("REQUIRED_EMAIL_DOMAIN").unwrap_or("gmail.com".to_string());
}

fn invalid_request() -> HttpResponse {
    HttpResponse::Unauthorized().body("Invalid request")
}

pub async fn graphiql(claims: Option<Claims>) -> HttpResponse {
    if *DISABLE_AUTH != 1
        && (claims.is_none() || {
            !claims.unwrap().validate(TestClaims {
                hd: Some(REQUIRED_EMAIL_DOMAIN.to_string()),
                ..TestClaims::default()
            })
        })
    {
        return invalid_request();
    }
    let api_base = dotenv::var("API_BASE").unwrap_or("http://localhost:8080".to_owned());
    let base_path = env::var("BASE_PATH").unwrap_or("".to_string());
    let html = graphiql_source(&format!("{}/{}/graphql", api_base, base_path));
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

pub async fn graphql(
    st: web::Data<Arc<Schema>>,
    clients: web::Data<Arc<Clients>>,
    data: web::Json<GraphQLRequest>,
    claims: Option<Claims>,
) -> Result<HttpResponse, Error> {
    let context = Context { clients, claims };

    let result = web::block(move || {
        let res = data.execute(&st, &context);
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await?;
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(result))
}
