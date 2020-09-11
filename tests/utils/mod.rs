use actix_web::web;
use mongodb_base_service::{mock_time, BaseService};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use sample_project::db::Clients;
use sample_project::schema::create_schema;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GqlQuery<'a> {
    pub operation_name: &'a str,
    pub query: &'a str,
}

fn get_data_from_file<T>(filename: &str) -> Vec<T>
where
    T: DeserializeOwned,
{
    let mut file =
        File::open(filename).expect(&format!("Unable to open data filename {}", filename));

    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();

    let items: Vec<T> = serde_json::from_str(&buffer).expect("Unable to parse");
    items
}

pub fn load_filled_database(config: &mut web::ServiceConfig) {
    // disable cache
    std::env::set_var("CACHE_TTL", "0");
    std::env::set_var("CACHE_CAPACITY", "0");

    let mongo_url = std::env::var("MONGO_URL").unwrap_or("mongodb://localhost:27017/".to_string());
    std::env::set_var("MONGO_URL", mongo_url);
    let db_name = std::env::var("MONGO_DB_NAME").unwrap_or("sample-project-test".to_string());
    std::env::set_var("MONGO_DB_NAME", db_name);

    // fix time to Jan 1, 2020 so that snapshots always have the same dateModified etc...
    mock_time::set_mock_time(SystemTime::UNIX_EPOCH + Duration::from_millis(1577836800000));

    let db_clients = Arc::new(Clients {
        mongo: sample_project::db::mongo::connect(),
    });

    // drop and load current data
    let dbs = vec!["samples"];
    dbs.iter().for_each(|db| {
        let _result = db_clients
            .mongo
            .get_mongo_service(db)
            .unwrap()
            .data_source()
            .drop(None);
    });

    // load data
    let samples: Vec<sample_project::models::Sample> =
        get_data_from_file("./tests/mock/samples.json");
    let service = db_clients.mongo.get_mongo_service("samples").unwrap();
    let _result = service.insert_many(samples, None);

    let gql = std::sync::Arc::new(create_schema());

    // connect the app
    config.data(db_clients.clone());
    config.data(gql);
}
