use std::{
    env, fs,
    io::{prelude::*, BufReader, Error, ErrorKind},
    net::{Shutdown, TcpListener, TcpStream},
    path::{Path, PathBuf},
    sync::mpsc::{self, Receiver, Sender},
    sync::Arc,
};

// LOCAL IMPORTS
use super::configurable::Configurable;
use crate::types::custom_abstractions::{
    exit_status::ExitReason, file_type::FileType, handler_func_type::HandlerFunctionSignature,
    shareable_data::ShareableData, static_file::StaticFile, valid_responses::ValidResponse,
};
use crate::types::error_types::{
    internal_server_error::InternalServerError, internal_server_result::InternalServerResult,
};
use crate::types::http_types::{http_request_type::HttpRequest, http_response_type::HttpResponse};

use crate::server::{started_server::StartedServer, stopped_server::StoppedServer};
use crate::utils::constants;

/// first state of the server, allows for configuation and adding of
/// middlewares: `add_middleware` and handlers: `add_handlers`
/// NOTE: ' this type will live for the duration of the whole program'
pub struct UnstartedServer {
    configurable_data: Configurable,
    inner_data: ShareableData,
    running_state: bool,
    debug_file: Arc<fs::File>,
    kill_channel: (mpsc::Sender<()>, mpsc::Receiver<()>),
}

