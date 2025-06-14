use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::action::ActionDef;

/// Retrieve a value at the given path.
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Parser)]
pub struct GetAction {
    /// The JSON Pointer (RFC6901) within the file to the value being retrieved.
    ///
    /// A bare slash is not a valid reference to the root; instead, specify an
    /// empty string ("") to get the entire document
    key: String,
}

impl ActionDef for GetAction {
    fn apply(&mut self, value: Value) -> Option<Value> {
        match value.pointer(&self.key) {
            Some(v) => Some(v.clone()),
            None => None,
        }
    }
}