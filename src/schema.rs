use actix_web::web::Data;
use bson::doc;
use cached::TimedCache;
use juniper::{FieldError, RootNode};
use jwt_validator::{Claims, TestClaims};
use log::debug;
use mongodb_base_service::{BaseService, DeleteResponseGQL, ServiceError, ID};
use mongodb_cursor_pagination::FindResult;
use std::env;
use std::sync::Arc;
use std::time::SystemTime;

use crate::db::Clients;
use crate::models::*;

pub fn now() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

lazy_static! {
    static ref CACHE_CAPACITY: usize = env::var("CACHE_CAPACITY")
        .unwrap_or("".to_string())
        .parse()
        .unwrap_or(10000);
    static ref CACHE_TTL: u64 = env::var("CACHE_TTL")
        .unwrap_or("".to_string())
        .parse()
        .unwrap_or(60);
    static ref DISABLE_AUTH: u8 = env::var("DISABLE_AUTH")
        .unwrap_or("".to_string())
        .parse()
        .unwrap_or(0);
    static ref REQUIRED_EMAIL_DOMAIN: String =
        env::var("REQUIRED_EMAIL_DOMAIN").unwrap_or("gmail.com".to_string());
}

pub struct Context {
    pub clients: Data<Arc<Clients>>,
    pub claims: Option<Claims>,
}

impl juniper::Context for Context {}

pub struct Query;

#[juniper::object(Context = Context)]
impl Query {
    fn all_samples(
        ctx: &Context,
        limit: Option<i32>,
        after: Option<String>,
        before: Option<String>,
        skip: Option<i32>,
    ) -> Result<SampleConnection, FieldError> {
        cached_key_result! {
            ALL_SAMPLES: TimedCache<String, SampleConnection> =
                TimedCache::with_lifespan_and_capacity(*CACHE_TTL, *CACHE_CAPACITY);
            Key = { format!("{:?},{:?},{:?},{:?}", limit, after, before, skip) };
            fn build(
                ctx: &Clients,
                limit: Option<i32>,
                after: Option<String>,
                before: Option<String>,
                skip: Option<i32>
            ) -> Result<SampleConnection, FieldError> = {
                debug!("Building all samples");
                let service = &ctx.mongo.get_mongo_service("samples").unwrap();
                let result: Result<FindResult<Sample>, ServiceError> = service.find(None, None, limit, after, before, skip);
                match result {
                    Ok(all_items) => {
                        let connection: SampleConnection = all_items.into();
                        Ok(connection)
                    },
                    Err(e) => Err(FieldError::from(e))
                }
            }
        }
        build(ctx.clients.get_ref(), limit, after, before, skip).map_err(|e| e.into())
    }

    fn search_samples(
        ctx: &Context,
        search_term: String,
        fields: Vec<String>,
        limit: Option<i32>,
        after: Option<String>,
        before: Option<String>,
        skip: Option<i32>,
    ) -> Result<SampleConnection, FieldError> {
        cached_key_result! {
            SEARCH_SAMPLES: TimedCache<String, SampleConnection> =
                TimedCache::with_lifespan_and_capacity(*CACHE_TTL, *CACHE_CAPACITY);
            Key = { format!("{:?},{:?},{:?},{:?},{:?},{:?}", search_term, fields, limit, after, before, skip) };
            fn build(
                ctx: &Clients,
                search_term: String,
                fields: Vec<String>,
                limit: Option<i32>,
                after: Option<String>,
                before: Option<String>,
                skip: Option<i32>
            ) -> Result<SampleConnection, FieldError> = {
                let service = &ctx.mongo.get_mongo_service("samples").unwrap();
                let result: Result<FindResult<Sample>, ServiceError> =
                    service.search(search_term, fields, None, limit, after, before, skip);
                match result {
                    Ok(all_items) => {
                        let connection: SampleConnection = all_items.into();
                        Ok(connection)
                    },
                    Err(e) => Err(FieldError::from(e))
                }

            }
        }
        build(
            ctx.clients.get_ref(),
            search_term,
            fields,
            limit,
            after,
            before,
            skip,
        )
    }

