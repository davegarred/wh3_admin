extern crate aws_lambda_events;
extern crate lambda_runtime;
extern crate rusoto_core;
extern crate rusoto_dynamodb;
extern crate serde;

use std::collections::HashMap;
use std::error::Error;

use aws_lambda_events::event::apigw::ApiGatewayProxyRequest;
use lambda_runtime::{Context, error::HandlerError, lambda};
use rusoto_core::Region;
use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, GetItemInput};
use serde::{Deserialize, Serialize};

fn main() -> Result<(), Box<dyn Error>> {
    lambda!(handler);
    Ok(())
}

fn handler(e: ApiGatewayProxyRequest, _c: Context) -> Result<Kennel, HandlerError> {
    println!("{:?}", e);
//    let val = get_kennel("PUGET_SOUND".to_string()).unwrap();
    match get_kennel("PUGET_SOUND".to_string()) {
        Ok(val) => {
            let k: Kennel = serde_json::from_str(val.as_str())?;
            Ok(k)
        }
        Err(e) => {
            panic!(e)
        }

    }
}

fn get_kennel(kennel_id: String) -> Result<String, String> {
    let mut query_key = HashMap::<String, AttributeValue>::new();
    query_key.insert("kennel_id".to_string(), AttributeValue {
        s: Some(kennel_id),
        ..Default::default()
    });
    let query = GetItemInput {
        key: query_key,
        table_name: "wh3_kennel".to_string(),
        ..Default::default()
    };
    let client = DynamoDbClient::new(Region::UsWest2);
    match client.get_item(query).sync() {
        Ok(result) => {
            match result.item {
                Some(attribute_map) => {
                    match attribute_map.get("payload") {
                        Some(payload) => {
                            match &payload.s {
                                Some(string_payload) => {
                                    Ok(string_payload.to_string())
                                }
                                None => Err("payload is not a string".to_string())
                            }
                        }
                        None => Err("no payload found".to_string())
                    }
                }
                None => Err("query returned no result".to_string())
            }
        }
        Err(err) => {
            let val = format!("failure to get kennels: {:?}", err);
            Err(val)
        }
    }
}

#[test]
fn test() {
    match get_kennel("PUGET_SOUND".to_string()) {
        Ok(val) => {
            let k: Kennel = serde_json::from_str(val.as_str()).unwrap();
            println!("{:#?}", k);
            println!("{}", val);
        }
        Err(e) => {
            panic!(e)
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Kennel {
    id: String,
    name: String,
    description: String,
    #[serde(rename(serialize = "hareraiserName", deserialize = "hareraiserName"))]
    hareraiser_name: String,
    #[serde(rename(serialize = "hareraiserEmail", deserialize = "hareraiserEmail"))]
    hareraiser_email: String,
    badges: Vec<String>,
    #[serde(rename(serialize = "firstHash", deserialize = "firstHash"))]
    first_hash: String,
    founders: String,
    lineage: String,
}