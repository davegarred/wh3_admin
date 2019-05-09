use std::collections::hash_map::HashMap;
use std::fs;
use std::string::ToString;

use rusoto_core::Region;
use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, GetItemInput};

const KENNEL_ID_KEY: &str = "kennel_id";
const KENNEL_TABLE_KEY: &str = "wh3_kennel";

pub trait Persister {
    fn get_kennel(&self, String) -> Result<String, String>;
}

pub struct Dynamo;

impl Persister for Dynamo {
    fn get_kennel(&self, kennel_id: String) -> Result<String, String> {
        let mut query_key = HashMap::<String, AttributeValue>::new();
        query_key.insert(String::from(KENNEL_ID_KEY), AttributeValue {
            s: Some(kennel_id),
            ..Default::default()
        });
        let query = GetItemInput {
            key: query_key,
            table_name: String::from(KENNEL_TABLE_KEY),
            ..Default::default()
        };
        let client = DynamoDbClient::new(Region::UsWest2);
        let result = match client.get_item(query).sync() {
            Ok(result) => result,
            Err(err) => {
                let val = format!("failure to get kennels: {:?}", err);
                return Err(val);
            }
        };
        let attribute_map = result.item.ok_or("query returned no result".to_string())?;
        let payload = attribute_map.get("payload").ok_or("no payload found".to_string())?;
        let string_payload = match &payload.s {
            Some(string_payload) => string_payload,
            None => return Err("payload is not a string".to_string()),
        };
//        let k: Kennel = serde_json::from_str(string_payload.as_str()).unwrap();
        Ok(string_payload.clone())
    }
}

pub struct TestDb;

impl Persister for TestDb {
    fn get_kennel(&self, _: String) -> Result<String, String> {
        let ser = fs::read_to_string(String::from("res/json/puget_sound.json")).unwrap();
//        let k: Kennel = serde_json::from_str(ser.as_str()).unwrap();
        Ok(ser)
    }
}

#[test]
fn test_dynamo() {
    let db = TestDb {};
//    let db = persist::Dynamo{};
    match db.get_kennel(String::from("PUGET_SOUND")) {
        Ok(val) => {
            println!("{:#?}", val);
        }
        Err(e) => {
            panic!(e)
        }
    }
}