    fn samples_by_status(
        ctx: &Context,
        status: Option<Status>,
        limit: Option<i32>,
        after: Option<String>,
        before: Option<String>,
        skip: Option<i32>,
    ) -> Result<SampleConnection, FieldError> {
        cached_key_result! {
            SAMPLES_BY_STATUS: TimedCache<String, SampleConnection> =
                TimedCache::with_lifespan_and_capacity(*CACHE_TTL, *CACHE_CAPACITY);
            Key = { format!("{:?},{:?},{:?},{:?},{:?}", status, limit, after, before, skip) };
            fn build(
                ctx: &Clients,
                status: Option<Status>,
                limit: Option<i32>,
                after: Option<String>,
                before: Option<String>,
                skip: Option<i32>
            ) -> Result<SampleConnection, FieldError> = {
                let service = &ctx.mongo.get_mongo_service("samples").unwrap();
                let timestamp = now();
                let filter = match status {
                    Some(status) => match status {
                        Status::Active => Some(doc! {
                            "available_date": { "$lt": timestamp },
                            "expiration_date": { "$gt": timestamp },
                        }),
                        Status::Expired => Some(doc! {
                            "expiration_date": { "$lt": timestamp },
                        }),
                        Status::Pending => Some(doc! {
                            "available_date": { "$gt": timestamp },
                        }),
                        Status::Available => Some(doc! {
                            "available_date": { "$lt": timestamp }
                        }),
                        Status::All => None,
                    },
                    None => None,
                };
                let result: Result<FindResult<Sample>, ServiceError> =
                    service.find(filter, None, limit, after, before, skip);
                match (result) {
                    Ok(all_items) => {
                        let connection: SampleConnection = all_items.into();
                        Ok(connection)
                    },
                    Err(e) => Err(FieldError::from(e))
                }
            }
        }
        build(ctx.clients.get_ref(), status, limit, after, before, skip)
    }

    // don't cache on requests by id
    fn sample_by_id(ctx: &Context, id: ID) -> Result<Sample, FieldError> {
        let service = ctx
            .clients
            .get_ref()
            .mongo
            .get_mongo_service("samples")
            .unwrap();
        let result: Result<Option<Sample>, ServiceError> = service.find_one_by_id(id);
        match result {
            Ok(item) => match item {
                Some(item) => Ok(item),
                None => Err("Unable to find item".into()),
            },
            Err(e) => Err(FieldError::from(e)),
        }
    }

    fn sample_by_names(
        ctx: &Context,
        names: Vec<String>,
        status: Option<Status>,
    ) -> Result<SampleConnection, FieldError> {
        cached_key_result! {
            SAMPLE_BY_NAME: TimedCache<String, SampleConnection> =
                TimedCache::with_lifespan_and_capacity(*CACHE_TTL, *CACHE_CAPACITY);
            Key = { format!("{:?},{:?}", names, status) };
            fn build(
                ctx: &Clients,
                names: Vec<String>,
                status: Option<Status>
            ) -> Result<SampleConnection, FieldError> = {
                let service = &ctx.mongo.get_mongo_service("samples").unwrap();
                let timestamp = now();

                let mut filter = match status {
                    Some(status) => match status {
                        Status::Active => Some(doc! {
                            "name": { "$in": names },
                            "available_date": { "$lt": timestamp },
                            "expiration_date": { "$gt": timestamp },
                        }),
                        Status::Expired => Some(doc! {
                            "name": { "$in": names },
                            "expiration_date": { "$lt": timestamp },
                        }),
                        Status::Pending => Some(doc! {
                            "name": { "$in": names },
                            "available_date": { "$gt": timestamp },
                        }),
                        Status::Available => Some(doc! {
                            "name": { "$in": names },
                            "available_date": { "$lt": timestamp }
                        }),
                        Status::All => Some(doc! { "name": doc! { "$in": names } }),
                    },
                    None => Some(doc! { "name": { "$in": names } }),
                };

                let result: Result<FindResult<Sample>, ServiceError> = service.find(filter, None, None, None, None, None);
                match result {
                    Ok(all_items) => {
                        let connection: SampleConnection = all_items.into();
                        Ok(connection)
                    },
                    Err(e) => Err(FieldError::from(e))
                }
            }
        }
        build(ctx.clients.get_ref(), names, status).map_err(|e| e.into())
    }
}

