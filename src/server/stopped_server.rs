use crate::types::custom_abstractions::exit_status::ExitReason;
use crate::types::error_types::internal_server_error::InternalServerError;

/// the final state of the server, will hold some info about the server that just ran
/// can be returned from `UnstartedServer` or `StartedServer`
#[derive(Debug)]
#[allow(dead_code)]
pub struct StoppedServer {
    pub exit_status: ExitReason,
    pub final_error: InternalServerError,
}

impl StoppedServer {
    pub fn new(exit_status: ExitReason, final_error: InternalServerError) -> Self {
        Self {
            exit_status,
            final_error,
        }
    }
}
