use std::collections::HashMap;

use aws_lambda_events::event::apigw::{ApiGatewayProxyResponse};

pub fn error_result(error: String, status: i64) -> ApiGatewayProxyResponse {
    ApiGatewayProxyResponse {
        status_code: status,
        headers: hashmap!(String::from("Content-Type") => String::from("application/json")),
        multi_value_headers: HashMap::new(),
        body: Some(error),
        is_base64_encoded: None,
    }
}
pub fn success(body: Option<String>) -> ApiGatewayProxyResponse {
    ApiGatewayProxyResponse {
        status_code: 200,
        headers: hashmap!(String::from("Content-Type") => String::from("application/json")),
        multi_value_headers: HashMap::new(),
        body: body,
        is_base64_encoded: None,
    }
}
