use super::internal_server_error::InternalServerError;

pub type InternalServerResult<T> = std::result::Result<T, InternalServerError>;
