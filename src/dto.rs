extern crate serde;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Kennel {
    id: String,
    #[serde(default)]
    name: String,
    #[serde(default)]
    description: String,
    #[serde(default,rename(serialize = "hareraiserName", deserialize = "hareraiserName"))]
    hareraiser_name: String,
    #[serde(default,rename(serialize = "hareraiserEmail", deserialize = "hareraiserEmail"))]
    hareraiser_email: String,
    badges: Vec<String>,
    #[serde(default,rename(serialize = "firstHash", deserialize = "firstHash"))]
    first_hash: String,
    #[serde(default)]
    founders: String,
    #[serde(default)]
    lineage: String,
}
