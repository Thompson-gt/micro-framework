use super::handler_map::HandlerMap;
use super::static_file::StaticFile;

/// inner struct that will carry the data that needs to be shared among threads
#[derive(Clone)]
pub struct ShareableData {
    pub handler_registry: HandlerMap,
    pub middlerware_registry: HandlerMap,
    pub static_files: Vec<StaticFile>,
}

impl ShareableData {
    pub fn new() -> Self {
        Self {
            handler_registry: HandlerMap::new(),
            middlerware_registry: HandlerMap::new(),
            static_files: Vec::new(),
        }
    }
}
