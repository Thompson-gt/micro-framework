use crate::types::error_types::internal_server_error::InternalServerError;
use crate::types::error_types::internal_server_result::InternalServerResult;
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum HttpMethods {
    GET,
    POST,
    DELETE,
    PUT,
    NEXT,
}
#[allow(unused)]
impl HttpMethods {
    /// returns the correct enum value for the given &str given
    pub fn interpret(method: &str) -> InternalServerResult<HttpMethods> {
        match method.to_uppercase().as_ref() {
            "GET" => Ok(HttpMethods::GET),
            "PUT" => Ok(HttpMethods::PUT),
            "POST" => Ok(HttpMethods::POST),
            "DELETE" => Ok(HttpMethods::DELETE),
            "NEXT" => Ok(HttpMethods::NEXT),
            _ => Err(InternalServerError::ConstructError(
                "HttpMethod".to_string(),
                "invalid http method was given".to_string(),
            )),
        }
    }
    /// returns the string version of the given enum value
    pub fn to_stirng(self) -> String {
        match self {
            HttpMethods::GET => "GET".to_string(),
            HttpMethods::PUT => "PUT".to_string(),
            HttpMethods::POST => "POST".to_string(),
            HttpMethods::DELETE => "DELETE".to_string(),
            HttpMethods::NEXT => "NEXT".to_string(),
        }
    }
}
