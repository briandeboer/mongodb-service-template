use bson::doc;
use chrono::{DateTime, TimeZone, Utc};
use mongodb_base_service::{Node, NodeDetails, ID};
use mongodb_cursor_pagination::{Edge, FindResult, PageInfo};
use serde::{Deserialize, Serialize};

use crate::models::Embedded;
use crate::schema::Context;

#[derive(Clone, Serialize, Deserialize)]
pub struct Sample {
    #[serde(rename = "_id")] // Use MongoDB's special primary key field name when serializing
    pub id: ID,
    pub node: NodeDetails,
    name: String,
    description: Option<String>,
    available_date: Option<i64>,
    expiration_date: Option<i64>,
    pub values: Option<Vec<Embedded>>,
}

impl Node for Sample {
    fn node(&self) -> &NodeDetails {
        &self.node
    }
}

#[juniper::object(Context = Context, description = "Sample model")]
impl Sample {
    fn id(&self) -> &ID {
        &self.id
    }

    fn date_created(&self) -> Option<DateTime<Utc>> {
        self.node.date_created()
    }

    fn date_modified(&self) -> Option<DateTime<Utc>> {
        self.node.date_modified()
    }

    fn created_by(&self) -> Option<&ID> {
        match self.node.created_by_id() {
            Some(id) => Some(id),
            None => None,
        }
    }

    fn updated_by(&self) -> Option<&ID> {
        match self.node.updated_by_id() {
            Some(id) => Some(id),
            None => None,
        }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &Option<String> {
        &self.description
    }

    fn available_date(&self) -> Option<DateTime<Utc>> {
        match self.available_date {
            Some(avail) => Some(Utc.timestamp(avail, 0)),
            None => self.node.date_created(),
        }
    }

    fn expiration_date(&self) -> Option<DateTime<Utc>> {
        match self.expiration_date {
            Some(expire) => Some(Utc.timestamp(expire, 0)),
            None => None,
        }
    }

    fn values(&self) -> &Option<Vec<Embedded>> {
        &self.values
    }

    fn min_value(&self) -> f64 {
        self.values
            .clone()
            .unwrap_or(vec![])
            .iter()
            .fold(0., |mut acc, value| {
                let value = value.value.unwrap_or(0.);
                if acc == 0. {
                    acc = value
                } else if value < acc && value != 0. {
                    acc = value;
                }
                acc
            })
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SampleConnection {
    pub page_info: PageInfo,
    pub edges: Vec<Edge>,
    pub items: Vec<Sample>,
    pub total_count: i64,
}

#[juniper::object(Context = Context)]
impl SampleConnection {
    fn page_info(&self) -> &PageInfo {
        &self.page_info
    }

    fn edges(&self) -> &Vec<Edge> {
        &self.edges
    }

    fn items(&self) -> &Vec<Sample> {
        &self.items
    }

    fn total_count(&self) -> i32 {
        self.total_count as i32
    }
}

impl From<FindResult<Sample>> for SampleConnection {
    fn from(fr: FindResult<Sample>) -> SampleConnection {
        SampleConnection {
            page_info: fr.page_info,
            edges: fr.edges,
            items: fr.items,
            total_count: fr.total_count,
        }
    }
}
#[derive(Serialize, Deserialize, juniper::GraphQLInputObject)]
pub struct NewSample {
    #[serde(rename = "_id")]
    id: Option<ID>,
    name: String,
    description: Option<String>,
    available_date: Option<i32>,
    expiration_date: Option<i32>,
}

#[derive(Serialize, Deserialize, juniper::GraphQLInputObject)]
pub struct UpdateSample {
    /// Optional updated name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Optional updated description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Optional updated available date for the content, sent as unix time stamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_date: Option<i32>,

    /// Optional updated expiration date for the content, sent as unix time stamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<i32>,
}
