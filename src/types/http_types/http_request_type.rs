use crate::types::error_types::{
    internal_server_error::InternalServerError, internal_server_result::InternalServerResult,
};
use crate::utils::{constants, https_methods::HttpMethods};
use std::{
    io::{prelude::*, BufReader},
    net::TcpStream,
};

#[derive(Debug)]
/// request struct the raw http request is parsed into
pub struct HttpRequest {
    pub method: HttpMethods,
    pub uri: String,
    pub version: String,
    pub body: String,
    pub headers: Vec<String>,
    pub raw_request_string: String,
}

/// the request that was passed by the user and parsed into a readable struct
impl HttpRequest {
    pub fn new(data: &mut TcpStream) -> InternalServerResult<Self> {
        let stream = match data.try_clone() {
            Ok(s) => s,
            Err(_) => {
                return Err(InternalServerError::FatalError(
                    "couldn't make reference to the tcp socket".to_string(),
                ))
            }
        };
        let buff = BufReader::new(data);
        let raw_request = match Self::parse_raw_request(buff) {
            Ok(r) => r,
            Err(_) => {
                stream.shutdown(std::net::Shutdown::Both).map_err(|e| {
                    InternalServerError::FatalError(format!(
                        "failed to close connection to client: {}",
                        e
                    ))
                })?;
                return Err(InternalServerError::ParseError(
                    "failed to parse the raw reqest from the user".to_string(),
                ));
            }
        };

        let raw_request_stirng = raw_request.clone();
        let split_request: Vec<_> = raw_request
            .split(constants::DOUBLE_CARRIAGE_RETURN)
            .collect();
        let mut http_request = Self::build_http_request(split_request)?;
        http_request.raw_request_string = raw_request_stirng;
        return Ok(http_request);
    }

    fn build_http_request(split_request: Vec<&str>) -> InternalServerResult<HttpRequest> {
        let info_line = match split_request.first() {
            Some(s) => s.to_owned(),
            None => {
                return Err(InternalServerError::FatalError(
                    "no request line was parsed".to_string(),
                ))
            }
        };
        // if body is empty this will be a empty string no need to check for the none value
        let request_body = split_request.last().unwrap().to_owned();
        let mut chuck_request_line: Vec<_> = info_line.split(constants::CARRIAGE_RETURN).collect();
        let request_line = chuck_request_line.remove(0);
        let request_headers = chuck_request_line;
        let request_line: Vec<&str> = request_line.split(constants::SINGLE_SPACE).collect();
        let method = request_line[0];
        let uri = request_line[1];
        let version = match request_line[2].split_once("/") {
            Some(s) => s.1.to_owned(),
            None => "1.1".to_owned(),
        };
        Ok(HttpRequest {
            raw_request_string: "".to_string(),
            uri: uri.to_string(),
            headers: request_headers.into_iter().map(|f| f.into()).collect(),
            body: request_body.to_owned(),
            version: version.to_owned(),
            method: HttpMethods::interpret(method)?,
        })
    }

    fn parse_raw_request(mut buff: BufReader<&mut TcpStream>) -> InternalServerResult<String> {
        let received = match buff.fill_buf() {
            Ok(t) => t.to_vec(),
            Err(e) => return Err(InternalServerError::FatalError(format!("`fill_buf()` call failed, failed to fill byte buffer in `parse_raw_request` \n Error: {}", e))),
        };
        buff.consume(received.len());
        match String::from_utf8(received) {
            Ok(s) => Ok(s),
            Err(e) => Err(InternalServerError::ConstructError(
                "`String from Vec<u8>`".to_string(),
                e.to_string(),
            )),
        }
    }
}
