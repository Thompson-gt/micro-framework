use crate::utils::http_status_codes::HTTP_STATUS_CODES;
use crate::{
    types::{
        custom_abstractions::{file_type::FileType, valid_responses::ValidResponse},
        error_types::internal_server_error::InternalServerError,
        error_types::internal_server_result::InternalServerResult,
    },
    utils::http_status_codes::StatusCode,
};
use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
/// response struct that will be sent to the client
/// formatted to function as a builder type for easy construction
pub struct HttpResponse {
    pub status_code: String,
    pub version: String,
    /// needs to be empty string if no body
    pub body: ValidResponse<Vec<u8>>,
    pub headers: HashMap<String, String>,
}

#[allow(unused)]
impl HttpResponse {
    pub fn new() -> Self {
        Self {
            status_code: " 404 NOT FOUND".to_string(),
            version: "HTTP/1.1".to_string(),
            body: ValidResponse::String("".to_string()),
            headers: HashMap::new(),
        }
    }

    /// sets body to a given string and will be sent as plain text
    pub fn file_to_body(&mut self, path: String) -> InternalServerResult<&mut Self> {
        if path.is_empty() {
            return Err(InternalServerError::ConstructError(
                "static dir".to_string(),
                "couldn't find the provied directory".to_string(),
            ));
        }
        let contents = match fs::read_to_string(path) {
            Ok(s) => s,
            Err(e) => {
                return Err(InternalServerError::ParseError(
                    "error when reading file contents".to_string(),
                ))
            }
        };
        if contents.is_empty() {
        } else {
            self.headers.insert(
                "Content-Type".to_string(),
                "text/html; charset=utf-8".to_string(),
            );
            self.headers
                .insert("Content-Length".to_string(), format!("{}", contents.len()));
            self.body = ValidResponse::String(contents);
        }

        Ok(self)
    }
    /// sets body to a given json string passed, needs to be in json format
    pub fn json_to_body(&mut self, b: String) -> InternalServerResult<&mut Self> {
        if b.is_empty() || !b.starts_with("{") || !b.ends_with("}") {
            return Err(InternalServerError::ParseError(
                "invalid json was given".to_string(),
            ));
        } else {
            self.headers
                .insert("Content-Type".to_string(), "application/json".to_string());
            self.headers
                .insert("Content-Length".to_string(), format!("{}", b.len()));
            self.body = ValidResponse::String(b);
            Ok(self)
        }
    }

    /// sets body to a given string and will be sent as plain text
    pub fn text_to_body(&mut self, b: String) -> InternalServerResult<&mut Self> {
        if b.is_empty() {
            return Err(InternalServerError::EmptyInputError(
                "cannot pass a empty sting as the body".to_string(),
            ));
        } else {
            self.headers.insert(
                "Content-Type".to_string(),
                "text/html; charset=utf-8".to_string(),
            );
            self.headers
                .insert("Content-Length".to_string(), format!("{}", b.len()));
            self.body = ValidResponse::String(b);
            Ok(self)
        }
    }
    /// sets the body to the given string, will not handle the content type header
    pub fn set_body_raw(&mut self, b: Vec<u8>, kind: FileType) -> InternalServerResult<&mut Self> {
        if let Some(check) = self.headers.get("Content-Type") {
            self.add_header("Content-Length".to_string(), format!("{}", b.len()));
            match kind {
                FileType::HTML | FileType::TEXT | FileType::RS => {
                    self.body = ValidResponse::String(
                        String::from_utf8(b).expect("failed to convert the bytes to a string"),
                    )
                }
                _ => self.body = ValidResponse::Vec(b),
            };
        } else {
            return Err(InternalServerError::ConstructError("Response Body".to_string(),"cannot use `set_raw_body` without setting the content type for the reposne\neither set content type with `add_header` method or use one of the `to_body` methods".to_string()));
        };

        Ok(self)
    }

    pub fn add_header(&mut self, key: String, value: String) -> InternalServerResult<&mut Self> {
        if key.is_empty() || value.is_empty() {
            return Err(InternalServerError::EmptyInputError(
                "can not add empty strings to headers".to_string(),
            ));
        } else {
            let _ = self.headers.insert(key, value);
            Ok(self)
        }
    }

    pub fn set_status_code(&mut self, status: usize) -> InternalServerResult<&mut Self> {
        let line: StatusCode = match HTTP_STATUS_CODES.get(status) {
            Ok(l) => l,
            Err(e) => {
                return Err(InternalServerError::ParseError(format!(
                    "{}: status code is not supported",
                    status
                )))
            }
        };
        let str_line = format!(" {} {}", line.0, line.1);
        self.status_code = str_line;
        Ok(self)
    }
}
