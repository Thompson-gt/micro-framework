use thiserror::Error;

#[derive(Error, Debug, Clone)]
#[allow(dead_code)]
pub enum InternalServerError {
    #[error("Server Failure : error `{0}`")]
    FatalError(String),
    #[error("Failed to parse `{0}`")]
    ParseError(String),
    #[error("empty data given to `{0}`")]
    EmptyInputError(String),
    #[error("Failed to encode `{0}`")]
    EncodeError(String),
    #[error("`{0}` is already registered in `{1}`")]
    RedundentDataError(String, String),
    #[error("Failed to append to `{0}`, reason: {1}")]
    AppenedDataError(String, String),
    #[error("Failed to retrieve `{0}` fail error: `{1}`")]
    RetrieveError(String, String),
    #[error("Failed to Construct type `{0}` fail reason: `{1}`")]
    ConstructError(String, String),
    #[error("Server Info {0}")]
    ServerInfo(String),
    #[error("Error when calling handler,  uri: {0}, method:{1} Error: {2}")]
    HandlerError(String, String, String),
    #[error("Function `{0}` changed the state of the type `{1}`\n")]
    StateChange(String, String),
}

impl PartialEq for InternalServerError {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

#[allow(dead_code)]
impl InternalServerError {
    /// returns the name of the enum variant that self is, (used for testing)
    pub fn identify(&self) -> String {
        match self {
            InternalServerError::FatalError(_) => "FatalError".to_string(),
            InternalServerError::ParseError(_) => "ParseError".to_string(),
            InternalServerError::EmptyInputError(_) => "EmptyInputError".to_string(),
            InternalServerError::EncodeError(_) => "EncodeError".to_string(),
            InternalServerError::RedundentDataError(_, _) => "RedundentDataError".to_string(),
            InternalServerError::AppenedDataError(_, _) => "AppenedDataError".to_string(),
            InternalServerError::RetrieveError(_, _) => "RetrieveError".to_string(),
            InternalServerError::ConstructError(_, _) => "ConstructError".to_string(),
            InternalServerError::ServerInfo(_) => "ServerInfo".to_string(),
            InternalServerError::HandlerError(_, _, _) => "HandlerError".to_string(),
            InternalServerError::StateChange(_, _) => "StateChange".to_string(),
        }
    }
}
