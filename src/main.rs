extern crate aws_lambda_events;
extern crate lambda_runtime;
#[macro_use]
extern crate maplit;
extern crate rusoto_core;
extern crate rusoto_dynamodb;
extern crate serde;

use aws_lambda_events::event::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use lambda_runtime::{Context, error::HandlerError, lambda};
use lambda_runtime::Handler;
use persist::*;
use std::error::Error;

mod persist;
mod dto;
mod web;

const POST: &str = "POST";
const GET: &str = "GET";
const REQUEST_PARAMETER: &str = "proxy";

struct App {}


fn main() -> Result<(), Box<dyn Error>> {
    let app = App {};
    lambda!(app);
    Ok(())
}

impl Handler<ApiGatewayProxyRequest, ApiGatewayProxyResponse, HandlerError> for App {
    fn run(&mut self, req: ApiGatewayProxyRequest, _ctx: Context) -> Result<ApiGatewayProxyResponse, HandlerError> {
        let kennel_id = match req.path_parameters.get(REQUEST_PARAMETER) {
            Some(kennel_id) => kennel_id.clone(),
            None => return Ok(web::error_result(String::from("requires kennel id"), 400)),
        };
        match req.http_method.unwrap().as_ref() {
            POST => {
                match req.body {
                    Some(body) => Ok(put_kennel(&kennel_id, body)),
                    None => Ok(web::error_result(String::from("no body"), 400)),
                }
            }
            GET => {
                Ok(get_kennel(&kennel_id))
            }
            _ => Ok(web::error_result(String::from("method not allowed"), 405)),
        }
    }
}

fn get_kennel(kennel_id: &String) -> ApiGatewayProxyResponse {
    let db = Dynamo::new();
    match db.get_kennel(kennel_id) {
        Ok(val) => web::success(Some(val)),
        Err(e) => web::error_result(e, 500),
    }
}

fn put_kennel(kennel_id: &String, kennel_serialization: String) -> ApiGatewayProxyResponse {
    let db = Dynamo::new();
    match db.put_kennel(kennel_id, kennel_serialization) {
        Ok(_) => web::success(Some(String::new())),
        Err(e) => web::error_result(e, 500),
    }
}


#[test]
fn test_err() {
//    HandlerError::from("an str".to_string())
}