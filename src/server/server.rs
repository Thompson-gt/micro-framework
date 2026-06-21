use crate::server::configurable::Configurable;
use crate::server::unstarted_server::UnstartedServer;
use crate::types::error_types::internal_server_error::InternalServerError;

// THIS IS JUST THE MAIN ENTRY INTO THE SERVER

/// Return type for all Handler Functions
pub type HandlerResult = Result<(), InternalServerError>;
/// the base type that will return the first state of the server
pub struct FrameWork {}

#[allow(dead_code)]
impl FrameWork {
    /// returns a configurable type to be given to the server
    pub fn new_config() -> Configurable {
        // might need to make a way for the user to opt/out of the handler and the middleware
        // limits, as well as the checks for the same methods added to the array
        Configurable::new()
    }
    /// creates new `UnstartedServer` that will be used to handle connections
    /// can pass default `Configurable` with `new_config()`
    pub fn new(config: Configurable) -> &'static mut UnstartedServer {
        UnstartedServer::new(config)
    }
}
