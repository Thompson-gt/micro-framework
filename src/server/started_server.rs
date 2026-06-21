use super::stopped_server::StoppedServer;
use crate::types::custom_abstractions::exit_status::ExitReason;
use crate::types::error_types::internal_server_error::InternalServerError;
use std::sync::mpsc;

/// state for after `start()` is called on the `Server` type
#[allow(dead_code)]
#[derive(Debug)]
pub struct StartedServer {
    kill_channel: mpsc::Sender<()>,
    // handler for the parent thread of the server loop
    // all other threads are spawed from this thread
    parent_thread: std::thread::JoinHandle<()>,
}

#[allow(dead_code)]
impl StartedServer {
    pub fn new(kill_channel: mpsc::Sender<()>, handler: std::thread::JoinHandle<()>) -> Self {
        Self {
            kill_channel,
            parent_thread: handler,
        }
    }

    /// block main thread until server stops accepting connections
    /// will comsume the struct
    pub fn wait(self) {
        self.parent_thread
            .join()
            .expect("failed to join `Main Server Loop` to main thread?")
    }

    /// will stop the server from accepting incoming connections
    /// (cannot call after wait is called unless in a new thread)
    pub fn kill_server(self) -> StoppedServer {
        match self.kill_channel.send(()) {
            Ok(_) => StoppedServer::new(
                ExitReason::UserEnd,
                InternalServerError::ServerInfo(
                    "Server ended successfully by the user".to_string(),
                ),
            ),
            Err(_) => StoppedServer::new(
                ExitReason::Unrecoverable,
                InternalServerError::FatalError("failed to kill the server cleanly".to_string()),
            ),
        }
    }
}
