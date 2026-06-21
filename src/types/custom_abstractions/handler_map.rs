use std::collections::HashMap;

use super::{
    super::error_types::hashmap_result::HashMapResult,
    capacity_array::CapacityArray,
    handler_func_type::{HandlerFuncType, HandlerFunctionSignature},
};
use crate::{
    types::error_types::internal_server_error::InternalServerError,
    utils::{constants, https_methods::HttpMethods},
};

/// this is the map that will hold all of the uris that will map to the handler
/// map[route uri] handler func
#[derive(Debug, Clone)]
pub struct HandlerMap {
    map: HashMap<String, CapacityArray<HandlerFuncType>>,
}

#[allow(dead_code)]
impl HandlerMap {
    pub fn new() -> HandlerMap {
        let f: HashMap<String, CapacityArray<HandlerFuncType>> = HashMap::new();
        HandlerMap { map: f }
    }

    /// will get the handler based on the http method and the uri
    pub fn get(&self, uri: &str, method: HttpMethods) -> Option<&HandlerFuncType> {
        let cap_array = match self.map.get(uri) {
            Some(a) => a,
            None => return None,
        };
        for f in cap_array.array.iter() {
            if f.method == method {
                return Some(f);
            } else {
                continue;
            }
        }
        None
    }

    /// will add the mehtod to the capacity array of the given uri, handler, method
    /// will also convert to the underlying type for the array
    pub fn add_handler(
        &mut self,
        uri: &str,
        method: &str,
        function: HandlerFunctionSignature,
    ) -> HashMapResult {
        if method == "NEXT" {
            return Err(InternalServerError::AppenedDataError("capacity array".to_string(), "cannot add `NEXT` to handler array, need to register using `register_middleware` function".to_string()));
        }

        let cap_array = match self.map.get_mut(uri) {
            Some(a) => a,
            None => {
                // this should never be hit
                return Err(InternalServerError::FatalError(
                    "route not registered, use 'add_handler' method to register it".to_string(),
                ));
            }
        };
        if cap_array.len() == constants::HANDLER_LIMIT {
            return Err(InternalServerError::AppenedDataError(
                "handler to capacity array".to_string(),
                "max size of array reached".to_string(),
            ));
        }
        let f = HandlerFuncType::new(method, function)?;
        for s in cap_array.array.iter() {
            if s == &f {
                return Err(InternalServerError::RedundentDataError(
                    format!("{:?}", f),
                    "capacity array".to_string(),
                ));
            }
        }
        cap_array.add_element(f)?;
        Ok(())
    }

    /// attempts to assign the uri to a capacity array to hold its handlers
    /// limit will be the size of the capacity array,
    /// Error: route already registered
    pub fn attempt_register_route(&mut self, uri: &str, limit: usize) -> HashMapResult {
        if self.map.get(uri).is_none() {
            self.insert_array(uri, limit)?;
        }
        Ok(())
    }

    fn insert_array(&mut self, uri: &str, limit: usize) -> HashMapResult {
        let limit = match limit {
            constants::HANDLER_LIMIT => constants::HANDLER_LIMIT,
            constants::MIDDLEWARE_LIMIT => constants::MIDDLEWARE_LIMIT,
            _ => {
                return Err(InternalServerError::ConstructError(
                    "`CapacityArray`".to_string(),
                    "invalid limit given to the capacity array".to_string(),
                ))
            }
        };

        self.map
            .insert(uri.to_string(), CapacityArray::new(Some(limit)));
        Ok(())
    }

    /// add a new handler type to the given uris capacity array
    /// limit is put here to allow for the size to be overridden by the user
    /// Error: to many handlers or dupicate method handler
    pub fn register_middlerware_handler(
        &mut self,
        uri: &str,
        handler: HandlerFunctionSignature,
        limit: usize,
    ) -> HashMapResult {
        let cap_array = match self.map.get_mut(uri) {
            Some(a) => a,
            // need to remove the panics
            None => panic!("route not registered, use 'add_handler' method to register it"),
        };
        if cap_array.len() == limit {
            return Err(InternalServerError::AppenedDataError(
                "middleware to capacity array".to_string(),
                "max size of array reached".to_string(),
            ));
        }
        let f = HandlerFuncType::new("NEXT", handler)?;
        cap_array.add_element(f)?;
        Ok(())
    }

    pub fn exists(&self, key: &str) -> bool {
        match self.map.get(key) {
            Some(_) => true,
            None => false,
        }
    }

    /// returns a slice of the underlying data so operations array opertions can be performed on it
    pub fn as_slice(&self, key: &str) -> Option<&[HandlerFuncType]> {
        match self.map.get(key) {
            Some(t) => Some(t.expose()),
            None => return None,
        }
    }
}
