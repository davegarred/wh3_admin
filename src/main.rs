extern crate aws_lambda_events;
extern crate lambda_runtime;
extern crate rusoto_core;
extern crate rusoto_dynamodb;
extern crate serde;

use std::error::Error;

use aws_lambda_events::event::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use lambda_runtime::{Context, error::HandlerError, lambda};

use persist::*;
use std::collections::HashMap;

mod persist;
mod dto;


static DB: Dynamo = persist::Dynamo{};

fn main() -> Result<(), Box<dyn Error>> {
    lambda!(handler);
    Ok(())
}

fn handler(req: ApiGatewayProxyRequest, _c: Context) -> Result<ApiGatewayProxyResponse, HandlerError> {
    let method = req.http_method.unwrap();
    let path = req.path.unwrap();
    let body = match req.body {
        Some(body) => body,
        None => String::new(),
    };
    let headers = req.headers;
    let multi_value_headers = req.multi_value_headers;
    println!("{}", method);
    println!("{}", path);
    println!("{}", body);
    let kennel_id = match req.path_parameters.get("proxy") {
        Some(kennel_id) => kennel_id.clone(),
        None => return Ok(unknown_error(String::from("requires kennel id"), 400, headers, multi_value_headers)),
    };
    let kennel = match DB.get_kennel(kennel_id) {
        Ok(val) => val,
        Err(e) => return Ok(unknown_error(e, 500,headers, multi_value_headers)),
    };
    let response = ApiGatewayProxyResponse{
        status_code: 200,
        headers: HashMap::new(),
        multi_value_headers: (HashMap::new()),
        body: Some(kennel),
        is_base64_encoded: None
    };
    Ok(response)
}

fn unknown_error(error: String, status: i64, headers: HashMap<String, String>, multi_value_headers: HashMap<String, Vec<String>>) -> ApiGatewayProxyResponse {
    ApiGatewayProxyResponse {
        status_code: status,
        headers: headers,
        multi_value_headers: multi_value_headers,
        body: Some(error),
        is_base64_encoded: None
    }
}

#[test]
fn test_err() {
//    HandlerError::from("an str".to_string())
}