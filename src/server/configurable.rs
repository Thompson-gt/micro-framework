use crate::types::custom_abstractions::handler_func_type::HandlerFunctionSignature;
use crate::utils::constants;
/// holds all the user configuable data for the `Server` type
#[allow(dead_code)]
pub struct Configurable {
    pub(super) port: String,
    pub(super) host: String,
    pub(super) use_default_handler: bool,
    pub(super) debug_mode: bool,
    pub(super) use_default_404: bool,
    pub(super) default_handler: Option<HandlerFunctionSignature>,
    pub(super) middleware_limit: usize,
    pub(super) debug_file_path: &'static str,
}

#[allow(dead_code)]
impl Configurable {
    pub fn new() -> Self {
        Self {
            port: constants::DEFAULT_PORT.to_string(),
            host: constants::DEFAULT_HOST.to_string(),
            use_default_handler: false,
            use_default_404: false,
            debug_mode: false,
            default_handler: None,
            middleware_limit: constants::MIDDLEWARE_LIMIT,
            debug_file_path: constants::DEBUG_FILE_PATH,
        }
    }

    // SETTERS

    ///used to configure the domain/host the http server will use
    pub fn set_host(&mut self, host: &str) -> &mut Self {
        if host.len() <= 1 || host == "localhost" {
            self.host = "127.0.0.1".to_string()
        } else {
            self.host = host.to_string()
        }
        self
    }
    ///used to configure the port the http server will use
    pub fn set_port(&mut self, port: usize) -> &mut Self {
        if port <= 1023 || port >= 49152 {
            self.port = 7878.to_string()
        } else {
            self.port = port.to_string()
        }
        self
    }

    /// send a default message to client if the handler was not found in the handler map
    pub fn use_default_404_handler(&mut self) -> &mut Self {
        self.use_default_404 = true;
        self
    }

    ///debug mode is when server will write to a debug file displaying each change in state
    pub fn enable_debug_mode(&mut self) -> &mut Self {
        self.debug_mode = true;
        self
    }

    /// default handler will parse the request and print it to the console
    pub fn set_default_handler(&mut self, handler: HandlerFunctionSignature) -> &mut Self {
        self.default_handler = Some(handler);
        self.use_default_handler = true;
        self
    }

    /// override the default limit of "5" for allowed middlerwares for a given route
    pub fn set_middleware_limit(&mut self, limit: usize) -> &mut Self {
        self.middleware_limit = limit;
        return self;
    }

    /// override the default location of the debug file
    pub fn set_deubug_file_path(&mut self, path: &'static str) -> &mut Self {
        self.debug_file_path = path;
        return self;
    }

    // GETTERS

    /// display the port set
    pub fn expose_port(&self) -> &str {
        return &self.port;
    }
    /// display the host set
    pub fn expose_host(&self) -> &str {
        return &self.host;
    }
}