pub struct Mutation;

fn has_auth(ctx: &Context) -> bool {
    if ctx.claims.is_none()
        || !ctx.claims.clone().unwrap().validate(TestClaims {
            hd: Some(REQUIRED_EMAIL_DOMAIN.to_string()),
            ..TestClaims::default()
        })
    {
        return false;
    }
    return true;
}

#[juniper::object(Context = Context)]
impl Mutation {
    // samples
    fn create_sample(
        ctx: &Context,
        mut new_sample: NewSample,
        created_by_id: Option<ID>,
    ) -> Result<Sample, FieldError> {
        if !has_auth(ctx) && *DISABLE_AUTH != 1 {
            return Err("Unauthorized".into());
        }
        let service = ctx.clients.mongo.get_mongo_service("samples").unwrap();
        let inserted_id: ID = service.insert_one(new_sample, created_by_id)?;
        let maybe_item = service.find_one_by_id(inserted_id)?;
        match maybe_item {
            Some(item) => Ok(item),
            None => Err("Unable to retrieve object after insert".into()),
        }
    }

    fn update_sample(
        ctx: &Context,
        id: ID,
        update_sample: UpdateSample,
        updated_by_id: Option<ID>,
    ) -> Result<Sample, FieldError> {
        if !has_auth(ctx) && *DISABLE_AUTH != 1 {
            return Err("Unauthorized".into());
        }
        // check authorization first
        let service = ctx.clients.mongo.get_mongo_service("samples").unwrap();
        service
            .update_one(id, update_sample, updated_by_id)
            .map_err(|e| e.into())
    }

    fn delete_sample(ctx: &Context, id: ID) -> Result<DeleteResponseGQL, FieldError> {
        if !has_auth(ctx) && *DISABLE_AUTH != 1 {
            return Err("Unauthorized".into());
        }
        let service = ctx.clients.mongo.get_mongo_service("samples").unwrap();
        match service.delete_one_by_id(id) {
            Ok(result) => Ok(result.into()),
            Err(e) => Err(e.into()),
        }
    }

    // embedded objects
    fn add_values_to_sample(
        ctx: &Context,
        sample_id: ID,
        new_values: Vec<NewEmbedded>,
        created_by_id: Option<ID>,
    ) -> Result<Sample, FieldError> {
        if !has_auth(ctx) && *DISABLE_AUTH != 1 {
            return Err("Unauthorized".into());
        }
        let service = ctx.clients.mongo.get_mongo_service("samples").unwrap();
        let _ids =
            service.insert_embedded(sample_id.clone(), "values", new_values, created_by_id)?;
        let maybe_item = service.find_one_by_id(sample_id)?;
        match maybe_item {
            Some(item) => Ok(item),
            None => Err("Unable to retrieve object after insert".into()),
        }
    }

    fn remove_value_from_sample(
        ctx: &Context,
        sample_id: ID,
        embedded_id: ID,
    ) -> Result<DeleteResponseGQL, FieldError> {
        if !has_auth(ctx) && *DISABLE_AUTH != 1 {
            return Err("Unauthorized".into());
        }
        let service = ctx.clients.mongo.get_mongo_service("samples").unwrap();
        match service.delete_embedded(sample_id, "values", embedded_id) {
            Ok(result) => Ok(result.into()),
            Err(e) => Err(e.into()),
        }
    }

    fn update_value_for_sample(
        ctx: &Context,
        sample_id: ID,
        embedded_id: ID,
        update_value: UpdateEmbedded,
        updated_by_id: Option<ID>,
    ) -> Result<Sample, FieldError> {
        if !has_auth(ctx) && *DISABLE_AUTH != 1 {
            return Err("Unauthorized".into());
        }
        let service = ctx.clients.mongo.get_mongo_service("samples").unwrap();
        service
            .update_embedded(
                sample_id,
                "values",
                embedded_id,
                update_value,
                updated_by_id,
            )
            .map_err(|e| e.into())
    }
}

pub type Schema = RootNode<'static, Query, Mutation>;

#[allow(dead_code)]
pub fn create_schema() -> Schema {
    Schema::new(Query {}, Mutation {})
}
