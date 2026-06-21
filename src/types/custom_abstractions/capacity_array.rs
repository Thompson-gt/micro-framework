use crate::{types::error_types::internal_server_error::InternalServerError, utils::constants};
use std::cmp::PartialEq;

use super::super::error_types::capacity_result::CapacityResult;

#[derive(Debug, Clone)]
pub struct CapacityArray<T: PartialEq + Copy + Clone> {
    pub array: Vec<T>,
    fallback_limit: usize,
}

#[allow(unused)]
impl<T: PartialEq + Copy + Clone> CapacityArray<T> {
    pub fn new(cap: Option<usize>) -> CapacityArray<T> {
        let limit = match constants::MIDDLEWARE_LIMIT > constants::HANDLER_LIMIT {
            true => constants::MIDDLEWARE_LIMIT,
            false => constants::HANDLER_LIMIT,
        };
        let max_cap = match cap {
            Some(c) => c,
            // middleware limit should be the default because it should always be larger than the
            // http handler limit
            None => constants::MIDDLEWARE_LIMIT,
        };
        let v = Vec::with_capacity(max_cap);
        CapacityArray {
            array: v,
            fallback_limit: limit,
        }
    }

    /// method to add whole vec to the capacity array
    pub fn populate(mut self, values: Vec<T>) -> Self {
        for val in values.into_iter() {
            self.add_element(val);
        }
        self
    }

    /// adds element to the capacity array, will check against the fall back limit
    /// so check for apporiate size before calling this fucntion
    pub fn add_element(&mut self, element: T) -> CapacityResult {
        if self.array.len() == self.fallback_limit {
            Err(InternalServerError::AppenedDataError(
                "CapcityArray".to_string(),
                "max capacity has been reached".to_string(),
            ))
        } else {
            self.array.push(element);
            Ok(())
        }
    }

    /// exposes the underlying data in the vec
    pub fn expose(&self) -> &[T] {
        return self.array.as_slice();
    }

    /// will return a result of if the item was in the array, error if element not found
    /// ok will be the element
    pub fn get(&self, element: &T) -> Option<&T> {
        for val in self.array.iter() {
            if val == element {
                return Some(&val);
            } else {
                continue;
            }
        }
        None
    }

    pub fn len(&self) -> usize {
        self.array.len()
    }
}
