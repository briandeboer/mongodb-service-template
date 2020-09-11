use bson::doc;
use chrono::{DateTime, Utc};
use mongodb_base_service::{Node, NodeDetails, ID};
use serde::{Deserialize, Serialize};
use std::hash::Hash;

use crate::schema::Context;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Embedded {
    #[serde(rename = "_id")] // Use MongoDB's special primary key field name when serializing
    pub id: ID,
    pub node: NodeDetails,
    pub embedded_type: EmbeddedType,
    pub value: Option<f64>,
}

impl Node for Embedded {
    fn node(&self) -> &NodeDetails {
        &self.node
    }
}

#[juniper::object(Context = Context, description = "An accepted rate")]
impl Embedded {
    fn id(&self) -> &ID {
        &self.id
    }

    fn date_created(&self) -> Option<DateTime<Utc>> {
        self.node.date_created()
    }

    fn date_modified(&self) -> Option<DateTime<Utc>> {
        self.node.date_modified()
    }

    fn embedded_type(&self) -> EmbeddedType {
        self.embedded_type
    }

    fn value(&self) -> f64 {
        self.value.unwrap_or(0.)
    }
}

#[derive(juniper::GraphQLEnum, Clone, Copy, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum EmbeddedType {
    One,
    Another,
}

#[derive(Serialize, Deserialize, juniper::GraphQLInputObject)]
pub struct NewEmbedded {
    #[serde(rename = "_id")]
    pub id: Option<ID>,
    pub embedded_type: EmbeddedType,
    pub value: Option<f64>,
}

#[derive(Serialize, Deserialize, juniper::GraphQLInputObject)]
pub struct UpdateEmbedded {
    /// Optional updated embedded type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedded_type: Option<EmbeddedType>,

    /// Optional updated value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}