#[allow(dead_code)]
impl UnstartedServer {
    pub fn new(config: Configurable) -> &'static mut UnstartedServer {
        // Box::leak will allow this type to be a static lifetime
        // will cause a memory leak if this is dropped during the program
        let f = Arc::new(
            std::fs::File::create(config.debug_file_path).expect("failed to create the debug file"),
        );
        Box::leak(Box::new(UnstartedServer {
            configurable_data: config,
            inner_data: ShareableData::new(),
            running_state: false,
            debug_file: f,
            kill_channel: mpsc::channel(),
        }))
    }

    /// will bind the server to the host and port and start accepting connections
    /// and push the server into the `StartedServer` state and return it
    // MAIN ENTRY INTO THE PROGRAM
    pub fn start(&'static mut self) -> Result<StartedServer, StoppedServer> {
        self.running_state = true;
        if !self.configurable_data.debug_mode {
            if let Err(_) = fs::remove_file(constants::DEBUG_FILE_PATH) {
                return Err(StoppedServer::new(
                    ExitReason::FailedStartup,
                    InternalServerError::FatalError("failed to remove the debug file".to_string()),
                ));
            };
        }
        let lisener = match TcpListener::bind(format!(
            "{}:{}",
            self.configurable_data.host, self.configurable_data.port
        )) {
            Ok(l) => l,
            Err(e) => {
                return Err(StoppedServer::new(
                    ExitReason::FailedStartup,
                    InternalServerError::FatalError(format!(
                        "failed to bind listener error: {}",
                        e.to_string()
                    )),
                ))
            }
        };

        let (sc, rc): (Sender<InternalServerError>, Receiver<InternalServerError>) =
            mpsc::channel();
        let use_404 = self.configurable_data.use_default_404;
        let kill_chan = self.kill_channel.0.clone();
        let default_handler = self.configurable_data.default_handler;
        let debug_mode = self.configurable_data.debug_mode;
        let mut fd = Arc::clone(&self.debug_file);

        if debug_mode {
            // will spawn the thread to read the errors
            let comsumer_thread = std::thread::Builder::new()
                .name("Consumer Thread".to_string())
                .spawn(move || loop {
                    if let Ok(server_message) = rc.try_recv() {
                        fd.write_all(server_message.to_string().as_bytes())
                            .expect("failed to write to the debug_file");
                    }
                });
            if comsumer_thread.is_err() {
                return Err(StoppedServer::new(
                    ExitReason::FailedStartup,
                    InternalServerError::FatalError(
                        format!("failed to spawn the comusmer thread",),
                    ),
                ));
            }
        }

        //this is running for the entire duration of the program
        let server_loop = std::thread::Builder::new()
            .name("Main Server Loop".to_string())
            .spawn(move || {
                'server_loop: loop {
                    match lisener.accept() {
                        Ok((stream, client_addr)) => {
                            match self.kill_channel.1.try_recv() {
                                Ok(_) => break 'server_loop,
                                Err(_) => {
                                    let sender = sc.clone();
                                    let ref data = self.inner_data;
                                    // will spawn the threads to handler each Tcp stream
                                    let handler = match self.configurable_data.use_default_handler {
                                        true => std::thread::Builder::new()
                                            .name("Default Worker Thread".to_string())
                                            .spawn(move || {
                                                Self::default_connection_handler(stream)
                                            }),
                                        false => std::thread::Builder::new()
                                            .name("Worker Thread".to_string())
                                            .spawn(move || {
                                                Self::handle_connection(
                                                    stream,
                                                    data,
                                                    use_404,
                                                    sender,
                                                    default_handler,
                                                )
                                            }),
                                    };
                                    if handler.is_err() || handler.unwrap().join().is_err() {
                                        let message = format!(
                                            "failed to spawn worker for client :{}",
                                            client_addr.to_string()
                                        );
                                        let e = InternalServerError::ConstructError(
                                            "worker thread".to_string(),
                                            message,
                                        );
                                        let _ = sc.send(e);
                                    }
                                }
                            }
                        }
                        Err(_) => {
                            sc.send(InternalServerError::FatalError(
                                "Failed to get connection stream from client".to_string(),
                            ))
                            .expect("failed to send error in error chan");
                            break 'server_loop;
                        }
                    };
                }
            });

        match server_loop {
            Ok(server_handler) => Ok(StartedServer::new(kill_chan, server_handler)),
            Err(_) => Err(StoppedServer::new(
                ExitReason::FailedStartup,
                InternalServerError::FatalError(
                    "failed to construct the main server loop".to_string(),
                ),
            )),
        }
    }

    fn default_connection_handler(mut stream: TcpStream) {
        let mut buff = BufReader::new(&mut stream);
        let received = buff.fill_buf().unwrap().to_vec();
        buff.consume(received.len());
        let _res = String::from_utf8(received)
            .map(|msg| println!("{}", msg))
            .map_err(|_| {
                Error::new(
                    ErrorKind::InvalidData,
                    "Couldn't parse received string as utf8",
                )
            });
    }

    //THIS IS THE START OF THE HTTP IMPLEMENTION

    ///MAIN ENTRY INTO CUSTOM HANDLER
    fn handle_connection(
        mut stream: TcpStream,
        data: &ShareableData,
        use_404: bool,
        channel: Sender<InternalServerError>,
        default_handler: Option<HandlerFunctionSignature>,
    ) {
        let mut request = HttpRequest::new(&mut stream).unwrap();
        let mut response = HttpResponse::new();
        let handler_fn = data
            .handler_registry
            .get(request.uri.as_str(), request.method.clone());

        if let Some(marray) = data.middlerware_registry.as_slice(request.uri.as_str()) {
            for v in marray.iter() {
                if let Err(base_error) = (v.handler)(&mut request, &mut response) {
                    let handler_error = InternalServerError::HandlerError(
                        request.uri.clone(),
                        request.method.to_stirng(),
                        base_error.to_string(),
                    );
                    channel
                        .send(handler_error)
                        .expect("failed to send `InternalServerError` into channel");
                    stream
                        .shutdown(Shutdown::Both)
                        .expect("failed to close socket connection to client");
                }
            }
        }
        if let Some(f) = handler_fn {
            if let Err(base_error) = (f.handler)(&mut request, &mut response) {
                let handler_error = InternalServerError::HandlerError(
                    request.uri.clone(),
                    request.method.to_stirng(),
                    base_error.to_string(),
                );
                channel
                    .send(handler_error)
                    .expect("failed to send `InternalServerError` into channel");
                stream
                    .shutdown(Shutdown::Both)
                    .expect("failed to close socket connection to client");
            }
        } else {
            let error =
                Self::handle_no_uri(&mut request, &mut response, data, use_404, default_handler);
            if error.is_err() {
                channel
                    .send(error.err().unwrap())
                    .expect("failed to send error through the channel");
                if let Err(socket_error) = stream.shutdown(Shutdown::Both) {
                    channel
                        .send(InternalServerError::FatalError(format!(
                            "failed to close the socket connection: {}",
                            socket_error.to_string()
                        )))
                        .expect("failed to send data into error channel");
                };
                // will end the thread
                return;
            }
        };

        let final_response = match Self::build_final_response(response) {
            Ok(r) => r,
            Err(e) => panic!("error when encoding the response struct:{}", e),
        };

        channel
            .send(Self::write_to_client(final_response, stream))
            .expect("channel fail, couldn't write result from the `write_to_client`");
    }

    /// write bytes to the client, will always return a `InternalServerError`
    /// Success: `InternalServerError::ServerInfo`
    /// Fail: `InternalServerError::FatalError`
    fn write_to_client(response: Vec<u8>, ref mut stream: TcpStream) -> InternalServerError {
        match stream.write_all(response.leak()) {
            Ok(_) => InternalServerError::ServerInfo(
                "reponse was successfully sent to the client".to_string(),
            ),
            Err(e) => {
                // this will panic of the thread
                if stream.shutdown(Shutdown::Both).is_err() {
                    return InternalServerError::FatalError(
                        "failed to close socket connection when `write_to_client` failed"
                            .to_string(),
                    );
                } else {
                    return InternalServerError::FatalError(format!(
                        "`write_to_client` failed to send repsonse {:?}",
                        e.to_string()
                    ));
                }
            }
        }
    }

    /// will appropriately  modify the response if no handler is defined to the current uri
    /// THREE OPTIONS
    /// 1: send static hosted file
    /// 2: no static hosted files
    /// 3: default response (if set to true by user)
    fn handle_no_uri(
        request: &mut HttpRequest,
        response: &mut HttpResponse,
        data: &ShareableData,
        use_404: bool,
        default_handler: Option<HandlerFunctionSignature>,
    ) -> InternalServerResult<()> {
        let uri: String = request.uri[1..request.uri.len()].to_owned();
        for sf in data.static_files.iter() {
            if sf.name == uri {
                let content_type = match sf.kind {
                    FileType::HTML | FileType::TEXT => constants::CONTENT_TEXT_HTML,
                    FileType::PNG => constants::CONTENT_PNG,
                    FileType::JPG => constants::CONTENT_JPG,
                    FileType::JPEG => constants::CONTENT_JPEG,
                    FileType::RS => constants::CONTENT_RS,
                };
                let file_contents = match fs::read(sf.location.clone()) {
                    Ok(f) => f,
                    Err(e) => {
                        return Err(InternalServerError::RetrieveError(
                            format!("failed to read contents of the static file `{}`\n", sf.name),
                            e.to_string(),
                        ))
                    }
                };

                response
                    .set_status_code(200)?
                    .add_header("Content-Type".to_string(), content_type.to_string())?
                    .set_body_raw(file_contents, sf.kind)?;
                return Ok(());
            } else {
                continue;
            };
        }

        if use_404 && default_handler.is_none() {
            response
                .set_status_code(404)?
                .text_to_body(format!("no handler for `{}` uri", request.uri))?;
            return Ok(());
        } else if use_404 && default_handler.is_some() {
            let _ = default_handler.unwrap()(request, response);
            return Ok(());
        } else {
            return Err(InternalServerError::ServerInfo(format!(
                "no handler or staic file found for `{}` uri",
                request.uri
            )));
        }
    }

    /// will apply the given function do all request with the given uri (no matter the http method)
    /// CHANGES THE STATE
    pub fn add_middleware_handler(
        &mut self,
        uri: &str,
        handler: HandlerFunctionSignature,
    ) -> InternalServerResult<()> {
        self.inner_data
            .middlerware_registry
            .attempt_register_route(uri, constants::MIDDLEWARE_LIMIT)?;
        if let Err(e) = self
            .inner_data
            .middlerware_registry
            .register_middlerware_handler(uri, handler, self.configurable_data.middleware_limit)
        {
            return Err(InternalServerError::AppenedDataError(
                "error when adding middleware to middleware map".to_string(),
                e.to_string(),
            ));
        }
        if self.configurable_data.debug_mode {
            let _ = Arc::clone(&self.debug_file).write_all(
                InternalServerError::StateChange(
                    "add_middleware_handler".to_string(),
                    "middleware_registry".to_string(),
                )
                .to_string()
                .as_bytes(),
            );
        }
        Ok(())
    }

    /// set a handler function for http method and uri
    pub fn add_handler(
        &mut self,
        uri: &str,
        method: &str,
        handler: HandlerFunctionSignature,
    ) -> InternalServerResult<()> {
        self.inner_data
            .handler_registry
            .attempt_register_route(uri, constants::HANDLER_LIMIT)?;

        self.inner_data
            .handler_registry
            .add_handler(uri, method, handler)?;

        if self.configurable_data.debug_mode {
            let _ = Arc::clone(&self.debug_file).write_all(
                InternalServerError::StateChange(
                    "add_handler".to_string(),
                    "handler_registry".to_string(),
                )
                .to_string()
                .as_bytes(),
            );
        }
        Ok(())
    }

    /// will give the complete path of the static files
    fn build_staic_dir_path(&self, path: &str) -> InternalServerResult<PathBuf> {
        let mut curr = match env::current_dir() {
            Ok(c) => c,
            Err(_) => {
                return Err(InternalServerError::RetrieveError(
                    "relative path".to_string(),
                    "invalid permissions to view given directory".to_string(),
                ))
            }
        };
        //used to make sure the path is added as a relative path not absolute
        let mut path: Vec<_> = path.split("/").collect();
        let first_element = path
            .first()
            .expect("there should always be a first element!")
            .to_owned();
        // edge case
        if first_element == constants::EMPTY_STRING {
            path.remove(0);
        }
        for element in path.into_iter() {
            if element == ".." {
                if !curr.pop() {
                    return Err(InternalServerError::ParseError(
                        "`StaticDir`, couldn't visit parent dir".to_string(),
                    ));
                }
            } else {
                curr.push(element)
            }
        }
        Ok(curr)
    }
    fn build_file_type(&self, f: PathBuf) -> Option<StaticFile> {
        match f.is_file() {
            true => Some(StaticFile::new(f)),
            false => None,
        }
    }

    /// construct the file vec recursivly on all child dirs of static file dir
    /// CHANGES THE STATE
    fn build_files_vec(&mut self, path: &Path) -> InternalServerResult<()> {
        if path.is_dir() {
            if let Ok(dir) = std::fs::read_dir(path) {
                for f in dir {
                    let f = match f {
                        Ok(f) => f.path(),
                        Err(e) => {
                            return Err(InternalServerError::RetrieveError(
                                path.to_str().unwrap().to_string(),
                                e.to_string(),
                            ))
                        }
                    };
                    if f.is_file() {
                        let file_type = match self.build_file_type(f) {
                            Some(ft) => ft,
                            None => {
                                return Err(InternalServerError::ConstructError(
                                    "StaticFile ".to_string(),
                                    "could not resolve file from `PathBuf`".to_string(),
                                ))
                            }
                        };
                        self.inner_data.static_files.push(file_type)
                    } else {
                        self.build_files_vec(f.as_path())?;
                    }
                }
            } else {
                return Err(InternalServerError::RetrieveError(
                    path.to_str().unwrap().to_string(),
                    "file could not be found".to_string(),
                ));
            }
        } else {
            return Err(InternalServerError::ConstructError(
                "staic_files_vec, inside `build_files_vec`".to_string(),
                " given path is not a dir, either file was given or directory not found "
                    .to_string(),
            ));
        }
        if self.configurable_data.debug_mode {
            let _ = Arc::clone(&self.debug_file).write_all(
                InternalServerError::StateChange(
                    "build_files_vec".to_string(),
                    "static_files".to_string(),
                )
                .to_string()
                .as_bytes(),
            );
        }
        Ok(())
    }

    /// will static host all of the files in the given directory
    /// relative will let the server know where to find the dir
    pub fn static_host(&mut self, dir: &str, relative: bool) -> InternalServerResult<()> {
        if dir.is_empty() {
            return Err(InternalServerError::EmptyInputError(
                "dir in `staic_host` function".to_string(),
            ));
        }
        let static_dir = match relative {
            true => self.build_staic_dir_path(dir)?,
            false => PathBuf::from(dir),
        };

        self.build_files_vec(static_dir.as_path())?;
        Ok(())
    }

    /// turns the response struct into the string format to send be sent to the client
    fn build_final_response(response: HttpResponse) -> InternalServerResult<Vec<u8>> {
        let status_line = format!("{}{}", response.version, response.status_code);
        let mut headers = String::from(constants::CARRIAGE_RETURN);
        for (key, val) in response.headers.iter() {
            headers = format!("{}{}:{}{}", headers, key, val, constants::CARRIAGE_RETURN);
        }
        match response.body {
            ValidResponse::String(s) => {
                let final_response = format!(
                    "{}{}{}{}{}",
                    status_line,
                    headers,
                    constants::CARRIAGE_RETURN,
                    s,
                    constants::DOUBLE_CARRIAGE_RETURN
                );
                Ok(final_response.as_bytes().to_vec())
            }
            ValidResponse::Vec(v) => {
                //have to concat after being encoded or will corrupt the image
                let final_response = [
                    format!("{}{}{}", status_line, headers, constants::CARRIAGE_RETURN).as_bytes(),
                    v.leak(),
                    constants::DOUBLE_CARRIAGE_RETURN.as_bytes(),
                ]
                .concat();

                Ok(final_response)
            }
        }
    }

}

