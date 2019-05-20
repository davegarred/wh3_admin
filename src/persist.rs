use std::collections::hash_map::{HashMap};
use std::error::Error;
use std::fs;
use std::string::ToString;

use rusoto_core::Region;
use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, GetItemInput, PutItemInput};

use dto::Kennel;

const KENNEL_ID_FIELD: &str = "kennel_id";
const PAYLOAD_FIELD: &str = "payload";
const KENNEL_TABLE: &str = "wh3_kennel";

pub trait Persister {
    fn get_kennel(&self, kennel_id: &String) -> Result<String, String>;
    fn put_kennel(&self, kennel_id: &String, kennel_serialization: String) -> Result<(), String>;
}

pub struct Dynamo{
    client: DynamoDbClient,
}

impl Dynamo {
    pub fn new() -> Dynamo {
        Dynamo{ client: DynamoDbClient::new(Region::UsWest2) }
    }
    fn string_attribute_value(value: &String) -> AttributeValue {
        AttributeValue {
            s: Some(value.clone()),
            ..Default::default()
        }
    }
}


impl Persister for Dynamo {
    fn get_kennel(&self, kennel_id: &String) -> Result<String, String> {
        let mut key = HashMap::<String, AttributeValue>::new();
        key.insert(String::from(KENNEL_ID_FIELD), Dynamo::string_attribute_value(&kennel_id));

        let query = GetItemInput {
            key,
            table_name: String::from(KENNEL_TABLE),
            ..Default::default()
        };
//        let client = DynamoDbClient::new(Region::UsWest2);
        let result = match self.client.get_item(query).sync() {
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
        Ok(string_payload.clone())
    }

    fn put_kennel(&self, kennel_id: &String, kennel_serialization: String) -> Result<(), String> {
        validate(&kennel_serialization)?;
        let mut item: HashMap<String, AttributeValue> = HashMap::new();
        item.insert(String::from(KENNEL_ID_FIELD), Dynamo::string_attribute_value(&kennel_id));
        item.insert(String::from(PAYLOAD_FIELD), Dynamo::string_attribute_value(&kennel_serialization));
        let query = PutItemInput {
            item,
            table_name: String::from(KENNEL_TABLE),
            ..Default::default()
        };
        match self.client.put_item(query).sync() {
            Ok(_) => Ok(()),
            Err(e) => Err(String::from(e.description())),
        }
    }
}


fn validate(kennel_serialization: &String) -> Result<(), String> {
    let _kennel: Kennel = match serde_json::from_str(kennel_serialization.as_ref()) {
        Ok(kennel) => kennel,
        Err(e) => return Err(String::from(e.description())),
    };
    Ok(())
}

struct TestDb;

impl Persister for TestDb {
    fn get_kennel(&self, _: &String) -> Result<String, String> {
        let ser = fs::read_to_string(String::from("res/json/puget_sound.json")).unwrap();
        Ok(ser)
    }
    fn put_kennel(&self, _kennel_id: &String, _kennel_serialization: String) -> Result<(), String> {
        Ok(())
    }
}

#[test]
fn test_dynamo() {
    let db = TestDb {};
//    let db = Dynamo::new();
    let puget_sound = String::from("PUGET_SOUND");


    let ser = fs::read_to_string(String::from("res/json/puget_sound.json")).unwrap();
    db.put_kennel(&puget_sound, ser).unwrap();

    let kennel_str = db.get_kennel(&puget_sound).unwrap();
    println!("{:#?}", kennel_str);

}

