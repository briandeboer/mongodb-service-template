mod embedded;
mod sample;

pub use embedded::*;
pub use sample::*;

use serde::{Deserialize, Serialize};

#[derive(juniper::GraphQLEnum, Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum Status {
    All,
    Available,
    Active,
    Expired,
    Pending,
}
