use crate::server::server::HandlerResult;
use crate::types::error_types::internal_server_result::InternalServerResult;
use crate::types::http_types::http_request_type::HttpRequest;
use crate::types::http_types::http_response_type::HttpResponse;
use crate::utils::https_methods::HttpMethods;

pub type HandlerFunctionSignature = fn(&mut HttpRequest, &mut HttpResponse) -> HandlerResult;

/// internal type to reprsent the http handler
/// type that is stored in the handler registry cap array
#[derive(Debug, Clone, Copy)]
pub struct HandlerFuncType {
    pub method: HttpMethods,
    pub handler: HandlerFunctionSignature,
}

impl HandlerFuncType {
    pub fn new(method: &str, handler: HandlerFunctionSignature) -> InternalServerResult<Self> {
        Ok(Self {
            method: HttpMethods::interpret(method)?,
            handler,
        })
    }
}
impl PartialEq for HandlerFuncType {
    fn eq(&self, other: &Self) -> bool {
        self.method == other.method
    }
}
